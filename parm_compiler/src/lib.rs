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
    pub bin: Bin,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Bin {
    pub entry: String,
}
