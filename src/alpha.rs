use std::collections::HashSet;
use swc_atoms::Atom;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Debug)]
struct AstAnalyzer {
    unique_operators: HashSet<String>,
    unique_operands: HashSet<String>,
    total_operators: usize,
    total_operands: usize,
}

impl AstAnalyzer {
    fn new() -> Self {
        AstAnalyzer {
            unique_operators: HashSet::new(),
            unique_operands: HashSet::new(),
            total_operators: 0,
            total_operands: 0,
        }
    }
}

impl Visit for AstAnalyzer {
    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Bin(binary_expr) => {
                self.unique_operators.insert(binary_expr.op.to_string());
                self.total_operators += 1;
                binary_expr.left.visit_with(self);
                binary_expr.right.visit_with(self);
            }
            Expr::Ident(ident) => {
                self.unique_operands.insert(ident.sym.to_string());
                self.total_operands += 1;
            }
            Expr::Lit(lit) => match lit {
                Lit::Str(str_lit) => {
                    let value = &str_lit.value;
                    self.total_operands += 1;
                    self.unique_operands.insert(value.to_string());
                }
                Lit::Bool(bool_lit) => {
                    let value = bool_lit.value.to_string();
                    self.total_operands += 1;
                    self.unique_operands.insert(value);
                }
                Lit::Null(_) => {
                    self.total_operands += 1;
                    self.unique_operands.insert("null".to_string());
                }
                Lit::Num(num_lit) => {
                    let value = num_lit.value.to_string();
                    self.total_operands += 1;
                    self.unique_operands.insert(value);
                }
                _ => {
                    println!("Unhandled Lit expression: {:?}", lit);
                }
            },
            Expr::Array(array) => {
                for elem in &array.elems {
                    if let Some(expr) = elem {
                        expr.visit_with(self);
                    }
                }
            }
            Expr::Arrow(arrow) => {
                arrow.params.visit_with(self);
                arrow.body.visit_with(self);
            }
            Expr::Assign(assign) => {
                self.total_operators += 1;
                assign.left.visit_with(self);
                assign.right.visit_with(self);
            }
            Expr::Call(call) => {
                call.callee.visit_with(self);
                for arg in &call.args {
                    arg.visit_with(self);
                }
            }
            Expr::Cond(cond) => {
                cond.test.visit_with(self);
                cond.cons.visit_with(self);
                cond.alt.visit_with(self);
            }
            Expr::Member(member) => {
                member.obj.visit_with(self);
                member.prop.visit_with(self);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    match prop {
                        PropOrSpread::Prop(boxed_prop) => match &**boxed_prop {
                            Prop::KeyValue(key_value) => {
                                key_value.key.visit_with(self);
                                key_value.value.visit_with(self);
                            }
                            Prop::Assign(assign) => {
                                assign.key.visit_with(self);
                                assign.value.visit_with(self);
                            }
                            // TODO Shorthand
                            _ => {
                                println!("Unhandled Object property: {:?}", boxed_prop);
                            }
                        },
                        PropOrSpread::Spread(spread) => {
                            spread.expr.visit_with(self);
                        }
                    }
                }
            }
            Expr::Tpl(tpl) => {
                for expr in &tpl.exprs {
                    expr.visit_with(self);
                }
            }
            Expr::TsAs(ts_as) => {
                ts_as.expr.visit_with(self);
                // No need to visit the type_ann as it doesn't contribute to operands or operators.
            }
            Expr::TsNonNull(ts_non_null) => {
                ts_non_null.expr.visit_with(self);
            }
            Expr::Unary(unary) => {
                self.total_operators += 1;
                unary.arg.visit_with(self);
            }
            // TODO New, Paren
            Expr::New(new_expr) => {
                // TODO implement
            }
            Expr::Paren(paren_expr) => {
                // TODO implement
            }
            _ => {
                println!("Unhandled expression: {:?}", expr);
                expr.visit_children_with(self);
            }
        }
    }

    fn visit_export_decl(&mut self, export_decl: &ExportDecl) {
        self.total_operators += 1;
        self.unique_operators.insert("export".to_string());

        // Continue visiting the declaration
        export_decl.visit_children_with(self);
    }

    fn visit_pat(&mut self, pat: &Pat) {
        match pat {
            Pat::Ident(ident) => {
                let ident_str = ident.sym.as_ref().to_string();
                self.unique_operands.insert(ident_str.clone());
                self.total_operands += 1;
            }
            _ => {
                // Handle other patterns if necessary or visit their children
                pat.visit_children_with(self);
            }
        }
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        self.total_operators += 1;
        self.unique_operators.insert("function".to_string());

        fn_decl.visit_children_with(self);
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        self.total_operators += 1;
        self.unique_operators.insert("class".to_string());

        class_decl.visit_children_with(self);
    }

    fn visit_class_method(&mut self, class_method: &ClassMethod) {
        self.total_operators += 1;
        self.unique_operators.insert("method".to_string());

        class_method.visit_children_with(self);
    }

    fn visit_var_declarator(&mut self, var_declarator: &VarDeclarator) {
        self.total_operators += 1;
        self.unique_operators.insert("variable".to_string());

        var_declarator.visit_children_with(self);
    }

    fn visit_arrow_expr(&mut self, arrow_expr: &ArrowExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("=>".to_string());

        arrow_expr.visit_children_with(self);
    }

    fn visit_update_expr(&mut self, update_expr: &UpdateExpr) {
        self.total_operators += 1;
        let op_str = format!("{:?}", update_expr.op);
        self.unique_operators.insert(op_str);

        update_expr.visit_children_with(self);
    }

    fn visit_assign_expr(&mut self, assign_expr: &AssignExpr) {
        self.total_operators += 1;
        let op_str = format!("{:?}", assign_expr.op);
        self.unique_operators.insert(op_str);

        assign_expr.visit_children_with(self);
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) {
        self.total_operators += 1;
        let op_str = format!("{:?}", unary_expr.op);
        self.unique_operators.insert(op_str);

        unary_expr.visit_children_with(self);
    }

    fn visit_member_expr(&mut self, member_expr: &MemberExpr) {
        self.total_operators += 1;
        self.unique_operators.insert(".".to_string());

        member_expr.visit_children_with(self);
    }

    fn visit_new_expr(&mut self, new_expr: &NewExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("new".to_string());

        new_expr.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("()".to_string());

        call_expr.visit_children_with(self);
    }

    fn visit_cond_expr(&mut self, conditional_expr: &CondExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("?".to_string());

        conditional_expr.visit_children_with(self);
    }

    fn visit_tpl(&mut self, template_literal: &Tpl) {
        self.total_operators += 1;
        self.unique_operators.insert("`".to_string());

        template_literal.visit_children_with(self);
    }

    fn visit_for_stmt(&mut self, for_stmt: &ForStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("for".to_string());

        for_stmt.visit_children_with(self);
    }

    fn visit_for_in_stmt(&mut self, for_in_stmt: &ForInStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("for-in".to_string());

        for_in_stmt.visit_children_with(self);
    }

    fn visit_for_of_stmt(&mut self, for_of_stmt: &ForOfStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("for-of".to_string());

        for_of_stmt.visit_children_with(self);
    }

    fn visit_while_stmt(&mut self, while_stmt: &WhileStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("while".to_string());

        while_stmt.visit_children_with(self);
    }

    fn visit_do_while_stmt(&mut self, do_while_stmt: &DoWhileStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("do-while".to_string());

        do_while_stmt.visit_children_with(self);
    }

    fn visit_if_stmt(&mut self, if_stmt: &IfStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("if".to_string());

        if_stmt.visit_children_with(self);
    }

    fn visit_switch_stmt(&mut self, switch_stmt: &SwitchStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("switch".to_string());

        switch_stmt.visit_children_with(self);
    }

    fn visit_break_stmt(&mut self, break_stmt: &BreakStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("break".to_string());

        break_stmt.visit_children_with(self);
    }

    fn visit_continue_stmt(&mut self, continue_stmt: &ContinueStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("continue".to_string());

        continue_stmt.visit_children_with(self);
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("return".to_string());

        return_stmt.visit_children_with(self);
    }

    fn visit_throw_stmt(&mut self, throw_stmt: &ThrowStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("throw".to_string());

        throw_stmt.visit_children_with(self);
    }

    fn visit_tagged_tpl(&mut self, tagged_tpl_expr: &TaggedTpl) {
        self.total_operators += 1;
        self.unique_operators.insert("tagged-template".to_string());

        tagged_tpl_expr.visit_children_with(self);
    }

    fn visit_await_expr(&mut self, await_expr: &AwaitExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("await".to_string());

        await_expr.visit_children_with(self);
    }

    fn visit_yield_expr(&mut self, yield_expr: &YieldExpr) {
        self.total_operators += 1;
        self.unique_operators.insert("yield".to_string());

        yield_expr.visit_children_with(self);
    }

    fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
        self.total_operators += 1;
        self.unique_operators.insert("import".to_string());

        import_decl.visit_children_with(self);
    }

    fn visit_export_default_decl(&mut self, export_default_decl: &ExportDefaultDecl) {
        self.total_operators += 1;
        self.unique_operators.insert("export-default".to_string());

        export_default_decl.visit_children_with(self);
    }

    fn visit_try_stmt(&mut self, try_stmt: &TryStmt) {
        self.total_operators += 1;
        self.unique_operators.insert("try".to_string());

        try_stmt.visit_children_with(self);
    }

    fn visit_assign_op(&mut self, assign_op: &AssignOp) {
        self.total_operators += 1;
        self.unique_operators.insert(assign_op.to_string());

        assign_op.visit_children_with(self);
    }

    fn visit_big_int(&mut self, big_int: &BigInt) {
        self.total_operands += 1;
        self.unique_operands.insert(big_int.value.to_string());

        big_int.visit_children_with(self);
    }

    fn visit_bin_expr(&mut self, bin_expr: &BinExpr) {
        self.total_operators += 1;
        self.unique_operators
            .insert(bin_expr.op.as_str().to_string());

        bin_expr.visit_children_with(self);
    }

    fn visit_bool(&mut self, bool_lit: &Bool) {
        self.total_operands += 1;
        self.unique_operands.insert(bool_lit.value.to_string());

        bool_lit.visit_children_with(self);
    }

    fn visit_number(&mut self, _number: &Number) {
        self.total_operands += 1;
    }

    fn visit_param(&mut self, p: &Param) {
        match &p.pat {
            Pat::Ident(_ident) => self.total_operands += 1,
            _ => {}
        }
    }

    fn visit_private_name(&mut self, _name: &PrivateName) {
        self.total_operands += 1;
    }

    fn visit_prop_or_spreads(&mut self, node: &[PropOrSpread]) {
        for child_node in node.iter() {
            self.visit_prop_or_spread(child_node);
        }
    }

    fn visit_str(&mut self, str_node: &Str) {
        self.unique_operands.insert(str_node.value.to_string());
        self.total_operands += 1;
        str_node.visit_children_with(self);
    }

    fn visit_reserved_unused(&mut self, reserved_word: &ReservedUnused) {
        let operand = format!("{:?}", reserved_word);
        self.unique_operands.insert(operand);
        self.total_operands += 1;
        reserved_word.visit_children_with(self);
    }

    fn visit_rest_pat(&mut self, rest_pat: &RestPat) {
        let operand = format!("{:?}", rest_pat.arg);
        self.unique_operands.insert(operand);
        self.total_operands += 1;
        rest_pat.visit_children_with(self);
    }

    fn visit_seq_expr(&mut self, seq_expr: &SeqExpr) {
        self.unique_operators.insert(",".to_string());
        self.total_operators += 1;
        seq_expr.visit_children_with(self);
    }

    fn visit_setter_prop(&mut self, setter_prop: &SetterProp) {
        self.unique_operators.insert("=".to_string());
        self.total_operators += 1;
        setter_prop.visit_children_with(self);
    }

    fn visit_spread_element(&mut self, spread_element: &SpreadElement) {
        self.unique_operators.insert("...".to_string());
        self.total_operators += 1;
        spread_element.visit_children_with(self);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        self.unique_operators.insert(";".to_string());
        self.total_operators += 1;
        stmt.visit_children_with(self);
    }

    fn visit_stmts(&mut self, stmts: &[Stmt]) {
        self.unique_operators.insert(";".to_string());
        self.total_operators += 1;
        stmts.visit_children_with(self);
    }

    fn visit_super(&mut self, _super_node: &Super) {
        self.unique_operands.insert("super".to_string());
        self.total_operands += 1;
        _super_node.visit_children_with(self);
    }

    fn visit_super_prop(&mut self, super_prop: &SuperProp) {
        self.unique_operands.insert("super".to_string());
        self.total_operands += 1;

        self.unique_operators.insert(".".to_string());
        self.total_operators += 1;

        super_prop.visit_children_with(self);
    }

    fn visit_super_prop_expr(&mut self, super_prop_expr: &SuperPropExpr) {
        self.unique_operands.insert("super".to_string());
        self.total_operands += 1;

        self.unique_operators.insert(".".to_string());
        self.total_operators += 1;

        super_prop_expr.prop.visit_with(self);
    }

    fn visit_this_expr(&mut self, node: &ThisExpr) {
        self.unique_operands.insert("this".to_string());
        self.total_operands += 1;
        node.visit_children_with(self);
    }

    fn visit_tpl_element(&mut self, node: &TplElement) {
        // This may be incorrect, but it's difficult to know how to stringify long templates
        self.unique_operands.insert("template".to_string());
        self.total_operands += 1;
        // self.operators_count += node.raw.value.matches(|c| !c.is_whitespace()).count() - 1;
        node.visit_children_with(self);
    }

    fn visit_true_plus_minus(&mut self, node: &TruePlusMinus) {
        self.unique_operands.insert("+-".to_string());
        node.visit_children_with(self);
    }
}

// TODO methods that do not count operands or operators can probably be removed? Since these are all overrides

pub fn analyze_module(module: &Module) -> (usize, usize, usize, usize) {
    let mut analyzer = AstAnalyzer::new();
    module.visit_with(&mut analyzer);

    (
        analyzer.unique_operators.len(),
        analyzer.unique_operands.len(),
        analyzer.total_operators,
        analyzer.total_operands,
    )
}
