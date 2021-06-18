use crate::matrix;

use rand::{self, Rng};
use std::thread;
use tokio::time::Duration;

// Add this line in main() to test
// let _ = senders::async_knocker_run(matrix_tx.clone(), signal_rx.clone()).await;

#[allow(dead_code)]
pub async fn async_knocker_run(
    tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    tokio::task::spawn(async move {
        let tx = tx;
        let rx = signal_rx;
        let mut rng = rand::thread_rng();
        loop {
            crossbeam_channel::select! {
                recv(rx) -> _ => break,
                default(Duration::from_millis(500)) => {
                    println!("async_knocker");
                    let mut d = matrix::Data::new();
                    for i in 0..64 {
                        d.r[i] = rng.gen_range(0..=63);
                        d.g[i] = rng.gen_range(0..=63);
                        d.b[i] = rng.gen_range(0..=63);
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
        let mut rng = rand::thread_rng();
        loop {
            crossbeam_channel::select! {
                recv(rx) -> _ => break,
                default => {
                    println!("sync_knocker");
                    let mut d = matrix::Data::new();
                    for i in 0..64 {
                        d.r[i] = rng.gen_range(0..=63);
                        d.g[i] = rng.gen_range(0..=63);
                        d.b[i] = rng.gen_range(0..=63);
                    }
                    tx.send(d).unwrap();
                    thread::sleep(Duration::from_millis(1100));
                }
            }
        }
    });
}
