mod complexity;
mod halstead;
mod parse_module;

use std::env;
use std::fs;

#[derive(Debug)]
struct HalsteadMetrics {
    uniq_operators: usize,  // number of unique operators
    uniq_operands: usize,   // number of unique operands
    total_operators: usize, // total number of operators
    total_operands: usize,  // total number of operands
    program_length: usize,
    vocabulary_size: usize,
    volume: f64,
    difficulty: f64,
    effort: f64,
    time: f64,
    bugs: f64,
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
        let volume = (program_length as f64) * (vocabulary_size as f64).log2();
        let difficulty =
            ((total_operators / 2) as f64) * (uniq_operands as f64) / (total_operands as f64);
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

fn main() {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a TypeScript file to analyze.");
    }
    let file_path = &args[1];

    // Read the file
    let source_code = fs::read_to_string(file_path).unwrap();

    let module = parse_module::parse_module(&source_code);
    let cyclo = complexity::cyclomatic_complexity(module.clone());
    let metrics = halstead::analyze_module(&module);

    // Print the results
    println!("Halstead Metrics for {}: {:?}", file_path, metrics);
    println!("Cyclomatic: {:?}", cyclo);
}
