use crossbeam_channel::unbounded;

pub fn signal_catcher() -> Result<crossbeam_channel::Receiver<()>, ctrlc::Error> {
    let (tx, rx) = unbounded();
    ctrlc::set_handler(move || {
        println!(" - got interrupt");
        let _ = tx.send(());
        let _ = tx.send(());
        let _ = tx.send(());
    })?;
    Ok(rx)
}
