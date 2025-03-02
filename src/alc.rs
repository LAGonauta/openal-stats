use std::ffi::c_void;

use crate::{al_api::api, definitions::*, stats_processor::init, DECL_THUNK};

#[unsafe(no_mangle)]
pub extern "C" fn alcOpenDevice(device_name: *const ALCchar) -> *mut ALCdevice {
    init();
    _ = crate::stats_processor::STATS_SEND.send(crate::stats_processor::Stats::alcOpenDevice);
    unsafe {
        api.alcOpenDevice(device_name)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn alcGetProcAddress(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut c_void {
    init();
    _ = crate::stats_processor::STATS_SEND.send(crate::stats_processor::Stats::alcGetProcAddress);
    // TODO: wrap the return function of alcGetProcAddress so we can get their stats
    unsafe {
        api.alcGetProcAddress(device, funcname)
    }
}

DECL_THUNK!{
    alcCreateContext(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext,
    alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean,
    alcProcessContext(context: *mut ALCcontext) -> (),
    alcSuspendContext(context: *mut ALCcontext) -> (),
    alcDestroyContext(context: *mut ALCcontext) -> (),
    alcGetCurrentContext() -> *mut ALCcontext,
    alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice,
    alcCloseDevice(device: *mut ALCdevice) -> ALCboolean,
    alcGetError(device: *mut ALCdevice) -> ALCenum,
    alcIsExtensionPresent(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean,
    alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum,
    alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar,
    alcGetIntegerv(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, values: *mut ALCint) -> (),
    alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice,
    alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean,
    alcCaptureStart(device: *mut ALCdevice) -> (),
    alcCaptureStop(device: *mut ALCdevice) -> (),
    alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei) -> (),
}