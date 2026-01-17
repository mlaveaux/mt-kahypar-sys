//! Suppress various warnings from the generated bindings.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// When the "bundled" feature is enabled, include the generated bindings.
#[cfg(feature = "bundled")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));