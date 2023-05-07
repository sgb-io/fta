use std::collections::HashMap;
use swc_ecma_ast::{Decl, Ident, Module, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

#[derive(Default)]
struct HalsteadMetrics {
    n1: usize,
    n2: usize,
    n1_vocab: HashMap<String, usize>,
    n2_vocab: HashMap<String, usize>,
}

impl HalsteadMetrics {
    fn calculate_metrics(&self) -> (usize, usize, f64, f64, f64) {
        let n1 = self.n1 as f64;
        let n2 = self.n2 as f64;
        let n1_vocab = self.n1_vocab.len() as f64;
        let n2_vocab = self.n2_vocab.len() as f64;

        let program_length = (n1 + n2) as usize;
        let program_vocabulary = (n1_vocab + n2_vocab) as usize;
        let estimated_length =
            n1_vocab * (1.0_f64).max(n1_vocab.log2()) + n2_vocab * (1.0_f64).max(n2_vocab.log2());

        let volume = (n1 + n2) * (1.0_f64).max((n1_vocab + n2_vocab).log2());
        let difficulty = (n1_vocab / 2.0) * (n1 / (n2_vocab.max(1.0)));

        (
            program_length,
            program_vocabulary,
            estimated_length,
            volume,
            difficulty,
        )
    }
}

struct HalsteadVisitor {
    metrics: HalsteadMetrics,
}

impl HalsteadVisitor {
    fn new() -> Self {
        Self {
            metrics: HalsteadMetrics::default(),
        }
    }
}

impl VisitMut for HalsteadVisitor {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, n: &mut Ident) {
        self.metrics.n2 += 1;
        *self.metrics.n2_vocab.entry(n.sym.to_string()).or_insert(0) += 1;
    }

    fn visit_mut_module_item(&mut self, item: &mut ModuleItem) {
        match item {
            ModuleItem::ModuleDecl(module_decl) => match module_decl {
                ModuleDecl::Import(_) | ModuleDecl::ExportDecl(_) | ModuleDecl::ExportNamed(_) => {
                    self.metrics.n1 += 1;
                }
                _ => (),
            },
            ModuleItem::Stmt(stmt) => match stmt {
                Stmt::Decl(decl) => match decl {
                    Decl::Var(_) | Decl::Fn(_) | Decl::Class(_) => {
                        self.metrics.n1 += 1;
                    }
                    _ => (),
                },
                Stmt::If(_)
                | Stmt::Switch(_)
                | Stmt::While(_)
                | Stmt::DoWhile(_)
                | Stmt::For(_)
                | Stmt::ForIn(_)
                | Stmt::ForOf(_)
                | Stmt::Throw(_)
                | Stmt::Try(_)
                | Stmt::Return(_)
                | Stmt::Break(_)
                | Stmt::Continue(_) => {
                    self.metrics.n1 += 1;
                }
                _ => (),
            },
        }
        item.visit_mut_children_with(self);
    }
}

pub fn halstead(module: Module) {
    let mut visitor = HalsteadVisitor::new();
    let mut program = module.clone().visit_mut_with(&mut visitor);

    let (program_length, program_vocabulary, estimated_length, volume, difficulty) =
        visitor.metrics.calculate_metrics();

    println!("Halstead Metrics:");
    println!("Program Length: {}", program_length);
    println!("Program Vocabulary: {}", program_vocabulary);
    println!("Estimated Length: {:.2}", estimated_length);
    println!("Volume: {:.2}", volume);
    println!("Difficulty: {:.2}", difficulty);
}
