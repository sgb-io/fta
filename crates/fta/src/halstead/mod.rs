use crate::structs::HalsteadMetrics;
use log::debug;
use std::collections::HashSet;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

mod tests;

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
                Lit::Regex(regex) => {
                    let regex_literal = format!("/{}/{}", regex.exp, regex.flags);
                    self.unique_operands.insert(regex_literal);
                    self.total_operands += 1;
                }
                _ => {
                    debug!(
                        "visit_expr(Expr::Lit): Literal expression assumed to not count towards operators and operands: {:?}",
                        lit
                    );
                }
            },
            Expr::Array(array) => {
                for elem in &array.elems {
                    if let Some(element) = elem {
                        match element {
                            ExprOrSpread { expr, spread } => {
                                if spread.is_some() {
                                    self.unique_operators.insert("...".to_string());
                                    self.total_operators += 1;
                                }
                                expr.visit_with(self);
                            }
                        }
                    }
                }
            }
            Expr::Arrow(arrow) => {
                self.unique_operators.insert("=>".to_string());
                self.total_operators += 1;

                arrow.params.visit_with(self);
                arrow.body.visit_with(self);
            }
            Expr::Assign(assign) => {
                self.unique_operators.insert(assign.op.to_string());
                self.total_operators += 1;

                assign.left.visit_with(self);
                assign.right.visit_with(self);
            }
            Expr::Call(call) => {
                self.unique_operators.insert("()".to_string());
                self.total_operators += 1;

                call.callee.visit_with(self);
                for arg in &call.args {
                    arg.visit_with(self);
                }
            }
            Expr::Cond(cond) => {
                self.unique_operators.insert("?".to_string());
                self.unique_operators.insert(":".to_string());
                self.total_operators += 2;

                cond.test.visit_with(self);
                cond.cons.visit_with(self);
                cond.alt.visit_with(self);
            }
            Expr::Member(member) => {
                self.unique_operators.insert(".".to_string());
                self.total_operators += 1;
                member.obj.visit_with(self);
                member.prop.visit_with(self);
            }
            Expr::Object(object) => {
                for prop in &object.props {
                    match prop {
                        PropOrSpread::Prop(boxed_prop) => match &**boxed_prop {
                            Prop::KeyValue(key_value) => {
                                self.unique_operators.insert(":".to_string());
                                self.total_operators += 1;
                                key_value.key.visit_with(self);
                                key_value.value.visit_with(self);
                            }
                            Prop::Assign(assign) => {
                                self.unique_operators.insert("=".to_string());
                                self.total_operators += 1;
                                assign.key.visit_with(self);
                                assign.value.visit_with(self);
                            }
                            Prop::Shorthand(ident) => {
                                ident.visit_with(self);
                            }
                            Prop::Method(method_prop) => {
                                method_prop.key.visit_with(self);
                                method_prop.function.visit_with(self);
                            }
                            _ => {
                                debug!(
                                    "visit_expr(Expr::Object): Object prop assumed to not count towards operators and operands: {:?}",
                                    boxed_prop
                                );
                            }
                        },
                        PropOrSpread::Spread(spread) => {
                            spread.expr.visit_with(self);
                        }
                    }
                }
            }
            Expr::Tpl(tpl) => {
                self.unique_operators.insert("`".to_string()); // Template literal backticks
                self.total_operators += 2; // Opening and closing backticks

                for expr in &tpl.exprs {
                    self.unique_operators.insert("${".to_string()); // Expression interpolation
                    self.total_operators += 1;
                    expr.visit_with(self);
                }
            }
            Expr::TsAs(ts_as) => {
                self.unique_operators.insert("TsAs".to_string());
                self.total_operators += 1;
                ts_as.expr.visit_with(self);
                ts_as.type_ann.visit_with(self);
                // No need to visit the type_ann as it doesn't contribute to operands or operators.
            }
            Expr::TsNonNull(ts_non_null) => {
                self.unique_operators.insert("TsNonNull".to_string());
                self.total_operators += 1;
                ts_non_null.expr.visit_with(self);
            }
            Expr::Unary(unary) => {
                self.unique_operators.insert(unary.op.to_string());
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
                self.unique_operators.insert("(".to_string());
                self.unique_operators.insert(")".to_string());
                self.total_operators += 2;

                paren_expr.expr.visit_with(self);
            }
            Expr::Update(update) => {
                self.unique_operators.insert(update.op.to_string());
                self.total_operators += 1;
                update.arg.visit_with(self);
            }
            Expr::OptChain(opt_chain) => {
                self.unique_operators.insert("?.".to_string());
                self.total_operators += 1;
                opt_chain.visit_with(self);
            }
            Expr::Seq(seq) => {
                self.unique_operators.insert("seq".to_string());
                self.total_operators += 1;

                for expr in &seq.exprs {
                    expr.visit_with(self);
                }
            }
            Expr::Await(await_expr) => {
                self.unique_operators.insert("await".to_string());
                self.total_operators += 1;
                await_expr.arg.visit_with(self);
            }
            Expr::This(_) => {
                self.unique_operands.insert("this".to_string());
                self.total_operands += 1;
            }

            // The below cases don't contribute to operators/operands, but their children could
            Expr::JSXElement(jsx_element) => {
                // Traverse the opening element.
                jsx_element.opening.visit_with(self);

                // Traverse the children.
                for child in &jsx_element.children {
                    child.visit_with(self);
                }

                // Traverse the closing element, if present.
                if let Some(closing) = &jsx_element.closing {
                    closing.visit_with(self);
                }
            }
            Expr::JSXFragment(fragment) => {
                for child in &fragment.children {
                    child.visit_with(self);
                }
            }
            Expr::TaggedTpl(tagged_tpl) => {
                self.unique_operators.insert("TaggedTemplate".to_string());
                self.total_operators += 1; // Implicit tagged template operator

                tagged_tpl.tag.visit_with(self);
                tagged_tpl.tpl.visit_with(self);
            }
            _ => {
                expr.visit_children_with(self);
                debug!(
                    "visit_expr: Expression assumed to not count towards operators and operands: {:?}",
                    expr
                );
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
                self.unique_operands.insert(ident_str);
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

    fn visit_member_expr(&mut self, node: &MemberExpr) {
        if let MemberProp::Ident(_) = &node.prop {
            self.unique_operators.insert(".".to_string()); // Non-computed member access operator
            self.total_operators += 1;
        }

        node.obj.visit_with(self);
    }

    fn visit_ident(&mut self, node: &Ident) {
        self.unique_operands.insert(node.sym.to_string());
        self.total_operands += 1;
    }

    fn visit_tpl(&mut self, node: &Tpl) {
        self.unique_operators.insert("Template String".to_string());
        self.total_operators += 1;

        for element in &node.quasis {
            element.visit_with(self);
        }
        for expr in &node.exprs {
            expr.visit_with(self);
        }
    }

    fn visit_ts_type_operator(&mut self, node: &TsTypeOperator) {
        let operator = format!("{:?}", node.op);
        self.unique_operators.insert(operator);
        self.total_operators += 1;

        node.type_ann.visit_with(self);
    }

    fn visit_ts_mapped_type(&mut self, node: &TsMappedType) {
        self.unique_operators.insert("TsMappedType".to_string());
        self.total_operators += 2; // Implicit key in keyof and value in mapping type operators

        node.type_param.visit_with(self);
        if let Some(type_ann) = &node.type_ann {
            type_ann.visit_with(self);
        }
    }

    fn visit_ts_indexed_access_type(&mut self, node: &TsIndexedAccessType) {
        self.unique_operators
            .insert("TsIndexedAccessType".to_string());
        self.total_operators += 1; // Implicit indexed access operator

        node.obj_type.visit_with(self);
        node.index_type.visit_with(self);
    }

    fn visit_yield_expr(&mut self, node: &YieldExpr) {
        self.unique_operators.insert("yield".to_string());
        self.total_operators += 1;
        if let Some(arg) = &node.arg {
            arg.visit_with(self);
        }
    }

    fn visit_meta_prop_expr(&mut self, _node: &MetaPropExpr) {
        self.unique_operators.insert("new.target".to_string());
        self.total_operators += 1;
        // No children to visit
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt) {
        // Capture the return operator
        self.unique_operators.insert("return".to_string());
        self.total_operators += 1;

        // Visit the expression within the return statement, if present
        if let Some(expr) = &return_stmt.arg {
            expr.visit_with(self);
        }
    }

    fn visit_import_decl(&mut self, node: &ImportDecl) {
        self.unique_operators.insert("import".to_string());
        self.total_operators += 1;

        self.unique_operators.insert("from".to_string());
        self.total_operators += 1;

        node.visit_children_with(self);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(_) | Stmt::Return(_) | Stmt::Throw(_) | Stmt::Decl(_) => {
                self.unique_operators.insert(";".to_string());
                self.total_operators += 1;
            }
            _ => {}
        }
        stmt.visit_children_with(self);
    }
}

impl HalsteadMetrics {
    fn new(
        uniq_operators: usize,
        uniq_operands: usize,
        total_operators: usize,
        total_operands: usize,
    ) -> HalsteadMetrics {
        let program_length = uniq_operators + uniq_operands;
        let vocabulary_size = total_operators + total_operands;
        let volume = if vocabulary_size == 0 {
            0.0
        } else {
            (program_length as f64) * (vocabulary_size as f64).log2()
        };
        let difficulty = if total_operators == 0 || total_operands == 0 {
            0.0
        } else {
            ((total_operators / 2) as f64) * (uniq_operands as f64) / (total_operands as f64)
        };
        let effort = difficulty * volume;
        let time = effort / 18.0;
        let bugs = volume / 3000.0;

        HalsteadMetrics {
            uniq_operators,
            uniq_operands,
            total_operators,
            total_operands,
            program_length,
            vocabulary_size,
            volume,
            difficulty,
            effort,
            time,
            bugs,
        }
    }
}

pub fn analyze_module(module: &Module) -> HalsteadMetrics {
    let mut analyzer = AstAnalyzer::new();
    module.visit_with(&mut analyzer);

    // Useful for debugging (but very verbose):
    // println!("unique operators: {:?}", analyzer.unique_operators);
    // println!("unique operands: {:?}", analyzer.unique_operands);

    HalsteadMetrics::new(
        analyzer.unique_operators.len(),
        analyzer.unique_operands.len(),
        analyzer.total_operators,
        analyzer.total_operands,
    )
}
