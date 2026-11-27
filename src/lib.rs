#![forbid(unsafe_code)]
//! GenOffice document editor library
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub status: ComponentStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Active,
    Inactive,
    Error(String),
}

#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("component not found: {0}")]
    NotFound(String),
    #[error("invalid component id: {0}")]
    InvalidId(String),
    #[error("duplicate component: {0}")]
    Duplicate(String),
}

pub struct ComponentRegistry {
    components: Vec<Component>,
}

impl ComponentRegistry {
    pub fn new() -> Self { Self { components: Vec::new() } }

    pub fn register(&mut self, id: impl Into<String>, name: impl Into<String>) -> Result<(), ComponentError> {
        let id = id.into();
        if id.is_empty() { return Err(ComponentError::InvalidId(id)); }
        if self.components.iter().any(|c| c.id == id) {
            return Err(ComponentError::Duplicate(id));
        }
        self.components.push(Component { id, name: name.into(), enabled: true, status: ComponentStatus::Inactive });
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.id == id)
    }

    pub fn activate(&mut self, id: &str) -> Result<(), ComponentError> {
        self.components.iter_mut()
            .find(|c| c.id == id)
            .map(|c| c.status = ComponentStatus::Active)
            .ok_or_else(|| ComponentError::NotFound(id.to_string()))
    }

    pub fn list(&self) -> &[Component] { &self.components }
}

impl Default for ComponentRegistry { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get_aaa() {
        // Arrange
        let mut reg = ComponentRegistry::new();
        // Act
        reg.register("comp-1", "Component One").unwrap();
        let c = reg.get("comp-1");
        // Assert
        assert!(c.is_some());
        assert_eq!(c.unwrap().name, "Component One");
    }

    #[test]
    fn test_activate_status_lifecycle_aaa() {
        // Arrange
        let mut reg = ComponentRegistry::new();
        reg.register("comp-2", "Component Two").unwrap();
        // Act
        reg.activate("comp-2").unwrap();
        let c = reg.get("comp-2").unwrap();
        // Assert
        assert_eq!(c.status, ComponentStatus::Active);
    }

    #[test]
    fn test_duplicate_registration_aaa() {
        // Arrange
        let mut reg = ComponentRegistry::new();
        reg.register("comp-3", "Component Three").unwrap();
        // Act
        let result = reg.register("comp-3", "Component Three Again");
        // Assert
        assert!(matches!(result, Err(ComponentError::Duplicate(_))));
    }

    #[test]
    fn test_invalid_empty_id_aaa() {
        // Arrange
        let mut reg = ComponentRegistry::new();
        // Act
        let result = reg.register("", "Empty");
        // Assert
        assert!(matches!(result, Err(ComponentError::InvalidId(_))));
    }
}
