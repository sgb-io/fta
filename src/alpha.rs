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
            Expr::Lit(lit) => {
                match lit {
                    Lit::Str(s) => {
                        self.unique_operands.insert(s.value.to_string());
                    }
                    Lit::Num(n) => {
                        self.unique_operands.insert(n.value.to_string());
                    }
                    Lit::Bool(b) => {
                        self.unique_operands.insert(b.value.to_string());
                    }
                    _ => {}
                }
                self.total_operands += 1;
            }
            _ => expr.visit_children_with(self),
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
