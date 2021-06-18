use crate::matrix;
use crossbeam_channel;
use tokio::sync;

pub fn get_channels() -> Result<
    (
        crossbeam_channel::Sender<()>,
        crossbeam_channel::Receiver<()>,
        crossbeam_channel::Sender<matrix::Data>,
        crossbeam_channel::Receiver<matrix::Data>,
        crossbeam_channel::Sender<matrix::Data>,
        crossbeam_channel::Receiver<matrix::Data>,
        sync::mpsc::UnboundedSender<()>,
        sync::mpsc::UnboundedReceiver<()>,
    ),
    String,
> {
    // To gracefully shutdown SenseHat runner and test functions
    let (signal_tx, signal_rx) = crossbeam_channel::unbounded();
    // To deliver arrays from outside to SenseHat runner
    let (matrix_tx, matrix_rx) = crossbeam_channel::unbounded();
    // To deliver arrays from SenseHat runner to outside
    let (ws_tx, ws_rx) = crossbeam_channel::unbounded();
    // To gracefully shutdown
    let (server_tx, server_rx) = sync::mpsc::unbounded_channel();

    Ok((
        signal_tx, signal_rx, matrix_tx, matrix_rx, ws_tx, ws_rx, server_tx, server_rx,
    ))
}
