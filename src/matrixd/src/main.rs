mod catcher; // has signal catcher code
mod channels; // has channel prep code
mod matrix; // has sense hat matrix driver code
mod senders; // has test code
mod server;

use catcher::*;
use channels::*;
use futures::future;
use matrix::*;

#[tokio::main]
async fn main() {
    println!("Start matrix service");

    // Create channels
    let (signal_tx, signal_rx, matrix_tx, matrix_rx, server_tx, server_rx) =
        get_channels().unwrap();

    // Create and run catcher handler
    let signal_catcher_handle = signal_catcher(signal_tx.clone(), server_tx.clone()).await;

    // Create and run SenseHat runner
    let mut sensehat_runner = SenseHatRunner::new(signal_rx.clone(), matrix_rx.clone()).unwrap();
    let sensehat_runner_handle = sensehat_runner.run().await;

    // Create and run webserver
    let server_handle = server::run(matrix_tx.clone(), server_rx).await;

    let handles = vec![
        signal_catcher_handle.unwrap(),
        sensehat_runner_handle.unwrap(),
        server_handle.unwrap(),
    ];
    future::join_all(handles).await;

    println!("End matrix service");
}
