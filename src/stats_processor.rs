use std::{collections::HashMap, sync::Once, thread::{self}};

use lazy_static::lazy_static;

lazy_static!{
    static ref STATS: (flume::Sender<Stats>, flume::Receiver<Stats>) = flume::unbounded::<Stats>();
    static ref STATS_RECV: flume::Receiver<Stats> = STATS.1.clone();
    pub static ref STATS_SEND: flume::Sender<Stats> = STATS.0.clone();
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Stats {
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
    alGenSources,
    alDeleteSources,
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

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        thread::spawn(|| {
            let mut stats: HashMap<Stats, i32> = HashMap::new();
            for stat in STATS_RECV.iter() {
                println!("received! {:?}", stat);

                match stats.get_mut(&stat) {
                    Some(v) => {
                        *v += 1;
                    },
                    None => {
                        stats.insert(stat, 1);
                    },
                }
                
                println!("current stats: {:?}", stats);
            }
        });
    });    
}
