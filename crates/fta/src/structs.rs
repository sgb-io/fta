use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct FtaConfig {
    pub extensions: Option<Vec<String>>,
    pub exclude_filenames: Option<Vec<String>>,
    pub exclude_directories: Option<Vec<String>>,
    pub output_limit: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HalsteadMetrics {
    pub uniq_operators: usize,  // number of unique operators
    pub uniq_operands: usize,   // number of unique operands
    pub total_operators: usize, // total number of operators
    pub total_operands: usize,  // total number of operands
    pub program_length: usize,
    pub vocabulary_size: usize,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
    pub time: f64,
    pub bugs: f64,
}
