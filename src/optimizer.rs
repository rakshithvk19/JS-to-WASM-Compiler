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

        // Stop processing after return (dead code elimination)
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

            // Dead code: if (0) -> remove, if (non-zero) -> keep then only
            if let Expr::Number(n) = &cond {
                if *n == 0 {
                    // if (false) - use else branch or empty block
                    return match else_branch {
                        Some(eb) => optimize_stmt(*eb),
                        None => Stmt {
                            kind: StmtKind::Block(vec![]),
                            line: stmt.line,
                        },
                    };
                } else {
                    // if (true) - use then branch
                    return optimize_stmt(*then_branch);
                }
            }

            let then_branch = Box::new(optimize_stmt(*then_branch));
            let else_branch = else_branch.map(|eb| Box::new(optimize_stmt(*eb)));
            StmtKind::If(cond, then_branch, else_branch)
        }
        StmtKind::While(cond, body) => {
            let cond = fold_expr(cond);

            // Dead code: while (0) -> remove entirely
            if let Expr::Number(0) = &cond {
                return Stmt {
                    kind: StmtKind::Block(vec![]),
                    line: stmt.line,
                };
            }

            let body = Box::new(optimize_stmt(*body));
            StmtKind::While(cond, body)
        }
        StmtKind::Block(stmts) => StmtKind::Block(optimize_stmts(stmts)),
        StmtKind::Return(expr) => StmtKind::Return(fold_expr(expr)),
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

            Expr::Binary(Box::new(left), op, Box::new(right))
        }
        Expr::Unary(op, operand) => {
            let operand = fold_expr(*operand);

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
        _ => expr,
    }
}
