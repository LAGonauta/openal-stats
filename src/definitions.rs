#![allow(non_snake_case)]

use std::ffi::{c_char, c_double, c_float, c_int, c_schar, c_short, c_uchar, c_uint, c_ushort, c_void};

pub(crate) type ALboolean = c_char;
pub(crate) type ALchar = c_char;
pub(crate) type ALbyte = c_schar;
pub(crate) type ALubyte = c_uchar;
pub(crate) type ALshort = c_short;
pub(crate) type ALushort = c_ushort;
pub(crate) type ALint = c_int;
pub(crate) type ALuint = c_uint;
pub(crate) type ALsizei = c_int;
pub(crate) type ALenum = c_int;
pub(crate) type ALfloat = c_float;
pub(crate) type ALdouble = c_double;
pub(crate) type ALvoid = c_void;

pub(crate) type ALCdevice = c_void;
pub(crate) type ALCcontext = c_void;
pub(crate) type ALCboolean = c_char;
pub(crate) type ALCchar = c_char;
pub(crate) type ALCbyte = c_schar;
pub(crate) type ALCubyte = c_uchar;
pub(crate) type ALCshort = c_short;
pub(crate) type ALCushort = c_ushort;
pub(crate) type ALCint = c_int;
pub(crate) type ALCuint = c_uint;
pub(crate) type ALCsizei = c_int;
pub(crate) type ALCenum = c_int;
pub(crate) type  ALCfloat = c_float;
pub(crate) type  ALCdouble = c_double;
pub(crate) type  ALCvoid = c_void;