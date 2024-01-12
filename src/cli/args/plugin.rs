use std::fmt::Display;
use std::str::FromStr;

use clap::Error;
use miette::Result;

use crate::plugins::{unalias_plugin, PluginType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PluginArg {
    pub plugin_name: String,
    pub plugin_type: PluginType,
}

impl FromStr for PluginArg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (plugin_type, plugin) = s.split_once(':').unwrap_or(("external", s));
        let plugin_name = unalias_plugin(plugin).to_string();
        let plugin_type = match plugin_type {
            "external" => PluginType::External,
            "cargo" => PluginType::Cargo,
            _ => unimplemented!("Unknown plugin type: {}", plugin_type),
        };
        Ok(Self {
            plugin_name,
            plugin_type,
        })
    }
}

impl Display for PluginArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.plugin_name)
    }
}
