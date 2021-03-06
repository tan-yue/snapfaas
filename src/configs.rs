//! Controller and function configuration
//! In-memory data structures that represent controller configuration and
//! function configurations
use serde::Deserialize;
use serde_yaml;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use url::{Url, ParseError};
use log::{error, warn, info};
use crate::*;

const DEFAULT_CONTROLLER_CONFIG_URL: &str = "file://localhost/etc/snapfaas/default-conf.yaml";

#[derive(Deserialize, Debug)]
pub struct ControllerConfig {
    pub kernel_path: String,
    pub kernel_boot_args: String,
    pub runtimefs_dir: String,
    pub appfs_dir: String,
    pub snapshot_dir: String,
    pub function_config: String,
}

impl ControllerConfig {

    /// Create in-memory ControllerConfig struct from a YAML file
    /// TODO: Currently only supports file://localhost urls
    pub fn new(config_file: Option<&str>) -> ControllerConfig {
        let config_url = match config_file {
            None => DEFAULT_CONTROLLER_CONFIG_URL.to_string(),
            Some(path) => convert_fs_path_to_url(path),
        };
        info!("Using controller config: {}", config_url);

        return ControllerConfig::initialize(&config_url);
    }

    fn initialize(config_url: &str) -> ControllerConfig {
        if let Ok(config_url) = Url::parse(config_url) {
            let config_path = Path::new(config_url.path());
            // populate a ControllerConfig struct from the yaml file
            if let Ok(config) = File::open(config_path) {
                let config: serde_yaml::Result<ControllerConfig> = serde_yaml::from_reader(config);
                if let Ok(config) = config {
                    return config;
                } else {
                    warn!("Invalid YAML file");
                }
            } else {
                warn!("Invalid local path to config file");
            }

        } else {
            warn!("Invalid URL to config file")
        }

        return ControllerConfig {
            kernel_path: "".to_string(),
            kernel_boot_args: "".to_string(),
            runtimefs_dir: "".to_string(),
            appfs_dir: "".to_string(),
            function_config: "".to_string(),
            snapshot_dir: "".to_string(),
        };
    }

    pub fn set_kernel_path(&mut self, path: &str) {
        self.kernel_path = convert_fs_path_to_url(path);
    }

    pub fn set_kernel_boot_args(&mut self, args: &str) {
        self.kernel_boot_args= args.to_string();
    }

    pub fn get_runtimefs_base(&self) -> String {
        Url::parse(&self.runtimefs_dir).expect("invalid runtimefs dir from url").path().to_string()
    }

    pub fn get_appfs_base(&self) -> String {
        Url::parse(&self.appfs_dir).expect("invalid runtimefs dir from url").path().to_string()
    }

    pub fn get_snapshot_base(&self) -> String {
        Url::parse(&self.snapshot_dir).expect("invalid snapshot dir from url").path().to_string()
    }

}

#[derive(Debug, Deserialize, Clone)]
pub struct FunctionConfig {
    pub name: String,
    pub runtimefs: String,
    pub appfs: String,
    pub vcpus: u64,
    pub memory: usize,
    pub concurrency_limit: usize, // not in use
    pub load_dir: Option<String>,
}

