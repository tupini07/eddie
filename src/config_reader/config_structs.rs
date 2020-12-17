#[derive(Debug)]
pub struct AppConfig {
    pub eddie_config: EddieConfig,
    pub config_tree: ConfigNode,
}

#[derive(Debug)]
pub struct EddieConfig {
    // config for the actual execution of Eddie
}

#[derive(Debug)]
/// This is a structure which represents a single item in the config tree loaded from the TOML files.
/// See the [crate::config_reader].
pub struct ConfigNode {
    pub name: String,
    pub description: String,
    pub command: String,
    pub children: Option<Vec<ConfigNode>>,
    pub opens_external: bool,
}

impl ConfigNode {
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn _is_valid(&self) -> bool {
        self.children.is_some() || !self.command.is_empty()
    }
}
