use crate::structs::{FileData, FtaConfig};
use ignore::DirEntry;

pub fn walk_and_analyze_files<I, P, V>(
    entries: I,
    repo_path: &String,
    config: &FtaConfig,
    process_entry: P,
    is_valid: V,
) -> Vec<FileData>
where
    I: Iterator<Item = Result<DirEntry, ignore::Error>>,
    P: Fn(DirEntry, &String, &FtaConfig) -> Option<Vec<FileData>>,
    V: Fn(&String, &DirEntry, &FtaConfig) -> bool,
{
    let mut file_data_list: Vec<FileData> = Vec::new();

    entries
        // 1. Were we able to successfully read the DirEntry & is it a file?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        // 2. Is the file considered valid according to our basic requirements plus user configuration?
        .filter(|entry| is_valid(repo_path, &entry, config))
        // 3. Analyze each file
        .filter_map(|entry| process_entry(entry, repo_path, config))
        // 4. Return a list of analyzed files
        .for_each(|data_vec| file_data_list.extend(data_vec));

    file_data_list
}
