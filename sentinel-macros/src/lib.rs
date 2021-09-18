//! This crate supplies out-of-the-box attribute macors to ease sentinel usage.  
//! It depends on the [sentinel-rs] crate.
//! Currently, only one sentinel attribute macro is permited to added on a single function.

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[macro_use]
#[doc(hidden)]
mod utils;
use utils::*;

mod flow;
mod system;

build!(flow);
build!(system);