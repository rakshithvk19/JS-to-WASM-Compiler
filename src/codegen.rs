use crate::ast::*;
use std::collections::{HashMap, HashSet};

pub struct CodeGen {
    output: Vec<String>,
    functions: HashMap<String, usize>, // name -> param count
    label_counter: usize,
    consts: HashSet<String>,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            output: Vec::new(),
            functions: HashMap::new(),
            label_counter: 0,
            consts: HashSet::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // Register all functions first
        for func in &program.functions {
            self.functions.insert(func.name.clone(), func.params.len());
        }

        self.output.push("(module".to_string());

        // Generate all functions
        for func in &program.functions {
            self.gen_function(func);
        }

        // Generate _start function for top-level code
        self.gen_start(&program.top_level);

        self.output.push(")".to_string());
        self.output.join("\n")
    }

    fn gen_function(&mut self, func: &Function) {
        self.consts.clear();
        let locals = self.collect_locals(&func.body, &func.params);

        let params: Vec<String> = func
            .params
            .iter()
            .map(|p| format!("(param ${} i32)", p))
            .collect();

        let local_decls: Vec<String> = locals
            .iter()
            .map(|l| format!("(local ${} i32)", l))
            .collect();

        self.output.push(format!(
            "  (func ${} (export \"{}\") {} (result i32) ;; line {}",
            func.name,
            func.name,
            params.join(" "),
            func.line
        ));

        for decl in local_decls {
            self.output.push(format!("    {}", decl));
        }

        // Add $_result for logical operators
        self.output.push("    (local $_result i32)".to_string());

        let all_vars: Vec<String> = func.params.iter().chain(locals.iter()).cloned().collect();

        for stmt in &func.body {
            self.gen_stmt(stmt, &all_vars);
        }

        self.output.push("    i32.const 0".to_string());
        self.output.push("  )".to_string());
    }

    fn gen_start(&mut self, stmts: &[Stmt]) {
        self.consts.clear();
        let locals = self.collect_locals(stmts, &[]);

        let local_decls: Vec<String> = locals
            .iter()
            .map(|l| format!("(local ${} i32)", l))
            .collect();

        self.output
            .push("  (func $_start (export \"_start\") (result i32)".to_string());

        for decl in local_decls {
            self.output.push(format!("    {}", decl));
        }

        // Track the last expression value
        self.output.push("    (local $_result i32)".to_string());

        for stmt in stmts {
            self.gen_stmt_with_result(stmt, &locals);
        }

        self.output.push("    local.get $_result".to_string());
        self.output.push("  )".to_string());
    }

    fn collect_locals(&self, stmts: &[Stmt], exclude: &[String]) -> Vec<String> {
        let mut locals = Vec::new();
        self.collect_locals_rec(stmts, &mut locals);
        locals
            .into_iter()
            .filter(|l| !exclude.contains(l))
            .collect()
    }

    fn collect_locals_rec(&self, stmts: &[Stmt], locals: &mut Vec<String>) {
        for stmt in stmts {
            match &stmt.kind {
                StmtKind::Let(name, _) | StmtKind::Const(name, _) => {
                    if !locals.contains(name) {
                        locals.push(name.clone());
                    }
                }
                StmtKind::Block(inner) => self.collect_locals_rec(inner, locals),
                StmtKind::If(_, then_branch, else_branch) => {
                    self.collect_locals_rec(&[*then_branch.clone()], locals);
                    if let Some(eb) = else_branch {
                        self.collect_locals_rec(&[*eb.clone()], locals);
                    }
                }
                StmtKind::While(_, body) => {
                    self.collect_locals_rec(&[*body.clone()], locals);
                }
                _ => {}
            }
        }
    }

    fn emit_line_comment(&mut self, line: usize) {
        self.output.push(format!("    ;; line {}", line));
    }

    fn gen_stmt(&mut self, stmt: &Stmt, vars: &[String]) {
        self.emit_line_comment(stmt.line);
        match &stmt.kind {
            StmtKind::Let(name, expr) => {
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::Const(name, expr) => {
                self.consts.insert(name.clone());
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::Assign(name, expr) => {
                if self.consts.contains(name) {
                    panic!("Cannot reassign const variable '{}'", name);
                }
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::If(cond, then_branch, else_branch) => {
                self.gen_expr(cond, vars);
                if else_branch.is_some() {
                    self.output.push("    if".to_string());
                    self.gen_stmt(then_branch, vars);
                    self.output.push("    else".to_string());
                    self.gen_stmt(else_branch.as_ref().unwrap(), vars);
                    self.output.push("    end".to_string());
                } else {
                    self.output.push("    if".to_string());
                    self.gen_stmt(then_branch, vars);
                    self.output.push("    end".to_string());
                }
            }
            StmtKind::While(cond, body) => {
                let id = self.label_counter;
                self.label_counter += 1;
                self.output.push(format!("    block $break_{}", id));
                self.output.push(format!("    loop $continue_{}", id));
                self.gen_expr(cond, vars);
                self.output.push("    i32.eqz".to_string());
                self.output.push(format!("    br_if $break_{}", id));
                self.gen_stmt(body, vars);
                self.output.push(format!("    br $continue_{}", id));
                self.output.push("    end".to_string());
                self.output.push("    end".to_string());
            }
            StmtKind::Block(stmts) => {
                for s in stmts {
                    self.gen_stmt(s, vars);
                }
            }
            StmtKind::Return(expr) => {
                if let Expr::Call(name, args) = expr {
                    // Tail call - use return_call
                    for arg in args {
                        self.gen_expr(arg, vars);
                    }
                    self.output.push(format!("    return_call ${}", name));
                } else {
                    // Normal return
                    self.gen_expr(expr, vars);
                    self.output.push("    return".to_string());
                }
            }
            StmtKind::Expr(expr) => {
                self.gen_expr(expr, vars);
                self.output.push("    drop".to_string());
            }
        }
    }

    fn gen_stmt_with_result(&mut self, stmt: &Stmt, vars: &[String]) {
        self.emit_line_comment(stmt.line);
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.gen_expr(expr, vars);
                self.output.push("    local.set $_result".to_string());
            }
            _ => self.gen_stmt(stmt, vars),
        }
    }

    fn gen_expr(&mut self, expr: &Expr, vars: &[String]) {
        match expr {
            Expr::Number(n) => {
                self.output.push(format!("    i32.const {}", n));
            }
            Expr::Identifier(name) => {
                self.output.push(format!("    local.get ${}", name));
            }
            Expr::Binary(left, op, right) => {
                self.gen_expr(left, vars);
                self.gen_expr(right, vars);
                let instr = match op {
                    BinOp::Add => "i32.add",
                    BinOp::Sub => "i32.sub",
                    BinOp::Mul => "i32.mul",
                    BinOp::Div => "i32.div_s",
                    BinOp::Mod => "i32.rem_s",
                    BinOp::Eq => "i32.eq",
                    BinOp::Ne => "i32.ne",
                    BinOp::Lt => "i32.lt_s",
                    BinOp::Gt => "i32.gt_s",
                    BinOp::Le => "i32.le_s",
                    BinOp::Ge => "i32.ge_s",
                };
                self.output.push(format!("    {}", instr));
            }
            Expr::Unary(op, operand) => match op {
                UnaryOp::Neg => {
                    self.output.push("    i32.const 0".to_string());
                    self.gen_expr(operand, vars);
                    self.output.push("    i32.sub".to_string());
                }
                UnaryOp::Not => {
                    self.gen_expr(operand, vars);
                    self.output.push("    i32.eqz".to_string());
                }
            },
            Expr::Call(name, args) => {
                for arg in args {
                    self.gen_expr(arg, vars);
                }
                self.output.push(format!("    call ${}", name));
            }
            Expr::Logical(left, op, right) => match op {
                LogicalOp::And => {
                    self.gen_expr(left, vars);
                    self.output.push("    local.tee $_result".to_string());
                    self.output.push("    i32.eqz".to_string());
                    self.output.push("    if (result i32)".to_string());
                    self.output.push("    local.get $_result".to_string());
                    self.output.push("    else".to_string());
                    self.gen_expr(right, vars);
                    self.output.push("    end".to_string());
                }
                LogicalOp::Or => {
                    self.gen_expr(left, vars);
                    self.output.push("    local.tee $_result".to_string());
                    self.output.push("    if (result i32)".to_string());
                    self.output.push("    local.get $_result".to_string());
                    self.output.push("    else".to_string());
                    self.gen_expr(right, vars);
                    self.output.push("    end".to_string());
                }
            },
        }
    }
}
