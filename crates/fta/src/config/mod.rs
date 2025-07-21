use crate::structs::{FtaConfigOptional, FtaConfigResolved};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod tests;
mod integration_tests;

#[derive(Debug, Clone)]
pub struct ConfigError {
    message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigError! {}", self.message)
    }
}

impl From<FtaConfigOptional> for FtaConfigResolved {
    fn from(opt_config: FtaConfigOptional) -> Self {
        let default_config = get_default_config();
        FtaConfigResolved {
            extensions: opt_config.extensions.unwrap_or(default_config.extensions),
            exclude_filenames: opt_config
                .exclude_filenames
                .unwrap_or(default_config.exclude_filenames),
            exclude_directories: opt_config
                .exclude_directories
                .unwrap_or(default_config.exclude_directories),
            output_limit: opt_config
                .output_limit
                .unwrap_or(default_config.output_limit),
            score_cap: opt_config.score_cap.unwrap_or(default_config.score_cap),
            include_comments: opt_config
                .include_comments
                .unwrap_or(default_config.include_comments),
            exclude_under: opt_config
                .exclude_under
                .unwrap_or(default_config.exclude_under),
        }
    }
}

pub fn get_default_config() -> FtaConfigResolved {
    let default_config = FtaConfigResolved {
        extensions: vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
        ],
        exclude_filenames: vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string(),
        ],
        exclude_directories: vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string(),
        ],
        output_limit: 5000,
        score_cap: 1000,
        include_comments: false,
        exclude_under: 6,
    };

    default_config
}

pub fn read_config(
    config_path: String,
    path_specified_by_user: bool,
) -> Result<FtaConfigResolved, ConfigError> {
    let default_config = get_default_config();
    if Path::new(&config_path).exists() {
        let mut file = File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let provided_config: FtaConfigOptional = serde_json::from_str(&content).unwrap_or_default();

        // For extensions, filenames and exclude_directories,
        // user-provided values are added to the defaults.
        return Result::Ok(FtaConfigResolved {
            extensions: {
                let mut extensions = default_config.extensions;
                if let Some(mut provided) = provided_config.extensions {
                    extensions.append(&mut provided);
                }
                extensions
            },
            exclude_filenames: {
                let mut exclude_filenames = default_config.exclude_filenames;
                if let Some(mut provided) = provided_config.exclude_filenames {
                    exclude_filenames.append(&mut provided);
                }
                exclude_filenames
            },
            exclude_directories: {
                let mut exclude_directories = default_config.exclude_directories;
                if let Some(mut provided) = provided_config.exclude_directories {
                    exclude_directories.append(&mut provided);
                }
                exclude_directories
            },
            output_limit: provided_config
                .output_limit
                .unwrap_or(default_config.output_limit),
            score_cap: provided_config
                .score_cap
                .unwrap_or(default_config.score_cap),
            exclude_under: provided_config
                .exclude_under
                .unwrap_or(default_config.exclude_under),
            include_comments: provided_config
                .include_comments
                .unwrap_or(default_config.include_comments),
        });
    }

    if !path_specified_by_user {
        return Result::Ok(default_config);
    }

    Result::Err(ConfigError {
        message: format!("Config file not found at file path: {}", config_path),
    })
}
