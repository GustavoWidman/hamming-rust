use colog::format::CologStyle;
use colored::Colorize;
use env_logger::Builder;
use log::Level;

use crate::cli;

struct CustomLevelTokens {
    pub mode: cli::enums::Mode,
}
impl From<cli::enums::Mode> for CustomLevelTokens {
    fn from(mode: cli::enums::Mode) -> Self {
        Self { mode }
    }
}

impl CologStyle for CustomLevelTokens {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }

    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "{}{}{} {}{}{} {}{}{}",
            "[".blue().bold(),
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S.%6f")
                .to_string()
                .white()
                .bold(),
            "]".blue().bold(),
            "[".blue().bold(),
            self.level_color(level, self.level_token(level)),
            "]".blue().bold(),
            "[".blue().bold(),
            self.mode.to_string().to_uppercase().purple().bold(),
            "]".blue().bold()
        )
    }
}

pub struct Logger;

impl Logger {
    pub fn init(args: &cli::Args) {
        Builder::new()
            .filter(None, args.verbosity)
            .format(colog::formatter(CustomLevelTokens::from(
                cli::enums::Mode::from(&args.command),
            )))
            .write_style(env_logger::WriteStyle::Always)
            .target(env_logger::Target::Pipe(Box::new(SplitWriter::new())))
            .init();
    }
}

// Custom writer to split logs between stdout and stderr
struct SplitWriter;

impl SplitWriter {
    fn new() -> Self {
        SplitWriter
    }
}

impl std::io::Write for SplitWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);

        // Check if this is an error message
        if s.contains("[ERR]") {
            std::io::stderr().write(buf)
        } else {
            std::io::stdout().write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()?;
        std::io::stderr().flush()
    }
}
