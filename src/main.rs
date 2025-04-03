use anyhow::anyhow;
use clap::Parser;
use cli::enums::DataType;
use encoding::bitvec::BitVec;
use std::io::{Read, Write};
use utils::misc::{bits_to_bytestring, bits_to_string, bytestring_to_bitvec, string_to_bits};

mod cli;
mod encoding;
mod proto;
mod utils;

fn main() {
    let args = cli::Args::parse();
    utils::log::Logger::init(&args);

    match args.command {
        cli::Subcommands::Sender(sender_args) => {
            sender(sender_args, args.type_data).unwrap_or_else(|e| {
                std::io::stdout()
                    .write_all(b"ERR")
                    .expect("Failed to write to stdout");
                log::error!("Error: {}", e);
                std::process::exit(1);
            });
        }
        cli::Subcommands::Receiver(receiver_args) => {
            receiver(receiver_args, args.type_data).unwrap_or_else(|e| {
                log::error!("Error: {}", e);
                std::process::exit(1);
            });
        }
    }
}

fn sender(args: cli::SenderArgs, data_type: DataType) -> Result<(), anyhow::Error> {
    let data: BitVec = match data_type {
        DataType::Binary => BitVec::from_vec(bytestring_to_bitvec(&args.data)?),
        DataType::Text => BitVec::from_vec(string_to_bits(&args.data)),
    };

    let packet =
        proto::GUSProtocol::new(data).map_err(|e| anyhow!("Error creating GUSProtocol: {}", e))?;
    let encoded = packet
        .encode()
        .map_err(|e| anyhow!("Error encoding GUSProtocol: {}", e))?;

    std::io::stdout().write_all(encoded.as_slice())?;

    Ok(())
}

fn receiver(_: cli::ReceiverArgs, data_type: DataType) -> Result<(), anyhow::Error> {
    log::info!("Receiving data...");

    // receive from stdin until EOF
    let mut buffer = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read from stdin");

    if buffer.starts_with(b"ERR") {
        return Err(anyhow!("Sender errored, exiting..."));
    }

    log::debug!("Buffer:\n{:?}", buffer);

    // decode the packet
    let (packet, errored) = proto::GUSProtocol::decode(buffer)
        .map_err(|e| anyhow!("Error decoding GUSProtocol: {}", e))?;

    if errored {
        log::warn!("Correctable error detected in received data");
    }

    log::info!("Length (bits): {:?}", packet.data.len());

    let bitvec = packet.data.to_vec();
    let data = match data_type {
        DataType::Binary => bits_to_bytestring(&bitvec),
        DataType::Text => bits_to_string(&bitvec),
    };

    log::info!("Received data:\n{}", data);

    Ok(())
}
