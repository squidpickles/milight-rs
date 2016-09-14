use ::std::io;
use ::std::net;
use ::std::sync;
use regex;

error_chain! {
    foreign_links {
        io::Error, Io;
        net::AddrParseError, Address;
        regex::Error, Parse;
        sync::mpsc::RecvError, RecvError;
    }
}
