use crossbeam_channel::unbounded;
use tokio::signal;

pub async fn signal_catcher() -> Result<crossbeam_channel::Receiver<()>, ctrlc::Error> {
    //
    let (signal_tx, signal_rx) = unbounded();
    tokio::task::spawn(async move {
        signal::ctrl_c().await.expect("");
        println!(" - got interrupt");
        let _ = signal_tx.send(());
        let _ = signal_tx.send(());
        let _ = signal_tx.send(());
    });
    // ctrlc::set_handler(move || {
    //     println!(" - got interrupt");
    //     let _ = signal_tx.send(());
    //     let _ = signal_tx.send(());
    //     let _ = signal_tx.send(());
    // })?;
    Ok(signal_rx.clone())
}
