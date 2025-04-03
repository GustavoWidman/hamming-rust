pub mod enums;

use clap::{Parser, Subcommand};
use enums::DataType;
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Args {
    /// Sets the logger's verbosity level
    #[arg(long, short, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,

    /// The type of data to send
    #[arg(long, short, default_value_t = DataType::Binary, id="type")]
    pub type_data: DataType,

    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Receiver(ReceiverArgs),
    Sender(SenderArgs),
}
#[derive(Parser, Debug)]
pub struct SenderArgs {
    /// The data to encode and send to the receiver
    #[arg(long, short)]
    pub data: String,
}

#[derive(Parser, Debug)]
pub struct ReceiverArgs {}
