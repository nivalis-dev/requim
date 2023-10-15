use crate::projects::ProjectDataError::ProjectNameConflict;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::{fs, io};
use toml::{de, ser};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ProjectData {
    pub projects: Vec<Project>,
}

impl ProjectData {
    pub fn read(data_dir: &Path) -> Result<Self, ProjectDataError> {
        let project_file_path = data_dir.join("Projects.toml");
        if project_file_path.exists() {
            let document = fs::read_to_string(data_dir.join("Projects.toml"))?;

            Self::from_toml(&document)
        } else {
            Ok(Self { projects: vec![] })
        }
    }

    fn from_toml(document: &str) -> Result<Self, ProjectDataError> {
        let project_data: Self = de::from_str(document)?;

        let mut seen_names = HashMap::<String, String>::new();

        if let Some(duplicate_name) = project_data.projects.iter().fold(None, |acc, proj| {
            if acc.is_some() {
                acc
            } else if let Some(name) = seen_names.get(&proj.name.to_lowercase()) {
                Some(name.clone())
            } else {
                seen_names.insert(proj.name.to_lowercase(), proj.name.clone());
                None
            }
        }) {
            Err(ProjectDataError::ProjectNameConflict(
                duplicate_name.clone(),
            ))
        } else {
            Ok(project_data)
        }
    }

    fn as_toml(&self) -> Result<String, ProjectDataError> {
        let document = ser::to_string_pretty(self)?;

        Ok(document)
    }

    pub fn add(&self, project_name: &str) -> Result<ProjectData, ProjectDataError> {
        if let Some(project_name) = self.project_name_collision(project_name) {
            Err(ProjectNameConflict(project_name))
        } else {
            let mut new_projects = self.projects.clone();
            new_projects.push(Project {
                name: project_name.to_string(),
            });

            Ok(ProjectData {
                projects: new_projects,
            })
        }
    }

    fn project_name_collision(&self, name: &str) -> Option<String> {
        let name_lowercase = name.to_lowercase();

        self.projects
            .iter()
            .find(|proj| proj.name.to_lowercase() == name_lowercase)
            .map(|proj| proj.name.clone())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Display)]
pub enum ProjectDataError {
    #[display(fmt = "Failed to deserialize config: {}", _0)]
    DeserializationError(de::Error),
    #[display(fmt = "failed to read project data ({})", _0)]
    IOError(io::Error),
    #[display(fmt = "a project named \"{}\" already exists", _0)]
    ProjectNameConflict(String),
    #[display(fmt = "Failed to serialize config: {}", _0)]
    SerializationError(ser::Error),
}

impl Error for ProjectDataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProjectDataError::DeserializationError(err) => Some(err),
            ProjectDataError::SerializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ProjectDataError {
    fn from(err: io::Error) -> Self {
        ProjectDataError::IOError(err)
    }
}

impl From<de::Error> for ProjectDataError {
    fn from(err: de::Error) -> Self {
        ProjectDataError::DeserializationError(err)
    }
}

impl From<ser::Error> for ProjectDataError {
    fn from(err: ser::Error) -> Self {
        ProjectDataError::SerializationError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::ProjectDataError::ProjectNameConflict;

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
name = \"test project\"
";

    const MINIMAL_CONFIG: ProjectData = ProjectData { projects: vec![] };

    #[test]
    fn deserialize_minmal_config() {
        let config = ProjectData::from_toml(MINIMAL_TOML).expect("Deserialization failure");

        assert_eq!(config, MINIMAL_CONFIG);
    }

    #[test]
    fn serialize_minimal_config() {
        let document = MINIMAL_CONFIG.as_toml().expect("Serialization failure");

        assert_eq!(document, MINIMAL_TOML);
    }

    #[test]
    fn deserialize_populated_config() {
        let config = ProjectData::from_toml(POPULATED_TOML).expect("Deserialization failure");

        assert_eq!(
            config.projects,
            vec![
                Project {
                    name: "Test Project 1".to_string()
                },
                Project {
                    name: "Test Project 2".to_string()
                },
            ]
        )
    }

    #[test]
    fn serialize_populated_config() {
        let config = ProjectData {
            projects: vec![
                Project {
                    name: "Test Project 1".to_string(),
                },
                Project {
                    name: "Test Project 2".to_string(),
                },
            ],
        };
        let document = config.as_toml().expect("Serialization failure");

        assert_eq!(document, POPULATED_TOML);
    }

    #[test]
    fn deserialize_invalid_project_names_fails() {
        let err = ProjectData::from_toml(INVALID_PROJECT_NAMES_TOML).expect_err("Did not fail");

        assert_eq!(err, ProjectNameConflict("Test Project".to_string()))
    }

    #[test]
    fn add_project() {
        let project_data = ProjectData { projects: vec![] };

        assert_eq!(
            project_data.add("Test Project"),
            Ok(ProjectData {
                projects: vec![Project {
                    name: "Test Project".to_string()
                }]
            })
        )
    }

    #[test]
    fn add_project_with_existing_name_fails() {
        let project_data = ProjectData {
            projects: vec![Project {
                name: "Test Project".to_string(),
            }],
        };

        assert_eq!(
            project_data.add("test project"),
            Err(ProjectNameConflict("Test Project".to_string())),
        )
    }
}
