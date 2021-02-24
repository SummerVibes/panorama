#![deny(missing_docs)]
//! A simple distributed task scheduling system
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

pub use anyhow::Result;

mod schedule;
mod data;
mod bus;
mod error;
pub mod common;