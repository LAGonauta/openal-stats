#![allow(non_snake_case)]

use crate::{definitions::{ALCcontext, ALsizei, ALuint, ALvoid}, DECL_THUNK};

DECL_THUNK!{
    alcDestroyContext(ctx: *mut ALCcontext) -> ALvoid,
    alGenSources(n: ALsizei, sources: *mut ALuint) -> ALvoid,
}