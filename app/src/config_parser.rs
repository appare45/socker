use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "ociVersion")]
    pub oci_version: String,
}

impl TryFrom<&str> for Config {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let config: Config = serde_json::from_str(value).context("Failed to parse config file")?;
        Ok(config)
    }
}

mod test {
    use super::Config;
    #[test]
    fn parser() {
        let data = r#"{
            "ociVersion": "1.0.0"
        }"#;
        let config = Config::try_from(data).unwrap();
        assert_eq!(config.oci_version, "1.0.0");
    }
}
