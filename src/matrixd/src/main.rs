mod catcher; // has signal catcher code
mod matrix; // has sense hat matrix driver codes
mod senders; // has test codes
mod server;

use catcher::*;
use matrix::*;

use crate::senders::async_knocker_run;

#[tokio::main]
async fn main() {
    println!("Start matrix service");

    let signal_rx = signal_catcher().await.unwrap();
    let mut sensehat_runner = SenseHatRunner::new(signal_rx.clone()).unwrap();
    let matrix_tx = sensehat_runner.get_matrix_tx().await;

    sensehat_runner.run().await;
    server::run(matrix_tx.clone(), signal_rx.clone()).await;

    println!("End matrix service");
}
