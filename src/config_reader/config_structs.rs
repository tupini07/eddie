#[derive(Debug)]
pub struct EddieConfig {
    // config for the actual execution of Eddie
}

#[derive(Debug)]
pub struct AppConfig {
    pub eddie_config: EddieConfig,
    pub config_tree: ConfigNode,
}

#[derive(Debug)]
pub struct ConfigNode {
    pub name: String,
    pub description: String,
    pub command: String,
    pub children: Option<Vec<ConfigNode>>,
}

impl ConfigNode {
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn is_valid(&self) -> bool {
        self.children.is_some() || !self.command.is_empty()
    }
}