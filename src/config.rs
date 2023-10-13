use std::collections::HashSet;
use serde_derive::{Deserialize, Serialize};
use toml::{de, ser};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
    pub projects: Vec<Project>,
}

impl Config {
    pub fn new() -> Self {
        Self{
            projects: Vec::<Project>::new(),
        }
    }

    fn from_toml(document: &str) -> Result<Self, de::Error> {
        let cfg: Self = de::from_str(document)?;

        let mut seen_names = HashSet::<String>::new();

        if let Some(duplicate_name) = cfg.projects.iter().fold(None, |acc, proj| {
            if acc.is_some() {
                acc
            } else if seen_names.contains(&proj.name) {
                Some(proj.name.clone())
            } else {
                seen_names.insert(proj.name.clone());
                None
            }
        }) {
            Err(de::Error::custom("Placeholder"))
        } else {
            Ok(cfg)
        }
    }

    fn as_toml(&self) -> Result<String, ser::Error> {
        let document = ser::to_string_pretty(self)?;

        Ok(document)
    }

    fn project_name_collision(&self, name: &str) -> Option<String> {
        let name_lowercase = name.to_lowercase();

        self.projects
            .iter()
            .find(|proj| proj.name.to_lowercase() == name_lowercase)
            .map(|proj| proj.name.clone())
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Project {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_TOML: &str = "projects = []
";

    const POPULATED_TOML: &str = "[[projects]]
name = \"Test Project 1\"

[[projects]]
name = \"Test Project 2\"
";

    const INVALID_PROJECT_NAMES_TOML: &str = "[[projects]]
name = \"Test Project\"

[[projects]]
name = \"Test Project\"
";

    const MINIMAL_CONFIG: Config = Config {
        projects: vec![],
    };

    #[test]
    fn deserialize_minmal_config() {
        let config = Config::from_toml(MINIMAL_TOML).expect("Deserialization failure");

        assert_eq!(config, MINIMAL_CONFIG);
    }

    #[test]
    fn serialize_minimal_config() {
        let document = MINIMAL_CONFIG.as_toml().expect("Serialization failure");

        assert_eq!(document, MINIMAL_TOML);
    }

    #[test]
    fn deserialize_populated_config() {
        let config = Config::from_toml(POPULATED_TOML).expect("Deserialization failure");

        assert_eq!(config.projects, vec![
            Project{ name: "Test Project 1".to_string() },
            Project{ name: "Test Project 2".to_string() },
        ])

    }

    #[test]
    fn serialize_populated_config() {
        let config = Config {
            projects: vec![
                Project{ name: "Test Project 1".to_string() },
                Project{ name: "Test Project 2".to_string() },
            ],
        };
        let document = config.as_toml().expect("Serialization failure");

        assert_eq!(document, POPULATED_TOML);
    }

    #[test]
    fn deserialize_invalid_project_names_fails() {
        assert!(Config::from_toml(INVALID_PROJECT_NAMES_TOML).is_err());
    }
}