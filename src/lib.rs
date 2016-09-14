//! This library provides a set of commands, and their binary equivalents, for communicating with
//! MiLight / LimitlessLED WiFi bridge controllers.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate net2;
extern crate regex;

pub mod v3;
pub mod colors;
pub mod errors;
pub mod wifi;
