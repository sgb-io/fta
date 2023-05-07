use swc_ecma_ast::Module;
use swc_ecma_visit::{Visit, VisitWith};

struct ComplexityVisitor {
    complexity: u32,
}

impl ComplexityVisitor {
    fn new() -> Self {
        ComplexityVisitor { complexity: 1 }
    }
}

impl Visit for ComplexityVisitor {
    fn visit_bin_expr(&mut self, expr: &swc_ecma_ast::BinExpr) {
        let op = expr.op.as_str();
        if op == "&&" || op == "||" {
            self.complexity += 1;
        }
        expr.visit_children_with(self);
    }

    fn visit_if_stmt(&mut self, stmt: &swc_ecma_ast::IfStmt) {
        self.complexity += 1;
        stmt.visit_children_with(self);
    }

    fn visit_switch_stmt(&mut self, stmt: &swc_ecma_ast::SwitchStmt) {
        self.complexity += 1;
        stmt.visit_children_with(self);
    }

    fn visit_for_stmt(&mut self, _: &swc_ecma_ast::ForStmt) {
        self.complexity += 1;
    }

    fn visit_while_stmt(&mut self, _: &swc_ecma_ast::WhileStmt) {
        self.complexity += 1;
    }

    fn visit_do_while_stmt(&mut self, _: &swc_ecma_ast::DoWhileStmt) {
        self.complexity += 1;
    }
}

pub fn cyclomatic_complexity(module: Module) -> u32 {
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_module(&module);
    visitor.complexity
}
