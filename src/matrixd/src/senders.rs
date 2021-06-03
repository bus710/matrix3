use crate::matrix;

use std::thread;
use tokio::time::Duration;

pub async fn async_knocker_run(
    tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    tokio::task::spawn(async move {
        let tx = tx;
        let rx = signal_rx;
        loop {
            crossbeam_channel::select! {
                recv(rx) -> _ => break,
                default(Duration::from_millis(500)) => {
                    println!("async_knocker");
                    let mut d = matrix::Data::new();
                    for i in 0..64 {
                        d.r[i] = rand::random();
                        d.g[i] = rand::random();
                        d.b[i] = rand::random();
                    }
                    tx.send(d).unwrap();
                },
            }
        }
    });
}

#[allow(dead_code)]
pub fn sync_knocker_run(
    tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    thread::spawn(move || {
        let tx = tx;
        let rx = signal_rx;
        loop {
            crossbeam_channel::select! {
                recv(rx) -> _ => break,
                default => {
                    println!("sync_knocker");
                    let mut d = matrix::Data::new();
                    for i in 0..64 {
                        d.r[i] = rand::random();
                        d.g[i] = rand::random();
                        d.b[i] = rand::random();
                    }

                    tx.send(d).unwrap();
                    thread::sleep(Duration::from_millis(1100));
                }
            }
        }
    });
}
