use crate::ast::*;

pub fn optimize_program(program: &mut Program) {
    for func in &mut program.functions {
        func.body = optimize_stmts(func.body.clone());
    }
    program.top_level = optimize_stmts(program.top_level.clone());
}

fn optimize_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut result = Vec::new();

    for stmt in stmts {
        let optimized = optimize_stmt(stmt);

        // Check if this statement is a return
        let is_return = matches!(optimized.kind, StmtKind::Return(_));

        result.push(optimized);

        // Stop processing after return 
        if is_return {
            break;
        }
    }

    result
}

fn optimize_stmt(stmt: Stmt) -> Stmt {
    let kind = match stmt.kind {
        StmtKind::Let(name, expr) => StmtKind::Let(name, fold_expr(expr)),
        StmtKind::Const(name, expr) => StmtKind::Const(name, fold_expr(expr)),
        StmtKind::Assign(name, expr) => StmtKind::Assign(name, fold_expr(expr)),
        StmtKind::If(cond, then_branch, else_branch) => {
            let cond = fold_expr(cond);

            // Dead code: if (0) or if (0.0) -> remove
            let is_false = match &cond {
                Expr::Number(n) => *n == 0,
                Expr::NumberF32(f) => *f == 0.0,
                _ => false,
            };

            if is_false {
                // if (false) - use else branch or empty block
                return match else_branch {
                    Some(eb) => optimize_stmt(*eb),
                    None => Stmt {
                        kind: StmtKind::Block(vec![]),
                        line: stmt.line,
                    },
                };
            }

            // if (non-zero constant) -> keep then only
            let is_true = match &cond {
                Expr::Number(n) => *n != 0,
                Expr::NumberF32(f) => *f != 0.0,
                _ => false,
            };

            if is_true {
                return optimize_stmt(*then_branch);
            }

            let then_branch = Box::new(optimize_stmt(*then_branch));
            let else_branch = else_branch.map(|eb| Box::new(optimize_stmt(*eb)));
            StmtKind::If(cond, then_branch, else_branch)
        }
        StmtKind::While(cond, body) => {
            let cond = fold_expr(cond);

            // Dead code: while (0) or while (0.0) -> remove entirely
            let is_false = match &cond {
                Expr::Number(n) => *n == 0,
                Expr::NumberF32(f) => *f == 0.0,
                _ => false,
            };

            if is_false {
                return Stmt {
                    kind: StmtKind::Block(vec![]),
                    line: stmt.line,
                };
            }

            let body = Box::new(optimize_stmt(*body));
            StmtKind::While(cond, body)
        }
        StmtKind::For(init, cond, incr, body) => {
            let init = init.map(|i| Box::new(optimize_stmt(*i)));
            let cond = cond.map(fold_expr);
            let incr = incr.map(|i| Box::new(optimize_stmt(*i)));

            // Dead code: for with false condition
            if let Some(cond_expr) = &cond {
                let is_false = match cond_expr {
                    Expr::Number(n) => *n == 0,
                    Expr::NumberF32(f) => *f == 0.0,
                    _ => false,
                };

                if is_false {
                    // Condition is false - loop never executes
                    // Just execute init if present, then return empty block
                    return if let Some(init_stmt) = init {
                        Stmt {
                            kind: StmtKind::Block(vec![*init_stmt]),
                            line: stmt.line,
                        }
                    } else {
                        Stmt {
                            kind: StmtKind::Block(vec![]),
                            line: stmt.line,
                        }
                    };
                }
            }

            let body = Box::new(optimize_stmt(*body));
            StmtKind::For(init, cond, incr, body)
        }
        StmtKind::Block(stmts) => StmtKind::Block(optimize_stmts(stmts)),
        StmtKind::Return(expr) => StmtKind::Return(fold_expr(expr)),
        StmtKind::Break => StmtKind::Break,
        StmtKind::Continue => StmtKind::Continue,
        StmtKind::Expr(expr) => StmtKind::Expr(fold_expr(expr)),
    };

    Stmt {
        kind,
        line: stmt.line,
    }
}

fn fold_expr(expr: Expr) -> Expr {
    match expr {
        Expr::Binary(left, op, right) => {
            let left = fold_expr(*left);
            let right = fold_expr(*right);

            // Fold i32 constants
            if let (Expr::Number(a), Expr::Number(b)) = (&left, &right) {
                let result = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    BinOp::Div => a / b,
                    BinOp::Mod => a % b,
                    BinOp::Eq => {
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                    BinOp::Ne => {
                        if a != b {
                            1
                        } else {
                            0
                        }
                    }
                    BinOp::Lt => {
                        if a < b {
                            1
                        } else {
                            0
                        }
                    }
                    BinOp::Gt => {
                        if a > b {
                            1
                        } else {
                            0
                        }
                    }
                    BinOp::Le => {
                        if a <= b {
                            1
                        } else {
                            0
                        }
                    }
                    BinOp::Ge => {
                        if a >= b {
                            1
                        } else {
                            0
                        }
                    }
                };
                return Expr::Number(result);
            }

            // Fold f32 constants
            if let (Expr::NumberF32(a), Expr::NumberF32(b)) = (&left, &right) {
                // For comparisons, return i32 result
                match op {
                    BinOp::Eq => {
                        return Expr::Number(if a == b { 1 } else { 0 });
                    }
                    BinOp::Ne => {
                        return Expr::Number(if a != b { 1 } else { 0 });
                    }
                    BinOp::Lt => {
                        return Expr::Number(if a < b { 1 } else { 0 });
                    }
                    BinOp::Gt => {
                        return Expr::Number(if a > b { 1 } else { 0 });
                    }
                    BinOp::Le => {
                        return Expr::Number(if a <= b { 1 } else { 0 });
                    }
                    BinOp::Ge => {
                        return Expr::Number(if a >= b { 1 } else { 0 });
                    }
                    _ => {}
                }

                // For arithmetic operations, return f32 result
                let result = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    BinOp::Div => a / b,
                    BinOp::Mod => {
                        // Modulo on f32 - shouldn't happen (semantic analyzer blocks it)
                        // But handle it here for safety
                        return Expr::Binary(Box::new(left), op, Box::new(right));
                    }
                    _ => unreachable!(), // Comparisons handled above
                };
                return Expr::NumberF32(result);
            }

            Expr::Binary(Box::new(left), op, Box::new(right))
        }
        Expr::Unary(op, operand) => {
            let operand = fold_expr(*operand);

            // Fold i32 unary
            if let Expr::Number(n) = operand {
                let result = match op {
                    UnaryOp::Neg => -n,
                    UnaryOp::Not => {
                        if n == 0 {
                            1
                        } else {
                            0
                        }
                    }
                };
                return Expr::Number(result);
            }

            // Fold f32 unary
            if let Expr::NumberF32(f) = operand {
                match op {
                    UnaryOp::Neg => return Expr::NumberF32(-f),
                    UnaryOp::Not => {
                        // ! on f32 returns i32 (0 or 1)
                        return Expr::Number(if f == 0.0 { 1 } else { 0 });
                    }
                }
            }

            Expr::Unary(op, Box::new(operand))
        }
        Expr::Call(name, args) => {
            let args = args.into_iter().map(fold_expr).collect();
            Expr::Call(name, args)
        }
        Expr::Logical(left, op, right) => {
            let left = fold_expr(*left);
            let right = fold_expr(*right);
            Expr::Logical(Box::new(left), op, Box::new(right))
        }
        Expr::NumberF32(_) => expr, 
        _ => expr,
    }
}
