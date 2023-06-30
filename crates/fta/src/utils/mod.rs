use globset::{Glob, GlobSetBuilder};
use ignore::DirEntry;
use crate::structs::{FtaConfig};
use log::warn;

mod tests;

pub fn is_excluded_filename(file_name: &str, patterns: &[String]) -> bool {
  let mut builder = GlobSetBuilder::new();

  for pattern in patterns {
      let glob = Glob::new(pattern).unwrap();
      builder.add(glob);
  }

  let glob_set = builder.build().unwrap();

  glob_set.is_match(file_name)
}

pub fn is_valid_file(repo_path: &String, entry: &DirEntry, config: &FtaConfig) -> bool {
  let file_name = entry.path().file_name().unwrap().to_str().unwrap();
  let relative_path = entry
      .path()
      .strip_prefix(repo_path)
      .unwrap()
      .to_str()
      .unwrap();

  let valid_extension = config
      .extensions
      .as_ref()
      .map_or(true, |exts| exts.iter().any(|ext| file_name.ends_with(ext)));

  let is_excluded_filename = config
      .exclude_filenames
      .as_ref()
      .map_or(false, |patterns| is_excluded_filename(file_name, patterns));

  let is_excluded_directory = config.exclude_directories.as_ref().map_or(false, |dirs| {
      dirs.iter().any(|dir| relative_path.starts_with(dir))
  });

  valid_extension && !is_excluded_filename && !is_excluded_directory
}

pub fn warn_about_language(file_name: &str, use_tsx: bool) {
    let tsx_name = if use_tsx { "j/tsx" } else { "non-j/tsx" };
    let opposite_tsx_name = if use_tsx { "non-j/tsx" } else { "j/tsx" };

    warn!(
        "File {} was interpreted as {} but seems to actually be {}. The file extension may be incorrect.",
        file_name,
        tsx_name,
        opposite_tsx_name
    );
}

pub fn check_score_cap_breach(
    file_name: String,
    fta_score: f64,
    score_cap: std::option::Option<usize>,
) {
    // Exit 1 if score_cap breached
    if let Some(score_cap) = score_cap {
        if fta_score > score_cap as f64 {
            eprintln!(
                "File {} has a score of {}, which is beyond the score cap of {}, exiting.",
                file_name, fta_score, score_cap
            );
            std::process::exit(1);
        }
    }
}

pub fn get_assessment(score: f64) -> String {
    if score > 60.0 {
        "Needs improvement".to_string()
    } else if score > 50.0 {
        "Could be better".to_string()
    } else {
        "OK".to_string()
    }
}