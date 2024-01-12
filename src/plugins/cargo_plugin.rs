use crate::cache::CacheManager;
use crate::install_context::InstallContext;
use crate::plugins::external_plugin_cache::ExternalPluginCache;
use crate::plugins::mise_plugin_toml::MisePluginToml;
use crate::plugins::{Plugin, PluginName, PluginType};
use crate::{dirs, env};
use std::sync::Arc;

#[derive(Debug)]
pub struct CargoPlugin {
    pub name: String,
}
impl CargoPlugin {
    pub fn new(name: PluginName) -> Self {
        // let plugin_path = dirs::PLUGINS.join(&name);
        // let cache_path = dirs::CACHE.join(&name);
        Self {
            // script_man: crate::plugins::external_plugin::build_script_man(&name, &plugin_path),
            // downloads_path: dirs::DOWNLOADS.join(&name),
            // installs_path: dirs::INSTALLS.join(&name),
            // cache: ExternalPluginCache::default(),
            // remote_version_cache: CacheManager::new(cache_path.join("remote_versions.msgpack.z"))
            //     .with_fresh_duration(*env::MISE_FETCH_REMOTE_VERSIONS_CACHE)
            //     .with_fresh_file(plugin_path.clone())
            //     .with_fresh_file(plugin_path.join("bin/list-all")),
            // latest_stable_cache: CacheManager::new(cache_path.join("latest_stable.msgpack.z"))
            //     .with_fresh_duration(*env::MISE_FETCH_REMOTE_VERSIONS_CACHE)
            //     .with_fresh_file(plugin_path.clone())
            //     .with_fresh_file(plugin_path.join("bin/latest-stable")),
            // alias_cache: CacheManager::new(cache_path.join("aliases.msgpack.z"))
            //     .with_fresh_file(plugin_path.clone())
            //     .with_fresh_file(plugin_path.join("bin/list-aliases")),
            // legacy_filename_cache: CacheManager::new(cache_path.join("legacy_filenames.msgpack.z"))
            //     .with_fresh_file(plugin_path.clone())
            //     .with_fresh_file(plugin_path.join("bin/list-legacy-filenames")),
            // plugin_path,
            // cache_path,
            // repo_url: None,
            // toml,
            name,
        }
    }
    pub fn newa(name: PluginName) -> Arc<dyn Plugin> {
        Arc::new(Self::new(name))
    }
}

impl Plugin for CargoPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> PluginType {
        PluginType::Cargo
    }

    fn list_remote_versions(&self) -> miette::Result<Vec<String>> {
        todo!()
    }

    fn install_version_impl(&self, ctx: &InstallContext) -> miette::Result<()> {
        todo!()
    }
}
