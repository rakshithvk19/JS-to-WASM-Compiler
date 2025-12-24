use crate::ast::*;
use crate::error::{CompilerError, Result};
use std::collections::{HashMap, HashSet};

pub struct SemanticAnalyzer {
    variables: Vec<HashMap<String, bool>>, // Stack of scopes, bool = is_const
    functions: HashSet<String>,
    loop_depth: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            variables: vec![HashMap::new()],
            functions: HashSet::new(),
            loop_depth: 0,
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        // Register all functions first
        for func in &program.functions {
            self.functions.insert(func.name.clone());
        }

        // Analyze each function
        for func in &program.functions {
            self.analyze_function(func)?;
        }

        // Analyze top-level code
        self.analyze_stmts(&program.top_level)?;

        Ok(())
    }

    fn analyze_function(&mut self, func: &Function) -> Result<()> {
        self.enter_scope();

        // Add parameters to scope
        for param in &func.params {
            self.variables
                .last_mut()
                .unwrap()
                .insert(param.clone(), false);
        }

        self.analyze_stmts(&func.body)?;

        self.exit_scope();
        Ok(())
    }

    fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.variables.pop();
    }

    fn is_variable_defined(&self, name: &str) -> bool {
        for scope in self.variables.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn is_variable_const(&self, name: &str) -> bool {
        for scope in self.variables.iter().rev() {
            if let Some(&is_const) = scope.get(name) {
                return is_const;
            }
        }
        false
    }

    fn analyze_stmts(&mut self, stmts: &[Stmt]) -> Result<()> {
        for stmt in stmts {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match &stmt.kind {
            StmtKind::Let(name, expr) => {
                self.analyze_expr(expr, stmt.line)?;
                self.variables
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), false);
            }
            StmtKind::Const(name, expr) => {
                self.analyze_expr(expr, stmt.line)?;
                self.variables
                    .last_mut()
                    .unwrap()
                    .insert(name.clone(), true);
            }
            StmtKind::Assign(name, expr) => {
                if !self.is_variable_defined(name) {
                    return Err(CompilerError::semantic(
                        stmt.line,
                        format!("Undefined variable '{}'", name),
                    ));
                }
                if self.is_variable_const(name) {
                    return Err(CompilerError::semantic(
                        stmt.line,
                        format!("Cannot reassign const variable '{}'", name),
                    ));
                }
                self.analyze_expr(expr, stmt.line)?;
            }
            StmtKind::If(cond, then_branch, else_branch) => {
                self.analyze_expr(cond, stmt.line)?;
                self.analyze_stmt(then_branch)?;
                if let Some(eb) = else_branch {
                    self.analyze_stmt(eb)?;
                }
            }
            StmtKind::While(cond, body) => {
                self.analyze_expr(cond, stmt.line)?;
                self.loop_depth += 1;
                self.analyze_stmt(body)?;
                self.loop_depth -= 1;
            }
            StmtKind::For(init, cond, incr, body) => {
                if let Some(init_stmt) = init {
                    self.analyze_stmt(init_stmt)?;
                }
                if let Some(cond_expr) = cond {
                    self.analyze_expr(cond_expr, stmt.line)?;
                }
                self.loop_depth += 1;
                self.analyze_stmt(body)?;
                if let Some(incr_stmt) = incr {
                    self.analyze_stmt(incr_stmt)?;
                }
                self.loop_depth -= 1;
            }
            StmtKind::Block(stmts) => {
                self.enter_scope();
                self.analyze_stmts(stmts)?;
                self.exit_scope();
            }
            StmtKind::Return(expr) => {
                self.analyze_expr(expr, stmt.line)?;
            }
            StmtKind::Break => {
                if self.loop_depth == 0 {
                    return Err(CompilerError::semantic(
                        stmt.line,
                        "Break statement outside of loop".to_string(),
                    ));
                }
            }
            StmtKind::Continue => {
                if self.loop_depth == 0 {
                    return Err(CompilerError::semantic(
                        stmt.line,
                        "Continue statement outside of loop".to_string(),
                    ));
                }
            }
            StmtKind::Expr(expr) => {
                self.analyze_expr(expr, stmt.line)?;
            }
        }
        Ok(())
    }

    fn analyze_expr(&mut self, expr: &Expr, line: usize) -> Result<()> {
        match expr {
            Expr::Number(_) => {}
            Expr::Identifier(name) => {
                if !self.is_variable_defined(name) && !self.functions.contains(name) {
                    return Err(CompilerError::semantic(
                        line,
                        format!("Undefined variable or function '{}'", name),
                    ));
                }
            }
            Expr::Binary(left, _, right) => {
                self.analyze_expr(left, line)?;
                self.analyze_expr(right, line)?;
            }
            Expr::Unary(_, operand) => {
                self.analyze_expr(operand, line)?;
            }
            Expr::Call(name, args) => {
                if !self.functions.contains(name) {
                    return Err(CompilerError::semantic(
                        line,
                        format!("Undefined function '{}'", name),
                    ));
                }
                for arg in args {
                    self.analyze_expr(arg, line)?;
                }
            }
            Expr::Logical(left, _, right) => {
                self.analyze_expr(left, line)?;
                self.analyze_expr(right, line)?;
            }
        }
        Ok(())
    }
}
