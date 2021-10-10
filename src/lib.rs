//! Implementation of the DDS RTPS protocol

#![deny(
    clippy::all,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo
)]
#![warn(clippy::pedantic)]
#![allow(dead_code)] // this is temporary

mod behaviour;
mod discovery;
mod messages;
mod structure;
