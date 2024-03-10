//! Module for working with services
//!
//! Service is a minimal unit in lfa_init, designed
//! to start and stop the services

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::process::Command;

use crate::consts::CONF_DIR;
use crate::traits::TomlConfig;

#[derive(Deserialize, Serialize)]
pub struct Service {
    pub service: ServiceSection,
    pub init: InitSection,
}

#[derive(Deserialize, Serialize)]
pub struct ServiceSection {
    pub start: Option<Vec<String>>,
    pub stop: Option<Vec<String>>,
    pub restart: Option<Vec<String>>,
    pub can_restart: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct InitSection {
    pub description: String,
    pub on_error: Option<OnError>,
}

#[derive(Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum OnError {
    ignore,
    error,
    abort,
}

struct Cmd {
    prog: String,
    args: Option<String>,
}

pub enum ExecType {
    Start,
    Restart,
    Stop,
}

impl Service {
    pub fn new(srv_name: &str, rl: &str) -> Result<Self> {
        // функция получает имя сервиса без расширения `*.toml`, поэтому
        // его нужно добавить
        let srv_name = format!("{srv_name}.toml");

        let srv_pth = Path::new(CONF_DIR).join(rl).join(&srv_name);
        Service::parse(srv_pth)
    }

    fn run_cmd(&self, command: &Cmd) -> Result<i32> {
        let code: i32;
        if let Some(args) = &command.args {
            let cmd = Command::new(&command.prog).arg(args).status()?.code();
            code = cmd.unwrap_or(0);
        } else {
            let cmd = Command::new(&command.prog).status()?.code();
            code = cmd.unwrap_or(0);
        }

        Ok(code)
    }

    fn parse_cmd(&self, cmd: &str) -> Cmd {
        let command = cmd.split_once(" ");
        match command {
            Some(c) => Cmd {
                prog: c.0.to_string(),
                args: Some(c.1.to_string()),
            },
            None => Cmd {
                prog: cmd.to_string(),
                args: None,
            },
        }
    }

    pub fn exec(&self, exec_type: ExecType) -> Result<i32> {
        let cmd = match exec_type {
            ExecType::Start => &self.service.start,
            ExecType::Restart => &self.service.restart,
            ExecType::Stop => &self.service.stop,
        };

        match cmd {
            Some(cmd) => {
                let mut run = 0;
                for c in cmd {
                    let c = self.parse_cmd(c);
                    run = self.run_cmd(&c)?;

                    // если одна команда завершилась с ошибкой,
                    // то прервать выполнение остальных команд
                    // и вернуть код ошибки
                    if run != 0 {
                        return Ok(run);
                    }
                }
                Ok(run)
            }
            None => Ok(0),
        }
    }
}

impl TomlConfig for Service {}

impl Default for Service {
    fn default() -> Self {
        Self {
            service: ServiceSection {
                start: None,
                stop: None,
                restart: None,
                can_restart: Some(false),
            },
            init: InitSection {
                description: "[example] Default service".to_string(),
                on_error: Some(OnError::default()),
            },
        }
    }
}

impl Default for OnError {
    fn default() -> Self {
        Self::error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn write_test() {
        let mut service = Service::default();
        service.service.start = Some(vec!["/bin/mount aa /bb".to_string()]);
        service.service.stop = Some(vec![
            "/bin/umount /bb".to_string(),
            "/bin/killall init".to_string(),
        ]);

        let data = toml::to_string(&service).unwrap();
        fs::write("test_service.toml", data).unwrap();
    }
}
