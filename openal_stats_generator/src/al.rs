#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{al_api::api, definitions::*, DECL_THUNK};

#[unsafe(no_mangle)]
pub extern "C" fn alGetProcAddress(fname: *const ALchar) -> *mut c_void {
    _ = crate::stats_processor::STATS_SEND.send(crate::stats_processor::Stats::alGetProcAddress);
    // TODO: wrap the return function of alGetProcAddress so we can get their stats
    unsafe {
        api.alGetProcAddress(fname)
    }
}

DECL_THUNK!{
    alDopplerFactor(value: ALfloat) -> (),
    alDopplerVelocity(value: ALfloat) -> (),
    alSpeedOfSound(value: ALfloat) -> (),
    alDistanceModel(distanceModel: ALenum) -> (),
    alEnable(capability: ALenum) -> (),
    alDisable(capability: ALenum) -> (),
    alIsEnabled(capability: ALenum) -> ALboolean,
    alGetString(param: ALenum) -> *const ALchar,
    alGetBooleanv(param: ALenum, values: *mut ALboolean) -> (),
    alGetIntegerv(param: ALenum, values: *mut ALint) -> (),
    alGetFloatv(param: ALenum, values: *mut ALfloat) -> (),
    alGetDoublev(param: ALenum, values: *mut ALdouble) -> (),
    alGetBoolean(param: ALenum) -> ALboolean,
    alGetInteger(param: ALenum) -> ALint,
    alGetFloat(param: ALenum) -> ALfloat,
    alGetDouble(param: ALenum) -> ALdouble,
    alGetError() -> ALenum,
    alIsExtensionPresent(extname: *const ALchar) -> ALboolean,
    alGetEnumValue(ename: *const ALchar) -> ALenum,
    alListenerf(param: ALenum, value: ALfloat) -> (),
    alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
    alListenerfv(param: ALenum, values: *const ALfloat) -> (),
    alListeneri(param: ALenum, value: ALint) -> (),
    alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
    alListeneriv(param: ALenum, values: *const ALint) -> (),
    alGetListenerf(param: ALenum, value: *mut ALfloat) -> (),
    alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
    alGetListenerfv(param: ALenum, values: *mut ALfloat) -> (),
    alGetListeneri(param: ALenum, value: *mut ALint) -> (),
    alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
    alGetListeneriv(param: ALenum, values: *mut ALint) -> (),
    alGenSources(n: ALsizei, sources: *mut ALuint) -> (),
    alDeleteSources(n: ALsizei, sources: *const ALuint) -> (),
    alIsSource(source: ALuint) -> ALboolean,
    alSourcef(source: ALuint, param: ALenum, value: ALfloat) -> (),
    alSource3f(source: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
    alSourcefv(source: ALuint, param: ALenum, values: *const ALfloat) -> (),
    alSourcei(source: ALuint, param: ALenum, value: ALint) -> (),
    alSource3i(source: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
    alSourceiv(source: ALuint, param: ALenum, values: *const ALint) -> (),
    alGetSourcef(source: ALuint, param: ALenum, value: *mut ALfloat) -> (),
    alGetSource3f(source: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
    alGetSourcefv(source: ALuint, param: ALenum, values: *mut ALfloat) -> (),
    alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint) -> (),
    alGetSource3i(source: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
    alGetSourceiv(source: ALuint, param: ALenum, values: *mut ALint) -> (),
    alSourcePlayv(n: ALsizei, sources: *const ALuint) -> (),
    alSourceStopv(n: ALsizei, sources: *const ALuint) -> (),
    alSourceRewindv(n: ALsizei, sources: *const ALuint) -> (),
    alSourcePausev(n: ALsizei, sources: *const ALuint) -> (),
    alSourcePlay(source: ALuint) -> (),
    alSourceStop(source: ALuint) -> (),
    alSourceRewind(source: ALuint) -> (),
    alSourcePause(source: ALuint) -> (),
    alSourceQueueBuffers(source: ALuint, nb: ALsizei, buffers: *const ALuint) -> (),
    alSourceUnqueueBuffers(source: ALuint, nb: ALsizei, buffers: *mut ALuint) -> (),
    alGenBuffers(n: ALsizei, buffers: *mut ALuint) -> (),
    alDeleteBuffers(n: ALsizei, buffers: *const ALuint) -> (),
    alIsBuffer(buffer: ALuint) -> ALboolean,
    alBufferData(buffer: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei) -> (),
    alBufferf(buffer: ALuint, param: ALenum, value: ALfloat) -> (),
    alBuffer3f(buffer: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
    alBufferfv(buffer: ALuint, param: ALenum, values: *const ALfloat) -> (),
    alBufferi(buffer: ALuint, param: ALenum, value: ALint) -> (),
    alBuffer3i(buffer: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
    alBufferiv(buffer: ALuint, param: ALenum, values: *const ALint) -> (),
    alGetBufferf(buffer: ALuint, param: ALenum, value: *mut ALfloat) -> (),
    alGetBuffer3f(buffer: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
    alGetBufferfv(buffer: ALuint, param: ALenum, values: *mut ALfloat) -> (),
    alGetBufferi(buffer: ALuint, param: ALenum, value: *mut ALint) -> (),
    alGetBuffer3i(buffer: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
    alGetBufferiv(buffer: ALuint, param: ALenum, values: *mut ALint) -> (),
}