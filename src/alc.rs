use crate::{definitions::{ALCchar, ALCdevice, ALCint}, stats_processor::init};

#[unsafe(no_mangle)]
pub extern "C" fn alcOpenDevice(device_name: *const ALCchar) {

}

#[unsafe(no_mangle)]
pub extern "C" fn alcCreateContext(dev: *const ALCdevice, attrlist: *const ALCint) {
    init();
}