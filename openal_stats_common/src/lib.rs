use serde::{Deserialize, Serialize};
use strum::Display;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Display, Serialize, Deserialize)]
pub enum Stats {
    Exe {name: String},
    Ping,
    alcOpenDevice,
    alcGetProcAddress,
    alcCreateContext,
    alcMakeContextCurrent,
    alcProcessContext,
    alcSuspendContext,
    alcDestroyContext,
    alcGetCurrentContext,
    alcGetContextsDevice,
    alcCloseDevice,
    alcGetError,
    alcIsExtensionPresent,
    alcGetEnumValue,
    alcGetString,
    alcGetIntegerv,
    alcCaptureOpenDevice,
    alcCaptureCloseDevice,
    alcCaptureStart,
    alcCaptureStop,
    alcCaptureSamples,
    alDopplerFactor,
    alDopplerVelocity,
    alSpeedOfSound,
    alDistanceModel,
    alEnable,
    alDisable,
    alIsEnabled,
    alGetString,
    alGetBooleanv,
    alGetIntegerv,
    alGetFloatv,
    alGetDoublev,
    alGetBoolean,
    alGetInteger,
    alGetFloat,
    alGetDouble,
    alGetError,
    alIsExtensionPresent,
    alGetProcAddress,
    alGetEnumValue,
    alListenerf,
    alListener3f,
    alListenerfv,
    alListeneri,
    alListener3i,
    alListeneriv,
    alGetListenerf,
    alGetListener3f,
    alGetListenerfv,
    alGetListeneri,
    alGetListener3i,
    alGetListeneriv,
    alGenSources { n: i32 },
    alDeleteSources { n: i32 },
    alIsSource,
    alSourcef,
    alSource3f,
    alSourcefv,
    alSourcei,
    alSource3i,
    alSourceiv,
    alGetSourcef,
    alGetSource3f,
    alGetSourcefv,
    alGetSourcei,
    alGetSource3i,
    alGetSourceiv,
    alSourcePlayv,
    alSourceStopv,
    alSourceRewindv,
    alSourcePausev,
    alSourcePlay,
    alSourceStop,
    alSourceRewind,
    alSourcePause,
    alSourceQueueBuffers,
    alSourceUnqueueBuffers,
    alGenBuffers,
    alDeleteBuffers,
    alIsBuffer,
    alBufferData,
    alBufferf,
    alBuffer3f,
    alBufferfv,
    alBufferi,
    alBuffer3i,
    alBufferiv,
    alGetBufferf,
    alGetBuffer3f,
    alGetBufferfv,
    alGetBufferi,
    alGetBuffer3i,
    alGetBufferiv,
    // for custom extensions, such as XRAM
    CustomAlExtension(String),
    CustomAlcExtension(String)
}
