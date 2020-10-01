use anyhow::{Context, Result};
use flexi_logger::{LogSpecBuilder, ReconfigurationHandle};

use crate::args::Args;

pub struct Logger {
    logger_reconfig: ReconfigurationHandle,
}

impl Logger {
    pub fn init() -> Result<Self> {
        let logger = flexi_logger::Logger::with_env()
            .start()
            .context("Logger initialization failed")?;

        Ok(Logger {
            logger_reconfig: logger,
        })
    }

    pub fn set_from_args(mut self, args: &Args) {
        if args.verbose {
            // Set the log level to debug if the verbose flag is set
            self.logger_reconfig.set_new_spec(
                LogSpecBuilder::new()
                    .default(log::LevelFilter::Debug)
                    .build(),
            );
        } else if args.log_level.is_some() {
            // Update the logger level if the cli flag specifies it
            // Overrides the verbose flag
            self.logger_reconfig.set_new_spec(
                LogSpecBuilder::new()
                    .default(args.log_level.unwrap())
                    .build(),
            );
        } else {
            self.logger_reconfig.set_new_spec(
                LogSpecBuilder::new()
                    .default(log::LevelFilter::Info)
                    .build(),
            );
        }
    }
}