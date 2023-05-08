use std::collections::HashSet;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

#[derive(Debug)]
struct AstAnalyzer {
    unique_operators: HashSet<BinaryOp>,
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
                self.unique_operators.insert(binary_expr.op);
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
                // self.unique_operators.insert(unary.op);
                unary.arg.visit_with(self);
            }
            _ => {
                println!("Unhandled expression: {:?}", expr);
                expr.visit_children_with(self);
            }
        }
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
