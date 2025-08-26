use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct OtelConfig {
    pub(crate) common: OtelCommon,
    pub(crate) log_format: LogFormat,
}

#[derive(Deserialize)]
pub(crate) struct OtelCommon {
    pub(crate) transport: OtelTransport,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub(crate) enum OtelTransport {
    #[allow(clippy::upper_case_acronyms)]
    HTTP(OtelTransportHttp),
}

#[derive(Deserialize)]
pub(crate) struct OtelTransportHttp {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) headers: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_log_format_parsing() {
        let yaml_config = r#"
log_format: json
common:
  transport:
    type: HTTP
    url: "http://localhost:4318"
"#;

        let config: OtelConfig = serde_yaml::from_str(yaml_config).expect("Failed to parse config");
        match config.log_format {
            LogFormat::Json => {}
            LogFormat::Text => panic!("Expected JSON format, got Text"),
        }
    }

    #[test]
    fn test_text_log_format_parsing() {
        let yaml_config = r#"
log_format: text
common:
  transport:
    type: HTTP
    url: "http://localhost:4318"
"#;

        let config: OtelConfig = serde_yaml::from_str(yaml_config).expect("Failed to parse config");
        match config.log_format {
            LogFormat::Text => {}
            LogFormat::Json => panic!("Expected Text format, got JSON"),
        }
    }
}
