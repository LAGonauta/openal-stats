#![allow(non_snake_case)]

use crate::{definitions::{ALsizei, ALuint}, stats_processor::init};

#[unsafe(no_mangle)]
pub extern "C" fn alGenSources(n: ALsizei, sources: *mut ALuint) {
}