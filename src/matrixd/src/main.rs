// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix; // has sense hat matrix driver codes
mod catcher; // has signal catcher code
mod senders; // has test codes 
mod server; // has webserver code

use matrix::*;
use catcher::*;
use server::*;

#[tokio::main]
async fn main() {
    println!("Start matrix service");

    let signal_rx = signal_catcher().unwrap();

    let mut sensehat_runner = SenseHatRunner::new(signal_rx.clone()).unwrap();
    let matrix_tx = sensehat_runner.get_matrix_tx().await;

    let server_runner= Server::new(matrix_tx.clone(), signal_rx.clone()).unwrap();

    sensehat_runner.run().await;
    server_runner.run().await;

    println!("End matrix service");
}
