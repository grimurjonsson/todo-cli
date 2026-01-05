pub mod generators;
pub mod subprocess;

use crate::todo::TodoItem;
use anyhow::Result;
use std::collections::HashMap;

pub trait TodoGenerator: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn check_available(&self) -> Result<(), String>;
    fn generate(&self, input: &str) -> Result<Vec<TodoItem>>;
}

pub struct PluginRegistry {
    generators: HashMap<String, Box<dyn TodoGenerator>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            generators: HashMap::new(),
        };
        registry.register_builtin_generators();
        registry
    }

    fn register_builtin_generators(&mut self) {
        self.register(Box::new(generators::JiraClaudeGenerator::new()));
    }

    pub fn register(&mut self, generator: Box<dyn TodoGenerator>) {
        self.generators.insert(generator.name().to_string(), generator);
    }

    pub fn get(&self, name: &str) -> Option<&dyn TodoGenerator> {
        self.generators.get(name).map(|g| g.as_ref())
    }

    pub fn list(&self) -> Vec<GeneratorInfo> {
        self.generators
            .values()
            .map(|g| GeneratorInfo {
                name: g.name().to_string(),
                description: g.description().to_string(),
                available: g.check_available().is_ok(),
                unavailable_reason: g.check_available().err(),
            })
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct GeneratorInfo {
    pub name: String,
    pub description: String,
    pub available: bool,
    pub unavailable_reason: Option<String>,
}
