//! Parsing command line arguments for programs from LFA init

use clap::Parser;
use clap::Subcommand;

/// Command line arguments for `poweroff` and `reboot` programs
#[derive(Parser)]
pub struct Power {
    /// Force immediate power-off, halt, or reboot
    #[arg(long, short, default_value_t = false)]
    force: bool,
}

/// Command line arguments for the `service` program
#[derive(Parser)]
pub struct Service {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the specified service
    Start {
        service: String,
    },

    /// Stops the specified service
    Stop {
        service: String,
    },

    /// Restarts the specified service
    Restart {
        service: String,

        /// Specifies init to use first the command to stop,
        /// then the command to start the service to restart
        /// the service
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
}
