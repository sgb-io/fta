use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

mod tests;

struct ComplexityVisitor {
    complexity: usize,
}

impl ComplexityVisitor {
    fn new() -> Self {
        ComplexityVisitor { complexity: 1 }
    }
}

impl Visit for ComplexityVisitor {
    fn visit_bin_expr(&mut self, node: &BinExpr) {
        let op = node.op.as_str();
        if op == "&&" || op == "||" {
            self.complexity += 1;
        }
        node.visit_children_with(self);
    }

    fn visit_if_stmt(&mut self, node: &IfStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_switch_stmt(&mut self, node: &SwitchStmt) {
        // Count each case as a decision point
        self.complexity += node.cases.len();

        // Traverse the child nodes (cases and their statements)
        node.visit_children_with(self);
    }

    fn visit_for_stmt(&mut self, node: &ForStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_while_stmt(&mut self, node: &WhileStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_do_while_stmt(&mut self, node: &DoWhileStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_for_in_stmt(&mut self, node: &ForInStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_for_of_stmt(&mut self, node: &ForOfStmt) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_catch_clause(&mut self, node: &CatchClause) {
        self.complexity += 1;
        node.visit_children_with(self);
    }

    fn visit_cond_expr(&mut self, node: &CondExpr) {
        self.complexity += 1;
        node.visit_children_with(self);
    }
}

pub fn cyclomatic_complexity(module: &Module) -> usize {
    let mut visitor = ComplexityVisitor::new();
    visitor.visit_module(&module);
    visitor.complexity
}
