mod catcher; // has signal catcher code
mod matrix; // has sense hat matrix driver codes
mod senders; // has test codes
mod server;

use catcher::*;
use matrix::*;

#[tokio::main]
async fn main() {
    println!("Start matrix service");

    // Create a catcher handler and get some receivers for graceful shutdown
    let (signal_rx, server_rx) = signal_catcher().await.unwrap();

    // Create a sense hat runner
    // Give it a signal receiver and take a matrix data server
    let mut sensehat_runner = SenseHatRunner::new(signal_rx.clone()).unwrap();
    let matrix_tx = sensehat_runner.get_matrix_tx().await;
    // Run the matrix thread
    sensehat_runner.run().await;

    // Run the webserver
    server::run(matrix_tx.clone(), server_rx).await;

    println!("End matrix service");
}
