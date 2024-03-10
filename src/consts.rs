/// Directory with initialization system configuration files, services
/// and startup levels
pub const CONF_DIR: &str = "/etc/init/";

/// Path to the master LFA init configuration file on the file system,
/// containing startup level declarations and associated services
pub const INIT_MASTER_CONF_FILE: &str = "/etc/init/init.toml";

/// Path to the config with information about loaded services
pub const LOADED_SERVICES_CONF_FILE: &str = "/var/log/ld_srv.toml.log";
