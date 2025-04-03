use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::Subcommands;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DataType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "binary")]
    Binary,
}
impl FromStr for DataType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(DataType::Text),
            "binary" => Ok(DataType::Binary),
            _ => Err(anyhow::anyhow!("Invalid data type: {}", s)),
        }
    }
}
impl ToString for DataType {
    fn to_string(&self) -> String {
        match self {
            DataType::Text => "text".to_string(),
            DataType::Binary => "binary".to_string(),
        }
    }
}

pub enum Mode {
    Receiver,
    Sender,
}
impl From<&Subcommands> for Mode {
    fn from(subcommand: &Subcommands) -> Self {
        match subcommand {
            Subcommands::Receiver(_) => Mode::Receiver,
            Subcommands::Sender(_) => Mode::Sender,
        }
    }
}
impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Receiver => "receiver".to_string(),
            Mode::Sender => "sender".to_string(),
        }
    }
}
