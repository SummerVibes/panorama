// #![deny(missing_docs)]
//! A simple distributed task scheduling system

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub use anyhow::Result;

pub mod schedule;
pub mod data;
pub mod bus;
pub mod error;
