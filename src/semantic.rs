use crate::ast::*;
use crate::error::{CompilerError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct VarInfo {
    is_const: bool,
    var_type: Type,
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    param_types: Option<Vec<Type>>, // None until first call
    return_type: Option<Type>,      // None until analyzed
}

pub struct SemanticAnalyzer {
    variables: Vec<HashMap<String, VarInfo>>, // Stack of scopes with type info
    functions: HashMap<String, FunctionInfo>, // Function signatures
    loop_depth: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            variables: vec![HashMap::new()],
            functions: HashMap::new(),
            loop_depth: 0,
        }
    }

    pub fn analyze(&mut self, program: &mut Program) -> Result<()> {
        // Register all functions first (without types yet)
        for func in &program.functions {
            self.functions.insert(
                func.name.clone(),
                FunctionInfo {
                    param_types: None,
                    return_type: None,
                },
            );
        }

        // First pass: Analyze each function with default i32 params
        for func in &program.functions {
            self.analyze_function_with_params(func, &vec![Type::I32; func.params.len()])?;
        }

        // Analyze top-level code (this sets param types on first call)
        self.analyze_stmts(&program.top_level)?;

        // Second pass: Re-analyze functions whose param types were set
        for func in &program.functions {
            let param_types = {
                let func_info = self.functions.get(&func.name).unwrap();
                func_info.param_types.clone() // Clone to drop the immutable borrow
            };

            if let Some(param_types) = param_types {
                // Param types were set, re-analyze with correct types
                self.analyze_function_with_params(func, &param_types)?;
            }
        }

        // Write inferred types back to AST
        for func in &mut program.functions {
            let func_info = self.functions.get(&func.name).unwrap();
            func.param_types = func_info.param_types.clone();
            func.return_type = func_info.return_type;
        }

        Ok(())
    }

    fn analyze_function_with_params(
        &mut self,
        func: &Function,
        param_types: &[Type],
    ) -> Result<()> {
        self.enter_scope();

        // Add parameters to scope with specified types
        for (param, param_type) in func.params.iter().zip(param_types.iter()) {
            self.variables.last_mut().unwrap().insert(
                param.clone(),
                VarInfo {
                    is_const: false,
                    var_type: *param_type,
                },
            );
        }

        // Analyze body statements FIRST so variables are declared
        self.analyze_stmts(&func.body)?;

        // THEN infer return type by searching for Return statements
        let return_type = self.infer_return_type_from_stmts(&func.body)?;

        // Store return type (default to i32 if no return)
        let return_type = return_type.unwrap_or(Type::I32);
        self.functions.get_mut(&func.name).unwrap().return_type = Some(return_type);

        self.exit_scope();
        Ok(())
    }

    //Recursively search for return statements in nested blocks
    fn infer_return_type_from_stmts(&mut self, stmts: &[Stmt]) -> Result<Option<Type>> {
        let mut return_type: Option<Type> = None;

        for stmt in stmts {
            let stmt_return_type = match &stmt.kind {
                StmtKind::Return(expr) => Some(self.infer_expr_type(expr, stmt.line)?),
                StmtKind::If(_, then_branch, else_branch) => {
                    let then_type = self.infer_return_type_from_stmts(&[*then_branch.clone()])?;
                    let else_type = if let Some(eb) = else_branch {
                        self.infer_return_type_from_stmts(&[*eb.clone()])?
                    } else {
                        None
                    };
                    then_type.or(else_type)
                }
                StmtKind::While(_, body) => self.infer_return_type_from_stmts(&[*body.clone()])?,
                StmtKind::For(_, _, _, body) => {
                    self.infer_return_type_from_stmts(&[*body.clone()])?
                }
                StmtKind::Block(inner_stmts) => self.infer_return_type_from_stmts(inner_stmts)?,
                _ => None,
            };

            if let Some(found_type) = stmt_return_type {
                if let Some(existing_type) = return_type {
                    if existing_type != found_type {
                        return Err(CompilerError::semantic(
                            stmt.line,
                            format!(
                                "Inconsistent return types: expected {:?}, got {:?}",
                                existing_type, found_type
                            ),
                        ));
                    }
                } else {
                    return_type = Some(found_type);
                }
            }
        }

        Ok(return_type)
    }

    fn enter_scope(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.variables.pop();
    }

    fn get_variable_info(&self, name: &str) -> Option<&VarInfo> {
        for scope in self.variables.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    fn is_variable_defined(&self, name: &str) -> bool {
        self.get_variable_info(name).is_some()
    }

    fn is_variable_const(&self, name: &str) -> bool {
        self.get_variable_info(name)
            .map(|info| info.is_const)
            .unwrap_or(false)
    }

    fn get_variable_type(&self, name: &str) -> Option<Type> {
        self.get_variable_info(name).map(|info| info.var_type)
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
                let expr_type = self.infer_expr_type(expr, stmt.line)?;
                self.variables.last_mut().unwrap().insert(
                    name.clone(),
                    VarInfo {
                        is_const: false,
                        var_type: expr_type,
                    },
                );
            }
            StmtKind::Const(name, expr) => {
                let expr_type = self.infer_expr_type(expr, stmt.line)?;
                self.variables.last_mut().unwrap().insert(
                    name.clone(),
                    VarInfo {
                        is_const: true,
                        var_type: expr_type,
                    },
                );
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

                // Type checking on assignment
                let var_type = self.get_variable_type(name).unwrap();
                let expr_type = self.infer_expr_type(expr, stmt.line)?;
                if var_type != expr_type {
                    return Err(CompilerError::semantic(
                        stmt.line,
                        format!(
                            "Type mismatch: cannot assign {:?} to {:?} variable '{}'",
                            expr_type, var_type, name
                        ),
                    ));
                }
            }
            StmtKind::If(cond, then_branch, else_branch) => {
                self.infer_expr_type(cond, stmt.line)?;
                self.analyze_stmt(then_branch)?;
                if let Some(eb) = else_branch {
                    self.analyze_stmt(eb)?;
                }
            }
            StmtKind::While(cond, body) => {
                self.infer_expr_type(cond, stmt.line)?;
                self.loop_depth += 1;
                self.analyze_stmt(body)?;
                self.loop_depth -= 1;
            }
            StmtKind::For(init, cond, incr, body) => {
                //For loops need their own scope for the init variable
                self.enter_scope();

                if let Some(init_stmt) = init {
                    self.analyze_stmt(init_stmt)?;
                }
                if let Some(cond_expr) = cond {
                    self.infer_expr_type(cond_expr, stmt.line)?;
                }
                self.loop_depth += 1;
                self.analyze_stmt(body)?;
                if let Some(incr_stmt) = incr {
                    self.analyze_stmt(incr_stmt)?;
                }
                self.loop_depth -= 1;

                self.exit_scope();
            }
            StmtKind::Block(stmts) => {
                self.enter_scope();
                self.analyze_stmts(stmts)?;
                self.exit_scope();
            }
            StmtKind::Return(expr) => {
                self.infer_expr_type(expr, stmt.line)?;
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
                self.infer_expr_type(expr, stmt.line)?;
            }
        }
        Ok(())
    }

    fn infer_expr_type(&mut self, expr: &Expr, line: usize) -> Result<Type> {
        match expr {
            Expr::Number(_) => Ok(Type::I32),
            Expr::NumberF32(_) => Ok(Type::F32),
            Expr::Identifier(name) => {
                if let Some(var_type) = self.get_variable_type(name) {
                    Ok(var_type)
                } else if self.functions.contains_key(name) {
                    // Function reference - for now, error (need first-class functions)
                    Err(CompilerError::semantic(
                        line,
                        format!("Cannot use function '{}' as a value", name),
                    ))
                } else {
                    Err(CompilerError::semantic(
                        line,
                        format!("Undefined variable '{}'", name),
                    ))
                }
            }
            Expr::Binary(left, op, right) => {
                let left_type = self.infer_expr_type(left, line)?;
                let right_type = self.infer_expr_type(right, line)?;

                // Check modulo restriction
                if matches!(op, BinOp::Mod) {
                    if left_type == Type::F32 || right_type == Type::F32 {
                        return Err(CompilerError::semantic(
                            line,
                            "Modulo operation not supported for f32 types".to_string(),
                        ));
                    }
                }

                // Comparison operations always return i32
                if matches!(
                    op,
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge
                ) {
                    return Ok(Type::I32);
                }

                // Arithmetic operations: widen to f32 if either operand is f32
                if left_type == Type::F32 || right_type == Type::F32 {
                    Ok(Type::F32)
                } else {
                    Ok(Type::I32)
                }
            }
            Expr::Unary(op, operand) => {
                let operand_type = self.infer_expr_type(operand, line)?;
                match op {
                    UnaryOp::Neg => Ok(operand_type), // -5 is i32, -3.14 is f32
                    UnaryOp::Not => Ok(Type::I32),    // ! always returns i32 (0 or 1)
                }
            }
            Expr::Call(name, args) => {
                // Infer argument types
                let arg_types: Vec<Type> = args
                    .iter()
                    .map(|arg| self.infer_expr_type(arg, line))
                    .collect::<Result<Vec<Type>>>()?;

                let func_info = self.functions.get_mut(name).ok_or_else(|| {
                    CompilerError::semantic(line, format!("Undefined function '{}'", name))
                })?;

                // First-call wins: set parameter types
                if func_info.param_types.is_none() {
                    func_info.param_types = Some(arg_types.clone());
                } else {
                    // Validate subsequent calls match
                    let expected_types = func_info.param_types.as_ref().unwrap();
                    if expected_types.len() != arg_types.len() {
                        return Err(CompilerError::semantic(
                            line,
                            format!(
                                "Function '{}' expects {} arguments, got {}",
                                name,
                                expected_types.len(),
                                arg_types.len()
                            ),
                        ));
                    }
                    for (i, (expected, actual)) in
                        expected_types.iter().zip(arg_types.iter()).enumerate()
                    {
                        if expected != actual {
                            return Err(CompilerError::semantic(
                                line,
                                format!(
                                    "Function '{}' parameter {} type mismatch: expected {:?}, got {:?}",
                                    name, i, expected, actual
                                ),
                            ));
                        }
                    }
                }

                // Return the function's return type
                Ok(func_info.return_type.unwrap_or(Type::I32))
            }
            Expr::Logical(left, _, right) => {
                let left_type = self.infer_expr_type(left, line)?;
                let right_type = self.infer_expr_type(right, line)?;

                // Logical operators return the type of last evaluated value
                // With auto-conversion: widen to f32 if either is f32
                if left_type == Type::F32 || right_type == Type::F32 {
                    Ok(Type::F32)
                } else {
                    Ok(Type::I32)
                }
            }
        }
    }
}
