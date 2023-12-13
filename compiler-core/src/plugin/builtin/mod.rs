//! Built-in plugins
//!
//! Built-in plugins are implemented in Rust and directly included in the compiler.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    Metrics,
    Link,
    Variables,
    // Compat,
    BotwAbilityUnstable,
}
