use std::{sync::Once, thread::{self}};

use lazy_static::lazy_static;

lazy_static!{
    static ref STATS: (flume::Sender<Stats>, flume::Receiver<Stats>) = flume::unbounded::<Stats>();
    static ref STATS_RECV: flume::Receiver<Stats> = STATS.1.clone();
    pub static ref STATS_SEND: flume::Sender<Stats> = STATS.0.clone();
}

pub enum Stats {}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        thread::spawn(|| {
            for stat in STATS_RECV.iter() {
    
            }
        });
    });    
}
