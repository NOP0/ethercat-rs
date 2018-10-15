extern crate libc;
extern crate memmap;
extern crate ethercat_sys as ec;

mod master;
mod types;

pub use self::types::*;
pub use self::master::{Master, Domain, SlaveConfig};