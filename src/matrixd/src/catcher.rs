// use std::thread::JoinHandle;
// use crossbeam_channel::unbounded;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::Duration;

pub async fn signal_catcher(
    signal_tx: crossbeam_channel::Sender<()>,
    server_tx: mpsc::UnboundedSender<()>,
) -> Result<tokio::task::JoinHandle<()>, ctrlc::Error> {
    let handle = tokio::task::spawn(async move {
        signal::ctrl_c().await.expect("");
        info!(" - got interrupt");
        let _ = signal_tx.send(());
        let _ = server_tx.send(());
        tokio::time::sleep(Duration::from_millis(100)).await;
    });

    Ok(handle)
}
