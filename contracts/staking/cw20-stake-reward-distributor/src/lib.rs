#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod contract;
mod error;
pub mod msg;
pub mod state;

mod robot;
mod robot_test;
#[cfg(test)]
mod tests;

pub use crate::error::ContractError;
