// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use crate::matrix::SenseHatRunner;
use std::thread;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut sh_runner = SenseHatRunner::new().unwrap();
    let tx = sh_runner.get_tx().await;
    let tx2 = sh_runner.get_tx().await;

    async_knocker_run(tx).await;
    sync_knocker_run(tx2);
    sh_runner.run().await;

    println!("Bye!");
}

async fn async_knocker_run(tx: crossbeam_channel::Sender<matrix::Data>) {
    tokio::task::spawn(async move {
        let tx = tx;

        loop {
            crossbeam_channel::select! {
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

fn sync_knocker_run(tx: crossbeam_channel::Sender<matrix::Data>) {
    thread::spawn(move || {
        let tx = tx;
        loop {
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
    });
}
