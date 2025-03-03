use std::{collections::HashMap, sync::Once, thread::{self}};

use interprocess::local_socket::{tokio::{prelude::*, Stream}, GenericFilePath, GenericNamespaced};
use openal_stats_common::Stats;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, try_join};
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
            if let Ok(rt) = tokio::runtime::Builder::new_current_thread().enable_all().build() {
                rt.block_on(async {
                    let r = tokio::spawn(async {
                        let socket_name = "openal_stats.sock";
                        let name = if GenericNamespaced::is_supported() {
                            socket_name.to_ns_name::<GenericNamespaced>()?
                        } else {
                            ("/tmp/".to_owned()+socket_name).to_fs_name::<GenericFilePath>()?
                        };

                        let conn = Stream::connect(name).await?;

                        let (recver, mut sender) = conn.split();
                        let _recver = BufReader::new(recver);

                        
                        while let Ok(stat) = STATS_RECV.recv_async().await {
                            if let Err(e) = sender.write_all(format!("{}\n", stat).as_bytes()).await {
                                println!("(ASYNC) Unable to send data to server: {}", e);
                                break;
                            }
                        }

                        anyhow::Ok(())
                    });

                    if let Ok(r) = r.await {
                        if let Err(e) = r {
                            eprintln!("Stat viewer error: '{}'", e);
                        }
                    }
                });
            }
            
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

            anyhow::Ok(())
        });
    });    
}
