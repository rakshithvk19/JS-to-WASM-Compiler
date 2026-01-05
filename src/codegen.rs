use crate::ast::*;
use crate::error::Result;
use std::collections::HashMap;

pub struct CodeGen {
    output: Vec<String>,
    function_return_types: HashMap<String, Type>,
    label_counter: usize,
    loop_stack: Vec<usize>,
    variable_types: HashMap<String, Type>,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            output: Vec::new(),
            function_return_types: HashMap::new(),
            label_counter: 0,
            loop_stack: Vec::new(),
            variable_types: HashMap::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> Result<String> {
        self.output.push("(module".to_string());

        //Build function return type map from AST
        for func in &program.functions {
            let return_type = func.return_type.unwrap_or(Type::I32);
            self.function_return_types
                .insert(func.name.clone(), return_type);
        }

        // Generate all functions
        for func in &program.functions {
            self.gen_function(func)?;
        }

        // Generate _start function for top-level code
        self.gen_start(&program.top_level)?;

        self.output.push(")".to_string());
        Ok(self.output.join("\n"))
    }

    fn infer_expr_type_quick(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::I32,
            Expr::NumberF32(_) => Type::F32,
            Expr::Binary(left, op, right) => {
                let left_type = self.infer_expr_type_quick(left);
                let right_type = self.infer_expr_type_quick(right);

                // Comparisons return i32
                if matches!(
                    op,
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge
                ) {
                    return Type::I32;
                }

                // Arithmetic: widen to f32 if either is f32
                if left_type == Type::F32 || right_type == Type::F32 {
                    Type::F32
                } else {
                    Type::I32
                }
            }
            Expr::Unary(op, operand) => match op {
                UnaryOp::Neg => self.infer_expr_type_quick(operand),
                UnaryOp::Not => Type::I32,
            },
            Expr::Logical(left, _, right) => {
                let left_type = self.infer_expr_type_quick(left);
                let right_type = self.infer_expr_type_quick(right);
                if left_type == Type::F32 || right_type == Type::F32 {
                    Type::F32
                } else {
                    Type::I32
                }
            }

            Expr::Identifier(name) => {
                // Look up variable type from the type map
                self.variable_types.get(name).copied().unwrap_or(Type::I32)
            }
            Expr::Call(name, _) => {
                // LOOK UP FUNCTION RETURN TYPE FROM THE MAP
                self.function_return_types
                    .get(name)
                    .copied()
                    .unwrap_or(Type::I32)
            }
        }
    }

    fn gen_function(&mut self, func: &Function) -> Result<()> {
        self.variable_types.clear();

        // Collect variable types from statements
        self.collect_variable_types(&func.body);

        // Get types from AST
        let default_param_types = vec![Type::I32; func.params.len()];
        let param_types = func.param_types.as_ref().unwrap_or(&default_param_types);
        let return_type = func.return_type.unwrap_or(Type::I32);

        // Add param types to variable_types
        for (param, param_type) in func.params.iter().zip(param_types.iter()) {
            self.variable_types.insert(param.clone(), *param_type);
        }

        let locals = self.collect_locals(&func.body, &func.params);

        // Generate typed parameter declarations
        let params: Vec<String> = func
            .params
            .iter()
            .zip(param_types.iter())
            .map(|(p, t)| format!("(param ${} {})", p, type_to_wasm(*t)))
            .collect();

        // Generate typed local declarations
        let local_decls: Vec<String> = locals
            .iter()
            .map(|l| {
                let var_type = self.variable_types.get(l).copied().unwrap_or(Type::I32);
                format!("(local ${} {})", l, type_to_wasm(var_type))
            })
            .collect();

        self.output.push(format!(
            "  (func ${} (export \"{}\") {} (result {}) ;; line {}",
            func.name,
            func.name,
            params.join(" "),
            type_to_wasm(return_type),
            func.line
        ));

        for decl in local_decls {
            self.output.push(format!("    {}", decl));
        }

        // Add $_result with correct type
        let result_type = if return_type == Type::F32 {
            "f32"
        } else {
            "i32"
        };
        self.output
            .push(format!("    (local $_result {})", result_type));

        let all_vars: Vec<String> = func.params.iter().chain(locals.iter()).cloned().collect();

        for stmt in &func.body {
            self.gen_stmt(stmt, &all_vars)?;
        }

        // Default return value
        if return_type == Type::F32 {
            self.output.push("    f32.const 0.0".to_string());
        } else {
            self.output.push("    i32.const 0".to_string());
        }
        self.output.push("  )".to_string());
        Ok(())
    }

    fn collect_variable_types(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match &stmt.kind {
                StmtKind::Let(name, expr) | StmtKind::Const(name, expr) => {
                    let expr_type = self.infer_expr_type_quick(expr);
                    self.variable_types.insert(name.clone(), expr_type);
                }
                StmtKind::Block(inner) => self.collect_variable_types(inner),
                StmtKind::If(_, then_branch, else_branch) => {
                    if let StmtKind::Block(stmts) = &then_branch.kind {
                        self.collect_variable_types(stmts);
                    }
                    if let Some(eb) = else_branch {
                        if let StmtKind::Block(stmts) = &eb.kind {
                            self.collect_variable_types(stmts);
                        }
                    }
                }
                StmtKind::While(_, body) => {
                    if let StmtKind::Block(stmts) = &body.kind {
                        self.collect_variable_types(stmts);
                    }
                }
                StmtKind::For(init, _, _, body) => {
                    // Collect types from init statement
                    if let Some(init_stmt) = init {
                        match &init_stmt.kind {
                            StmtKind::Let(name, expr) | StmtKind::Const(name, expr) => {
                                let expr_type = self.infer_expr_type_quick(expr);
                                self.variable_types.insert(name.clone(), expr_type);
                            }
                            _ => {}
                        }
                    }
                    if let StmtKind::Block(stmts) = &body.kind {
                        self.collect_variable_types(stmts);
                    }
                }
                _ => {}
            }
        }
    }

    fn gen_start(&mut self, stmts: &[Stmt]) -> Result<()> {
        self.variable_types.clear();
        self.collect_variable_types(stmts);

        let locals = self.collect_locals(stmts, &[]);

        // Generate typed local declarations
        let local_decls: Vec<String> = locals
            .iter()
            .map(|l| {
                let var_type = self.variable_types.get(l).copied().unwrap_or(Type::I32);
                format!("(local ${} {})", l, type_to_wasm(var_type))
            })
            .collect();

        // Infer _start return type from last expression
        let start_return_type = self.infer_start_return_type(stmts);

        self.output.push(format!(
            "  (func $_start (export \"_start\") (result {})",
            type_to_wasm(start_return_type)
        ));

        for decl in local_decls {
            self.output.push(format!("    {}", decl));
        }

        // Track the last expression value with correct type
        self.output.push(format!(
            "    (local $_result {})",
            type_to_wasm(start_return_type)
        ));

        for stmt in stmts {
            self.gen_stmt_with_result(stmt, &locals)?;
        }

        self.output.push("    local.get $_result".to_string());
        self.output.push("  )".to_string());
        Ok(())
    }

    fn infer_start_return_type(&self, stmts: &[Stmt]) -> Type {
        if let Some(last) = stmts.last() {
            if let StmtKind::Expr(expr) = &last.kind {
                return self.infer_expr_type_quick(expr);
            }
        }
        Type::I32
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
                StmtKind::For(init, _, _, body) => {
                    if let Some(init_stmt) = init {
                        self.collect_locals_rec(&[*init_stmt.clone()], locals);
                    }
                    self.collect_locals_rec(&[*body.clone()], locals);
                }
                _ => {}
            }
        }
    }

    fn emit_line_comment(&mut self, line: usize) {
        self.output.push(format!("    ;; line {}", line));
    }

    fn gen_stmt(&mut self, stmt: &Stmt, vars: &[String]) -> Result<()> {
        self.emit_line_comment(stmt.line);
        match &stmt.kind {
            StmtKind::Let(name, expr) => {
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::Const(name, expr) => {
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::Assign(name, expr) => {
                self.gen_expr(expr, vars);
                self.output.push(format!("    local.set ${}", name));
            }
            StmtKind::If(cond, then_branch, else_branch) => {
                self.gen_expr(cond, vars);
                // Convert f32 to i32 for condition check
                let cond_type = self.infer_expr_type_quick(cond);
                if cond_type == Type::F32 {
                    self.output.push("    f32.const 0.0".to_string());
                    self.output.push("    f32.ne".to_string()); // f32 != 0.0
                }

                if else_branch.is_some() {
                    self.output.push("    if".to_string());
                    self.gen_stmt(then_branch, vars)?;
                    self.output.push("    else".to_string());
                    self.gen_stmt(else_branch.as_ref().unwrap(), vars)?;
                    self.output.push("    end".to_string());
                } else {
                    self.output.push("    if".to_string());
                    self.gen_stmt(then_branch, vars)?;
                    self.output.push("    end".to_string());
                }
            }
            StmtKind::While(cond, body) => {
                let id = self.label_counter;
                self.label_counter += 1;
                self.loop_stack.push(id);

                self.output.push(format!("    block $break_{}", id));
                self.output.push(format!("    loop $continue_{}", id));
                self.gen_expr(cond, vars);

                // Convert f32 to boolean for condition
                let cond_type = self.infer_expr_type_quick(cond);
                if cond_type == Type::F32 {
                    self.output.push("    f32.const 0.0".to_string());
                    self.output.push("    f32.eq".to_string()); // Check if == 0.0 (false)
                } else {
                    self.output.push("    i32.eqz".to_string());
                }

                self.output.push(format!("    br_if $break_{}", id));
                self.gen_stmt(body, vars)?;
                self.output.push(format!("    br $continue_{}", id));
                self.output.push("    end".to_string());
                self.output.push("    end".to_string());

                self.loop_stack.pop();
            }
            StmtKind::For(init, cond, incr, body) => {
                if let Some(init_stmt) = init {
                    self.gen_stmt(init_stmt, vars)?;
                }

                let id = self.label_counter;
                self.label_counter += 1;
                self.loop_stack.push(id);

                self.output.push(format!("    block $break_{}", id));
                self.output.push(format!("    loop $loop_{}", id));

                if let Some(cond_expr) = cond {
                    self.gen_expr(cond_expr, vars);

                    // Convert f32 to boolean
                    let cond_type = self.infer_expr_type_quick(cond_expr);
                    if cond_type == Type::F32 {
                        self.output.push("    f32.const 0.0".to_string());
                        self.output.push("    f32.eq".to_string());
                    } else {
                        self.output.push("    i32.eqz".to_string());
                    }

                    self.output.push(format!("    br_if $break_{}", id));
                }

                self.output.push(format!("    block $continue_{}", id));
                self.gen_stmt(body, vars)?;
                self.output.push("    end".to_string());

                if let Some(incr_stmt) = incr {
                    self.gen_stmt(incr_stmt, vars)?;
                }

                self.output.push(format!("    br $loop_{}", id));
                self.output.push("    end".to_string());
                self.output.push("    end".to_string());

                self.loop_stack.pop();
            }
            StmtKind::Block(stmts) => {
                for s in stmts {
                    self.gen_stmt(s, vars)?;
                }
            }
            StmtKind::Return(expr) => {
                if let Expr::Call(name, args) = expr {
                    // Tail call
                    for arg in args {
                        self.gen_expr(arg, vars);
                    }
                    self.output.push(format!("    return_call ${}", name));
                } else {
                    self.gen_expr(expr, vars);
                    self.output.push("    return".to_string());
                }
            }
            StmtKind::Break => {
                if let Some(&loop_id) = self.loop_stack.last() {
                    self.output.push(format!("    br $break_{}", loop_id));
                }
            }
            StmtKind::Continue => {
                if let Some(&loop_id) = self.loop_stack.last() {
                    self.output.push(format!("    br $continue_{}", loop_id));
                }
            }
            StmtKind::Expr(expr) => {
                self.gen_expr(expr, vars);
                self.output.push("    drop".to_string());
            }
        }
        Ok(())
    }

    fn gen_stmt_with_result(&mut self, stmt: &Stmt, vars: &[String]) -> Result<()> {
        self.emit_line_comment(stmt.line);
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.gen_expr(expr, vars);
                self.output.push("    local.set $_result".to_string());
            }
            _ => self.gen_stmt(stmt, vars)?,
        }
        Ok(())
    }

    fn gen_expr(&mut self, expr: &Expr, vars: &[String]) {
        match expr {
            Expr::Number(n) => {
                self.output.push(format!("    i32.const {}", n));
            }
            Expr::NumberF32(f) => {
                self.output.push(format!("    f32.const {}", f));
            }
            Expr::Identifier(name) => {
                self.output.push(format!("    local.get ${}", name));
            }
            Expr::Binary(left, op, right) => {
                let left_type = self.infer_expr_type_quick(left);
                let right_type = self.infer_expr_type_quick(right);

                // Generate left operand
                self.gen_expr(left, vars);
                // Convert if needed
                if left_type == Type::I32 && right_type == Type::F32 {
                    self.output.push("    f32.convert_i32_s".to_string());
                }

                // Generate right operand
                self.gen_expr(right, vars);
                // Convert if needed
                if right_type == Type::I32 && left_type == Type::F32 {
                    self.output.push("    f32.convert_i32_s".to_string());
                }

                // Determine which instruction to use
                let use_f32 = left_type == Type::F32 || right_type == Type::F32;

                let instr = if use_f32 {
                    match op {
                        BinOp::Add => "f32.add",
                        BinOp::Sub => "f32.sub",
                        BinOp::Mul => "f32.mul",
                        BinOp::Div => "f32.div",
                        BinOp::Mod => "f32.rem",
                        BinOp::Eq => "f32.eq",
                        BinOp::Ne => "f32.ne",
                        BinOp::Lt => "f32.lt",
                        BinOp::Gt => "f32.gt",
                        BinOp::Le => "f32.le",
                        BinOp::Ge => "f32.ge",
                    }
                } else {
                    match op {
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
                    }
                };
                self.output.push(format!("    {}", instr));
            }
            Expr::Unary(op, operand) => {
                let operand_type = self.infer_expr_type_quick(operand);

                match op {
                    UnaryOp::Neg => {
                        if operand_type == Type::F32 {
                            self.gen_expr(operand, vars);
                            self.output.push("    f32.neg".to_string());
                        } else {
                            self.output.push("    i32.const 0".to_string());
                            self.gen_expr(operand, vars);
                            self.output.push("    i32.sub".to_string());
                        }
                    }
                    UnaryOp::Not => {
                        self.gen_expr(operand, vars);
                        if operand_type == Type::F32 {
                            // Convert f32 to boolean (0.0 = false, else = true)
                            self.output.push("    f32.const 0.0".to_string());
                            self.output.push("    f32.eq".to_string()); // Returns i32
                        } else {
                            self.output.push("    i32.eqz".to_string());
                        }
                    }
                }
            }
            Expr::Call(name, args) => {
                for arg in args {
                    self.gen_expr(arg, vars);
                }
                self.output.push(format!("    call ${}", name));
            }
            Expr::Logical(left, op, right) => {
                let left_type = self.infer_expr_type_quick(left);
                let right_type = self.infer_expr_type_quick(right);
                let result_type = if left_type == Type::F32 || right_type == Type::F32 {
                    Type::F32
                } else {
                    Type::I32
                };

                match op {
                    LogicalOp::And => {
                        self.gen_expr(left, vars);

                        // Convert left to result_type if needed
                        if left_type == Type::I32 && result_type == Type::F32 {
                            self.output.push("    f32.convert_i32_s".to_string());
                        }

                        self.output.push("    local.tee $_result".to_string());

                        // Check truthiness based on the type currently on stack
                        if result_type == Type::F32 {
                            self.output.push("    f32.const 0.0".to_string());
                            self.output.push("    f32.eq".to_string());
                        } else {
                            self.output.push("    i32.eqz".to_string());
                        }

                        self.output
                            .push(format!("    if (result {})", type_to_wasm(result_type)));
                        self.output.push("    local.get $_result".to_string());
                        self.output.push("    else".to_string());
                        self.gen_expr(right, vars);

                        // Convert right to result_type if needed
                        if right_type == Type::I32 && result_type == Type::F32 {
                            self.output.push("    f32.convert_i32_s".to_string());
                        }

                        self.output.push("    end".to_string());
                    }
                    LogicalOp::Or => {
                        self.gen_expr(left, vars);

                        // Convert left to result_type if needed
                        if left_type == Type::I32 && result_type == Type::F32 {
                            self.output.push("    f32.convert_i32_s".to_string());
                        }

                        self.output.push("    local.tee $_result".to_string());

                        // Check truthiness based on the type currently on stack
                        if result_type == Type::F32 {
                            self.output.push("    f32.const 0.0".to_string());
                            self.output.push("    f32.ne".to_string());
                        } else {
                            // For i32 OR: if truthy (non-zero), return left
                            self.output.push("    i32.const 0".to_string());
                            self.output.push("    i32.ne".to_string());
                        }

                        self.output
                            .push(format!("    if (result {})", type_to_wasm(result_type)));
                        self.output.push("    local.get $_result".to_string());
                        self.output.push("    else".to_string());
                        self.gen_expr(right, vars);

                        // Convert right to result_type if needed
                        if right_type == Type::I32 && result_type == Type::F32 {
                            self.output.push("    f32.convert_i32_s".to_string());
                        }

                        self.output.push("    end".to_string());
                    }
                }
            }
        }
    }
}

// Helper function to convert Type to WASM type string
fn type_to_wasm(t: Type) -> &'static str {
    match t {
        Type::I32 => "i32",
        Type::F32 => "f32",
    }
}
