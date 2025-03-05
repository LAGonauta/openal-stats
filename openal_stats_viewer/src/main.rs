slint::include_modules!();

use std::{collections::HashMap, io, thread};

use anyhow::Context;
use interprocess::local_socket::{tokio::Stream, traits::tokio::Listener, GenericNamespaced, ListenerOptions, ToNsName};
use openal_stats_common::Stats;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

// TODO: add proper coloured logging
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let tk = CancellationToken::new();
    let server_runtime_thread = thread::spawn({
        let tk = tk.clone();
        || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
            rt.block_on(async move {
                tk.run_until_cancelled(async move {
                    let socket_name = "openal_stats.sock";
                    let name = socket_name.to_ns_name::<GenericNamespaced>()?;
        
                    let opts = ListenerOptions::new()
                        .name(name);
        
                    let listener = match opts.create_tokio() {
                        Ok(l) => l,
                        Err(e) => todo!("{}", e),
                    };
        
                    eprintln!("Server running at {socket_name}");
        
                    loop {
                        // Sort out situations when establishing an incoming connection caused an error.
                        let conn = match listener.accept().await {
                            Ok(c) => c,
                            Err(e) => {
                                eprintln!("There was an error with an incoming connection: {e}");
                                continue;
                            }
                        };
                        eprintln!("Client connected");
        
                        // Spawn new parallel asynchronous tasks onto the Tokio runtime and hand the connection
                        // over to them so that multiple clients could be processed simultaneously in a
                        // lightweight fashion.
                        tokio::spawn(async move {
                            // The outer match processes errors that happen when we're connecting to something.
                            // The inner if-let processes errors that happen during the connection.
                            if let Err(e) = handle_conn(conn).await {
                                eprintln!("Error while handling connection: {e}");
                            }
                        });

                        if false {
                            break;
                        }
                    }
        
                    anyhow::Ok(())
                }).await.context("cancelled")
            })
        }
    });

    let ui = AppWindow::new()?;
    ui.run()?;

    tk.cancel();
    _ = server_runtime_thread.join();

    Ok(())
}

async fn handle_conn(conn: Stream) -> io::Result<()> {
    let mut recver = BufReader::new(&conn);
    //let mut sender = &conn;

    let mut buffer = Vec::with_capacity(4096);

    let mut sources = 0;
    let mut sources_playing = 0;
    let mut executable = "Unknown".to_owned();
    let mut stats: HashMap<Stats, i32> = HashMap::new();
    // Describe the receive operation as receiving a line into our big buffer.
    while let Ok(r) = recver.read_until(0x00, &mut buffer).await {
        if r == 0 {
            // EOF
            break;
        }

        _ = buffer.truncate(r);

        match postcard::from_bytes_cobs(&mut buffer) {
            Ok(stat) => {
                match stat {                    
                    Stats::Exe { ref name } => {
                        executable = name.to_owned();
                    },
                    _ => {
                        match stat {
                            Stats::alGenSources { n } => {
                                sources += n;
                                println!("'{}' current sources: {}", executable, sources);
                            },
                            Stats::alDeleteSources { n } => {
                                sources -= n;
                                println!("'{}' current sources: {}", executable, sources);
                            },
                            Stats::alSourcePlay => {
                                sources_playing += 1;
                                println!("'{}' playing sources: {}", executable, sources_playing);
                            },
                            Stats::alSourceStop => {
                                sources_playing -= 1;
                                println!("'{}' playing sources: {}", executable, sources_playing);
                            }
                            Stats::alSourceStopv => {
                                eprintln!("'{}' uses stopv!", executable);
                            },
                            Stats::alSourcePlayv => {
                                eprintln!("'{}' uses playv!", executable);
                            },
                            _ => {}
                        }

                        match stats.get_mut(&stat) {
                            Some(v) => {
                                *v += 1;
                            },
                            None => {
                                stats.insert(stat.clone(), 1);
                            },
                        }
                    }
                }
               
                //println!("Got {:?}, '{}' current stats: {:?}", stat, executable, stats);
                //println!("Got {:?}, '{}' current stats: {:?}", stat, executable, stats);
            }
            Err(e) => {
                println!("(ASYNC) Unable to deserialize due to '{}'", e);
            },
        }

        buffer.truncate(0);
    }

    Ok(())
}