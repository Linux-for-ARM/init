//! Contains the declaration of all LFA init configuration files

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use nix::sys::reboot::reboot;
use nix::sys::reboot::RebootMode;

use crate::traits::TomlConfig;

/// Master config file of the LFA init
///
/// Contains a list of runlevels and the order in which services that
/// belong to each runlevel are loaded.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Last runlevel loaded
    pub final_runlevel: String,

    /// List of runlevels
    pub runlevel: Vec<Runlevel>,
}

/// Runlevel configuration
///
/// **Runlevel** - a certain stage of LFA loading, at which a number of strictly defined
/// components (services) are loaded. By default there are 6 runlevels in the system,
/// but the user can define his own if necessary.
#[derive(Deserialize, Serialize)]
pub struct Runlevel {
    /// The directory with the services of this runlevel and it's name
    pub dir: String,

    /// Whether to use this runlevel when booting/rebooting the system
    ///
    /// Default: `Some(true)`
    pub r#use: Option<bool>,

    /// Short description of this runlevel
    pub description: String,

    /// Action to be performed by the initialization system
    /// when switching to this runlevel. Default: `Action::run_service`
    pub action: Option<Action>,

    /// A list of services that are in the `dir` directory and will be
    /// started when going to this level of execution. Used only if
    /// `action = Action::run_services` or `action = None`.
    pub services: Option<Vec<String>>,

    /// Path to the login shell program that will be executed when all
    /// services of this runlevel have finished loading. The login shell
    /// will run only if this `runlevel` is specified in the
    /// `final_runlevel` parameter.
    pub login_shell: Option<String>,
}

/// Action to be performed by the initialization system
/// when switching to certain runlevel. Default: `Action::run_service`
#[derive(Deserialize, Serialize, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Action {
    /// Sequentially starts the services specified in the `Config.services` parameter
    run_services,

    /// Terminates running services and shuts down the system
    power_off,

    /// Terminates running services and reboots the system
    reboot,
}

impl Action {
    /// Performs a reboot or system shutdown
    pub fn take(&self) -> Result<()> {
        match self {
            Self::run_services => {
                eprintln!("/sbin/init: WARNING: No implementation is provided for the `run_services` action.");
            }
            Self::power_off => {
                reboot(RebootMode::RB_POWER_OFF)?;
            }
            Self::reboot => {
                reboot(RebootMode::RB_AUTOBOOT)?;
            }
        }

        Ok(())
    }
}

impl TomlConfig for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            final_runlevel: "rl1".to_string(),
            runlevel: vec![Runlevel::default()],
        }
    }
}

impl Default for Runlevel {
    fn default() -> Self {
        Self {
            dir: "rl1".to_string(),
            r#use: Some(true),
            description: "Single user mode".to_string(),
            action: Some(Action::default()),
            services: Some(vec![
                "mount_procfs".to_string(),
                "mount_sysfs".to_string(),
                "mount_tmpfs".to_string(),
                "mount_devpts".to_string(),
                "mount_devshm".to_string(),
            ]),
            login_shell: Some("/bin/login".to_string()),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::run_services
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_test() {
        let conf = Config::default();
        conf.write("test_config.toml").unwrap();
    }

    #[test]
    fn read_test() {
        let _ = Config::parse("data/init.toml").unwrap();
    }
}
