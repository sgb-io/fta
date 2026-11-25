use crate::structs::{FileData, FtaConfigResolved};
use ignore::DirEntry;

pub fn walk_and_analyze_files<I, P, V>(
    entries: I,
    repo_path: &String,
    config: &FtaConfigResolved,
    process_entry: P,
    is_valid: V,
) -> (Vec<FileData>, usize)
where
    I: Iterator<Item = Result<DirEntry, ignore::Error>>,
    P: Fn(DirEntry, &String, &FtaConfigResolved) -> Option<FileData>,
    V: Fn(&String, &DirEntry, &FtaConfigResolved) -> bool,
{
    let mut analyzed_count = 0;
    let file_data_list = entries
        // 1. Were we able to successfully read the DirEntry & is it a file?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        // 2. Is the file considered valid according to our basic requirements plus user configuration?
        .filter(|entry| is_valid(repo_path, &entry, config))
        // 3. Analyze each file
        .filter_map(|entry| process_entry(entry, repo_path, config))
        .inspect(|_| analyzed_count += 1)
        .filter(|data| data.line_count > config.exclude_under)
        .collect();

    (file_data_list, analyzed_count)
}
