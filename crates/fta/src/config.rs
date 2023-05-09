use crate::structs::FtaConfig;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_config(config_path: &str) -> FtaConfig {
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
    };

    if Path::new(config_path).exists() {
        let mut file = File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let provided_config: FtaConfig = serde_json::from_str(&content).unwrap_or_default();

        FtaConfig {
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
            exclude_directories: provided_config
                .exclude_directories
                .or(default_config.exclude_directories),
            output_limit: provided_config.output_limit.or(default_config.output_limit),
        }
    } else {
        default_config
    }
}
