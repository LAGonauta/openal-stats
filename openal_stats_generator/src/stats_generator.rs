use std::{env, io::{BufReader, Write}, ops::ControlFlow, sync::Once, thread::{self}};

use interprocess::local_socket::{prelude::*, GenericFilePath, GenericNamespaced, SendHalf, Stream};
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
                
                // fallback to not accumulate data in the channel
                for _ in STATS_RECV.iter() {}
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

    let mut buffer = Vec::new();
    buffer.resize(4096, 0);
    
    let exe = env::current_exe()
        .ok()
        .and_then(|p| p.file_name().and_then(|f| f.to_str().map(|s| s.to_owned())))
        .unwrap_or("Unknown".to_owned());

    _ = send_stats(&mut sender, &mut buffer, Stats::Exe { name: exe });

    while let Ok(stat) = STATS_RECV.recv() {
        if let ControlFlow::Break(_) = send_stats(&mut sender, &mut buffer, stat) {
            break;
        }
    }

    anyhow::Ok(())
}

fn send_stats(sender: &mut SendHalf, buffer: &mut Vec<u8>, stat: Stats) -> ControlFlow<()> {
    if let Ok(s) = postcard::to_slice_cobs(&stat, buffer) {
        if let Err(e) = sender.write_all(&s) {
            println!("(ASYNC) Unable to send data to server: {}", e);
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}
