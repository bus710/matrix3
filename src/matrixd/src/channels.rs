use crate::matrix;
use crossbeam_channel;
use tokio::sync;

pub fn get_channels() -> Result<
    (
        crossbeam_channel::Sender<()>,
        crossbeam_channel::Receiver<()>,
        crossbeam_channel::Sender<matrix::Data>,
        crossbeam_channel::Receiver<matrix::Data>,
        sync::mpsc::UnboundedSender<()>,
        sync::mpsc::UnboundedReceiver<()>,
    ),
    String,
> {
    // To gracefully shutdown the SenseHat runner and test functions
    let (signal_tx, signal_rx) = crossbeam_channel::unbounded();
    // To deliver arrays from outside to the SenseHat runner
    let (matrix_tx, matrix_rx) = crossbeam_channel::unbounded();
    // To gracefully shutdown the server
    let (server_tx, server_rx) = sync::mpsc::unbounded_channel();

    Ok((
        signal_tx, signal_rx, matrix_tx, matrix_rx, server_tx, server_rx,
    ))
}
