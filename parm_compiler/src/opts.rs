use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Compiler {}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub package: Package,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Package {
    pub name: String,
}
