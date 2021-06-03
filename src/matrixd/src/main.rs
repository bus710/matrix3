mod catcher; // has signal catcher code
mod matrix; // has sense hat matrix driver codes
mod senders; // has test codes
mod server;

use catcher::*;
use matrix::*;

use crate::senders::async_knocker_run;

use actix_web::{web, App, HttpServer};

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    println!("Start matrix service");

    let signal_rx = signal_catcher().unwrap();

    let mut sh_runner = SenseHatRunner::new(signal_rx.clone()).unwrap();
    let matrix_tx = sh_runner.get_tx().await;

    async_knocker_run(matrix_tx.clone(), signal_rx.clone()).await;
    sh_runner.run().await;

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(|| App::new().route("/", web::get().to(server::hello)))
        .bind("0.0.0.0:8000")?
        .run()
        .await?;
    sys.await?;
    Ok(server_res)

    // HttpServer::new(|| {
    //     App::new()
    //         .service(server::hello)
    //         .service(server::echo)
    //         .route("/hey", web::get().to(server::manual_hello))
    // })
    // .bind("0.0.0.0:8080")?
    // .run()
    // .await
}
