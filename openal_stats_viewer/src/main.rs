use std::{collections::HashMap, io};

use anyhow::Context;
use interprocess::local_socket::{tokio::Stream, traits::tokio::Listener, GenericNamespaced, ListenerOptions, ToNsName};
use openal_stats_common::Stats;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

// TODO: add proper coloured logging
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // create RT by hand, in its own thread.
    // the main thread will be owned by Slint.
    let tk = CancellationToken::new();
    tokio::spawn(async move {
        tk.run_until_cancelled(async {
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
            }

            anyhow::Ok(())
        }).await.context("cancelled")
    }).await???;

    Ok(())
}

async fn handle_conn(conn: Stream) -> io::Result<()> {
    let mut recver = BufReader::new(&conn);
    //let mut sender = &conn;

    let mut buffer = Vec::new();
    buffer.resize(4096, 0);

    let mut stats: HashMap<Stats, i32> = HashMap::new();
    // Describe the receive operation as receiving a line into our big buffer.
    while let Ok(r) = recver.read_until(0, &mut buffer).await {
        if r == 0 {
            // EOF
            break;
        }

        _ = buffer.truncate(r);
        if let Ok(stat) = postcard::from_bytes_cobs(&mut buffer) {
            match stats.get_mut(&stat) {
                Some(v) => {
                    *v += 1;
                },
                None => {
                    stats.insert(stat.clone(), 1);
                },
            }
    
            println!("(ASYNC) Got {:?}, current stats: {:?}", stat, stats);
        }

        buffer.truncate(0);
    }

    Ok(())
}