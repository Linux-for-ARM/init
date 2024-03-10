//! Formatting and outputting messages to stdout/stderr and logging

use std::env::args;
use std::fmt::Display;

pub struct Msg {
    indent: usize,
    prefix: String,
    msg_type: MsgType,
}

pub enum MsgType {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Emerg = 4,
}

impl Msg {
    /// Creates a new instance of `Msg`
    pub fn new(msg_type: MsgType) -> Self {
        let prefix = args().next().unwrap_or("init".to_string());

        Self {
            indent: 0,
            prefix,
            msg_type,
        }
    }

    /// Set a new message type
    pub fn set_msg_type(&mut self, msg_type: MsgType) {
        self.msg_type = msg_type;
    }

    /// Set a different message indentation from
    /// the beginning of the screen/terminal
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    /// Output a message to the terminal
    pub fn print<M: Display>(&self, msg: M) {
        let msg_type = match self.msg_type {
            MsgType::Debug => "DEBUG:",
            MsgType::Info => "", // для info нам не нужно выводить никаких типов
            MsgType::Warning => "warning:",
            MsgType::Error => "error:",
            MsgType::Emerg => "EMERG:",
        };

        match self.msg_type {
            MsgType::Info => println!(
                "{:>indent$}{prefix}: {msg}",
                "",
                prefix = &self.prefix,
                indent = self.indent
            ),
            MsgType::Emerg => {
                eprintln!("{prefix}: {msg_type} {msg}", prefix = &self.prefix);
                loop {}
            }
            _ => eprintln!(
                "{:>indent$}{prefix}: {msg_type} {msg}",
                "",
                prefix = &self.prefix,
                indent = self.indent
            ),
        }
    }
}
