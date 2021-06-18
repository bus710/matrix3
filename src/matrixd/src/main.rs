mod catcher; // has signal catcher code
mod channels; // has channel prep code
mod matrix; // has sense hat matrix driver code
mod senders; // has test code
mod server; // has web server code

use catcher::*;
use channels::*;
use futures::future;
use matrix::*;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    info!("Start matrix service");

    // Init logger
    pretty_env_logger::init();

    // Create channels
    let (signal_tx, signal_rx, matrix_tx, matrix_rx, ws_tx, ws_rx, server_tx, server_rx) =
        get_channels().unwrap();

    // Create and run catcher handler
    let signal_catcher_handle = signal_catcher(signal_tx.clone(), server_tx.clone()).await;

    // Create and run SenseHat runner
    let mut sensehat_runner =
        SenseHatRunner::new(matrix_rx.clone(), ws_tx.clone(), signal_rx.clone()).unwrap();
    let sensehat_runner_handle = sensehat_runner.run().await;

    // Create and run webserver
    let server_handle = server::run(matrix_tx.clone(), ws_rx.clone(), server_rx).await;

    // Wait for threads
    let handles = vec![
        signal_catcher_handle.unwrap(),
        sensehat_runner_handle.unwrap(),
        server_handle.unwrap(),
    ];
    future::join_all(handles).await;

    info!("End matrix service");
}
