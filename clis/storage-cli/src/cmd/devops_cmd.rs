use std::fmt::Display;

use clap::{Parser, ValueEnum};
use serde::Deserialize;

use super::FileOperation;

#[derive(Debug, Default, Parser)]
#[command(name = "devops-cli")]
#[command(version, about = "DevOps command tool", long_about = None)]
pub struct DevopsCmd {
    #[arg(short, long, default_value_t = StorageSource::Obs)]
    pub service: StorageSource,

    #[arg(short, long)]
    pub bucket: Option<String>,

    #[arg(long)]
    pub ak: Option<String>,

    #[arg(long)]
    pub sk: Option<String>,

    #[arg(short('f'), long)]
    pub config_file: Option<String>,

    #[command(subcommand)]
    pub file_op: Option<FileOperation>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, ValueEnum, Deserialize)]
pub enum StorageSource {
    /// 华为云 OBS
    #[default]
    Obs,
    /// 阿里云 OSS
    Oss,
}

impl Display for StorageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageSource::Obs => f.write_str("obs"),
            StorageSource::Oss => f.write_str("oss"),
        }
    }
}
