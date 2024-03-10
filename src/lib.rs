//! LFA init - простейшая система инициализации для LFA. Она предназначена для изучения принципа работы
//! таких программ.
//!
//! ## Программы
//! - `/sbin/init` - система инициализации;
//! - `/sbin/service` - программа для управления сервисами;
//! - `/sbin/poweroff` - останавливает все запущенные сервисы и выключает систему;
//! - `/sbin/reboot` - останавливает все запущенные сервисы и перезагружает систему;
//!
//! ## Примитивы
//! LFA init использует два понятия: *сервис* и *уровень запуска*.
//!
//! Сервис - это TOML-конфиг, содержащий команды для запуска, перезапуска и остановки демонов и иных
//! программ. Упрощённый аналог загрузочных скриптов из SysVInit и OpenRC.
//!
//! Как и во многих других системах инициализации, в LFA init есть понятие уровней запуска (runlevel). Это
//! режим функционирования ОС или стадия её загрузки, подразумевающая доступность в этой системе каких-либо
//! возможностей и функций. В LFA init по умолчанию существуют следующие уровни запуска, хотя пользователь
//! может создавать и свои:
//!
//! - `rl0` - выключение системы. Не содержит никаких сервисов.
//! - `rl1` - загрузка системы в однопользовательском режиме. Смонтированы указанные в `/etc/fstab`
//!   разделы, а также ряд виртуальных файловых систем (см. `data/rl1/mount_*.toml`), в системе
//!   присутствует только пользователь `root`.
//! - `rl2` - многопользовательский режим **без** поддержки сети.
//! - `rl3` - многопользовательский режим с поддержкой сети. Используется по умолчанию в LFA.
//! - `rl4` - многопользовательский режим с поддержкой графики (X.org/Wayland). На данный момент не
//!   поддерживается.
//! - `rl5` - перезагрузка системы. Не содержит никаких сервисов.
//!
//! ## Конфигурационные файлы
//! Все конфигурационные файлы и сервисы содержатся в директории `/etc/init`. Там содержится главный
//! конфигурационный файл `/etc/init/init.toml`, содержащий сведения об уровнях запуска и порядке
//! загрузки ассоциированных с ними сервисов, файл `/etc/init/console.toml`, содержащий сведения
//! о консолях (TTY), которые нужно использовать и директории `/etc/init/rl[0..5]` с сервисами.

pub mod config;
pub mod consts;
pub mod service;
pub mod traits;

pub mod cmd;
pub mod msg;

use std::fmt::Display;

use service::OnError;
use traits::TomlConfig;

use config::Action;
use config::Config;

use service::ExecType;
use service::Service;

use consts::INIT_MASTER_CONF_FILE;

pub fn init_main() {
    println!("Starting init...");

    let conf = parse_master_conf();
    let final_rl = &conf.final_runlevel;
    let mut login_shell = "/bin/ash".to_string();
    let mut is_continue = true;

    for rl in &conf.runlevel {
        if !rl.r#use.unwrap_or(true) {
            println!("\nSkiping runlevel {}", &rl.dir);
            continue;
        }

        if &rl.dir == final_rl {
            if let Some(sh) = &rl.login_shell {
                login_shell = sh.to_string();
            }
            is_continue = false;
        }

        println!("\nSwitching to the {} runlevel...", &rl.dir);

        if let Action::run_services = rl.action.unwrap_or_default() {
            if let Some(services) = &rl.services {
                for service in services {
                    print!("  -> running {service} service...");

                    match Service::new(service, &rl.dir) {
                        Ok(service) => exec_service(service, ExecType::Start),
                        Err(why) => println!("ERROR: {why}"),
                    }
                }
            } else {
                eprintln!("init: error: services not found!");
            }
        }

        if !is_continue {
            break;
        }
    }

    println!("\nRunning login shell ({})...", &login_shell);
}

/*****************************************************************************
 *****************************************************************************/

fn parse_master_conf() -> Config {
    match Config::parse(INIT_MASTER_CONF_FILE) {
        Ok(conf) => conf,
        Err(why) => {
            eprintln!("init: {INIT_MASTER_CONF_FILE} parsing error: {why}");
            eprintln!("Using default configurations...");

            Config::default()
        }
    }
}

fn exec_service(service: Service, exec_type: ExecType) {
    match service.exec(exec_type) {
        Ok(run) => {
            if run == 0 {
                println!("ok");
            } else {
                on_error(
                    &service.init.on_error,
                    format!("non-zero return code ({run})"),
                );
            }
        }
        Err(why) => {
            on_error(&service.init.on_error, why);
        }
    }
}

fn on_error<D: Display>(err: &Option<OnError>, err_txt: D) {
    if let Some(err) = err {
        match err {
            OnError::ignore => println!("ERROR"),
            OnError::error => {
                eprintln!("ERROR: {err_txt}");
            }
            OnError::abort => {
                eprintln!("ERROR: {err_txt}");
                std::process::exit(1);
            }
        }
    } else {
        on_error(&Some(OnError::default()), err_txt);
    }
}
