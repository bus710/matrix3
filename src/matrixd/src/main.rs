// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod catcher; // has signal catcher code
mod matrix; // has sense hat matrix driver codes
mod senders; // has test codes

use catcher::*;
use matrix::*;

use crate::senders::async_knocker_run;

#[tokio::main]
async fn main() {
    println!("Start matrix service");

    let signal_rx = signal_catcher().unwrap();

    let mut sh_runner = SenseHatRunner::new(signal_rx.clone()).unwrap();
    let matrix_tx = sh_runner.get_tx().await;

    async_knocker_run(matrix_tx.clone(), signal_rx.clone()).await;
    sh_runner.run().await;

    println!("End matrix service");
}
