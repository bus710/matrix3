use crossbeam_channel::unbounded;
use tokio::signal;
use tokio::sync::mpsc;

pub async fn signal_catcher() -> Result<(crossbeam_channel::Receiver<()>, mpsc::Receiver<()>), ctrlc::Error> {
    let (signal_tx, signal_rx) = unbounded();
    let (server_tx, server_rx) = mpsc::channel(1);
    tokio::task::spawn(async move {
        signal::ctrl_c().await.expect("");
        println!(" - got interrupt");
        let _ = signal_tx.send(());
        let _ = signal_tx.send(());
        let _ = signal_tx.send(());
        let _ = server_tx.send(());
        let _ = server_tx.send(());
        let _ = server_tx.send(());
    });

    Ok((signal_rx.clone(), server_rx))
}
