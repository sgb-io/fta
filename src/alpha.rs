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
                let left = format!("{:?}", assign.left);
                let op = format!("{:?}", assign.op);
                let right = format!("{:?}", assign.right);
                let operator_name = format!("{} {} {}", left, op, right);
                self.unique_operators.insert(operator_name);
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
                let op = format!("{:?}", unary.op);
                let expr = format!("{:?}", unary.arg);
                let operator_name = format!("{} {}", op, expr);
                self.unique_operators.insert(operator_name);
                self.total_operators += 1;
                unary.arg.visit_with(self);
            }
            Expr::New(new_expr) => {
                self.unique_operators.insert("new".to_string());
                self.total_operators += 1;

                new_expr.callee.visit_with(self);
                if let Some(args) = &new_expr.args {
                    for arg in args {
                        arg.visit_with(self);
                    }
                }
            }
            Expr::Paren(paren_expr) => {
                // No specific operator or operand for parenthesized expressions
                paren_expr.expr.visit_with(self);
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

    fn visit_bin_expr(&mut self, node: &BinExpr) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator.clone());
        self.total_operators += 1;

        node.left.visit_with(self);
        node.right.visit_with(self);
    }

    fn visit_unary_expr(&mut self, node: &UnaryExpr) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator.clone());
        self.total_operators += 1;

        node.arg.visit_with(self);
    }

    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator.clone());
        self.total_operators += 1;

        node.left.visit_with(self);
        node.right.visit_with(self);
    }

    fn visit_update_expr(&mut self, node: &UpdateExpr) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator.clone());
        self.total_operators += 1;

        node.arg.visit_with(self);
    }

    fn visit_member_expr(&mut self, node: &MemberExpr) {
        match &node.prop {
            MemberProp::Ident(_) => {
                self.unique_operators.insert(".".to_string()); // Non-computed member access operator
            }
            MemberProp::Computed(expr) => {
                self.unique_operators.insert("[]".to_string()); // Computed member access operator
                expr.visit_with(self);
            }
            MemberProp::PrivateName(_) => {
                // Can be safely ignored, as I understand it:
                // Private class fields in JavaScript are accessed using the `.#` syntax.
                // However, this syntax is not considered an operator in the same way `.` and `[]` are.
            }
        }
        self.total_operators += 1;

        node.obj.visit_with(self);
    }

    fn visit_call_expr(&mut self, node: &CallExpr) {
        self.total_operators += 1; // Implicit call operator

        node.callee.visit_with(self);
        for arg in &node.args {
            arg.visit_with(self);
        }
    }

    fn visit_new_expr(&mut self, node: &NewExpr) {
        self.total_operators += 1; // Implicit constructor call operator

        node.callee.visit_with(self);
        if let Some(args) = &node.args {
            for arg in args {
                arg.visit_with(self);
            }
        }
    }

    fn visit_ident(&mut self, node: &Ident) {
        self.unique_operands.insert(node.sym.to_string());
        self.total_operands += 1;
    }

    fn visit_lit(&mut self, node: &Lit) {
        let lit = format!("{:?}", node);
        self.unique_operands.insert(lit.clone());
        self.total_operands += 1;
    }

    fn visit_arrow_expr(&mut self, node: &ArrowExpr) {
        self.total_operators += 1; // Implicit arrow function operator

        for param in &node.params {
            param.visit_with(self);
        }
        node.body.visit_with(self);
    }

    fn visit_tpl(&mut self, node: &Tpl) {
        for element in &node.quasis {
            element.visit_with(self);
        }
        for expr in &node.exprs {
            expr.visit_with(self);
        }
    }

    fn visit_tagged_tpl(&mut self, node: &TaggedTpl) {
        self.total_operators += 1; // Implicit tagged template operator

        node.tag.visit_with(self);
    }

    fn visit_spread_element(&mut self, node: &SpreadElement) {
        self.total_operators += 1; // Implicit spread operator

        node.expr.visit_with(self);
    }

    fn visit_ts_non_null_expr(&mut self, node: &TsNonNullExpr) {
        self.total_operators += 1; // Implicit non-null assertion operator

        node.expr.visit_with(self);
    }

    fn visit_ts_type_assertion(&mut self, node: &TsTypeAssertion) {
        self.total_operators += 1; // Implicit type assertion operator

        node.expr.visit_with(self);
        node.type_ann.visit_with(self);
    }

    fn visit_ts_as_expr(&mut self, node: &TsAsExpr) {
        self.total_operators += 1; // Implicit type cast operator (as)

        node.expr.visit_with(self);
        node.type_ann.visit_with(self);
    }

    fn visit_ts_type_operator(&mut self, node: &TsTypeOperator) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator.clone());
        self.total_operators += 1;

        node.type_ann.visit_with(self);
    }

    fn visit_ts_qualified_name(&mut self, node: &TsQualifiedName) {
        self.total_operators += 1; // Implicit qualified name operator

        node.left.visit_with(self);
        node.right.visit_with(self);
    }

    fn visit_ts_mapped_type(&mut self, node: &TsMappedType) {
        self.total_operators += 2; // Implicit key in keyof and value in mapping type operators

        node.type_param.visit_with(self);
        if let Some(type_ann) = &node.type_ann {
            type_ann.visit_with(self);
        }
    }

    fn visit_ts_indexed_access_type(&mut self, node: &TsIndexedAccessType) {
        self.total_operators += 1; // Implicit indexed access operator

        node.obj_type.visit_with(self);
        node.index_type.visit_with(self);
    }

    fn visit_cond_expr(&mut self, node: &CondExpr) {
        self.unique_operators.insert("?".to_string()); // Conditional operator '?'
        self.unique_operators.insert(":".to_string()); // Alternate operator ':'
        self.total_operators += 2; // Counting both conditional and alternate operators

        node.test.visit_with(self);
        node.cons.visit_with(self);
        node.alt.visit_with(self);
    }

    fn visit_await_expr(&mut self, node: &AwaitExpr) {
        self.unique_operators.insert("await".to_string());
        self.total_operators += 1;
        node.arg.visit_with(self);
    }

    fn visit_yield_expr(&mut self, node: &YieldExpr) {
        self.unique_operators.insert("yield".to_string());
        self.total_operators += 1;
        if let Some(arg) = &node.arg {
            arg.visit_with(self);
        }
    }

    fn visit_meta_prop_expr(&mut self, node: &MetaPropExpr) {
        self.unique_operators.insert("new.target".to_string());
        self.total_operators += 1;
        // No children to visit
    }

    fn visit_seq_expr(&mut self, node: &SeqExpr) {
        self.unique_operators.insert(",".to_string());
        self.total_operators += node.exprs.len() - 1; // n-1 commas for n expressions
        node.visit_children_with(self);
    }
}

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
