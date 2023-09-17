#![feature(absolute_path)]

pub mod op;
pub mod reg;

#[cfg(feature = "full")]
mod std;

#[cfg(feature = "full")]
pub mod compiler;
