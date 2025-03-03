use std::{collections::HashMap, io::{BufReader, Write}, sync::Once, thread::{self}};

use interprocess::local_socket::{prelude::*, Stream, GenericFilePath, GenericNamespaced};
use openal_stats_common::Stats;
use lazy_static::lazy_static;

lazy_static!{
    static ref STATS: (flume::Sender<Stats>, flume::Receiver<Stats>) = flume::unbounded::<Stats>();
    static ref STATS_RECV: flume::Receiver<Stats> = STATS.1.clone();
    pub static ref STATS_SEND: flume::Sender<Stats> = STATS.0.clone();
}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        thread::spawn(|| {
            
            if let Err(e) = connected_stats() {
                eprintln!("Unable to connect to server: '{}'", e);
                
                // fallback
                let mut stats: HashMap<Stats, i32> = HashMap::new();
                for stat in STATS_RECV.iter() {
                    println!("received! {:?}", stat);
    
                    match stats.get_mut(&stat) {
                        Some(v) => {
                            *v += 1;
                        },
                        None => {
                            stats.insert(stat.clone(), 1);
                        },
                    }
                    
                    println!("(SYNC) Got {:?}, current stats: {:?}", stat, stats);
                }
            }


            anyhow::Ok(())
        });
    });    
}

fn connected_stats() -> anyhow::Result<()> {
    let socket_name = "openal_stats.sock";
    let name = if GenericNamespaced::is_supported() {
        socket_name.to_ns_name::<GenericNamespaced>()?
    } else {
        ("/tmp/".to_owned()+socket_name).to_fs_name::<GenericFilePath>()?
    };

    let conn = Stream::connect(name)?;

    let (recver, mut sender) = conn.split();
    let _recver = BufReader::new(recver);

    
    while let Ok(stat) = STATS_RECV.recv() {
        if let Err(e) = sender.write_all(format!("{}\n", stat).as_bytes()) {
            println!("(ASYNC) Unable to send data to server: {}", e);
            break;
        }
    }

    anyhow::Ok(())
}
