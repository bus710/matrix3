// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use crate::matrix::SenseHatRunner;
use crossbeam_channel::unbounded;
use std::thread;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let signal_rx = signal_catcher().unwrap();
    let signal_rx2 = signal_rx.clone();
    let signal_rx3 = signal_rx.clone();

    let mut sh_runner = SenseHatRunner::new(signal_rx).unwrap();
    let tx = sh_runner.get_tx().await;
    let tx2 = sh_runner.get_tx().await;

    sync_knocker_run(tx, signal_rx2);
    async_knocker_run(tx2, signal_rx3).await;
    sh_runner.run().await;

    println!("Bye!");
}

async fn async_knocker_run(
    tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    tokio::task::spawn(async move {
        let tx = tx;
        let rx = signal_rx;
        loop {
            crossbeam_channel::select! {
                recv(rx) -> _ => break,
                default(Duration::from_millis(2000)) => {
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

fn sync_knocker_run(
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

fn signal_catcher() -> Result<crossbeam_channel::Receiver<()>, ctrlc::Error> {
    let (tx, rx) = unbounded();
    ctrlc::set_handler(move || {
        println!(" - got interrupt");
        let _ = tx.send(());
        let _ = tx.send(());
        let _ = tx.send(());
    })?;
    Ok(rx)
}
