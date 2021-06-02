// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;
mod senders;

use matrix::SenseHatRunner;
use senders::*;

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
