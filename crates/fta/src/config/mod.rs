use crate::structs::FtaConfig;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod tests;

#[derive(Debug, Clone)]
pub struct ConfigError {
    message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigError! {}", self.message)
    }
}

pub fn read_config(
    config_path: String,
    path_specified_by_user: bool,
) -> Result<FtaConfig, ConfigError> {
    let default_config = FtaConfig {
        extensions: Some(vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
        ]),
        exclude_filenames: Some(vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string(),
        ]),
        exclude_directories: Some(vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string(),
        ]),
        output_limit: Some(5000),
        score_cap: Some(1000),
        include_comments: Some(false),
    };

    if Path::new(&config_path).exists() {
        let mut file = File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let provided_config: FtaConfig = serde_json::from_str(&content).unwrap_or_default();

        return Result::Ok(FtaConfig {
            extensions: {
                let mut extensions = default_config.extensions.unwrap();
                if let Some(mut provided) = provided_config.extensions {
                    extensions.append(&mut provided);
                }
                Some(extensions)
            },
            exclude_filenames: {
                let mut exclude_filenames = default_config.exclude_filenames.unwrap();
                if let Some(mut provided) = provided_config.exclude_filenames {
                    exclude_filenames.append(&mut provided);
                }
                Some(exclude_filenames)
            },
            exclude_directories: {
                let mut exclude_directories = default_config.exclude_directories.unwrap();
                if let Some(mut provided) = provided_config.exclude_directories {
                    exclude_directories.append(&mut provided);
                }
                Some(exclude_directories)
            },
            output_limit: provided_config.output_limit.or(default_config.output_limit),
            score_cap: provided_config.score_cap.or(default_config.score_cap),
            include_comments: provided_config
                .include_comments
                .or(default_config.include_comments),
        });
    }

    if !path_specified_by_user {
        return Result::Ok(default_config);
    }

    Result::Err(ConfigError {
        message: format!("Config file not found at file path: {}", config_path),
    })
}
