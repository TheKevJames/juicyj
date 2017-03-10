#![cfg_attr(feature = "strict", deny(missing_docs))]
//! Compiler for JOOS 1W.

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod analysis;
pub mod error;
pub mod scanner;
