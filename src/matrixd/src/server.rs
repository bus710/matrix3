use crate::matrix;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use warp::http;
use warp::Filter;
use warp::{reject, Rejection, Reply};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub r0: [u8; 3],
    pub r1: [u8; 3],
    pub g0: [u8; 3],
    pub g1: [u8; 3],
    pub b0: [u8; 3],
    pub b1: [u8; 3],
}

pub async fn hello2(
    d: Data,
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("hello2 - {:?}", d);
    println!("hello2");

    Ok(warp::reply::with_status("hello2", http::StatusCode::OK))
}

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let matrix_tx_filter = warp::any().map(move || matrix_tx.clone());
    let body_size_filter = warp::body::content_length_limit(1024 * 32).and(warp::body::json());

    let hello2 = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("data"))
        .and(body_size_filter)
        .and(warp::path::end())
        .and(matrix_tx_filter.clone())
        .and_then(hello2);

    let routes = hello.or(hello2);
    let routes = routes.with(warp::cors().allow_any_origin());
    let (tx, mut rx) = mpsc::channel(1);

    bb(tx, signal_rx);

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            println!("a");
            rx.recv().await;
            println!("b");
        });
    println!("{}", addr);
    tokio::task::spawn(server).await.unwrap();
}

fn bb(tx: mpsc::Sender<()>, server_rx: crossbeam_channel::Receiver<()>) {
    tokio::task::spawn(async move {
        let tx = tx;
        loop {
            println!("runner");
            let _ = server_rx.recv();
            let _ = tx.send(());
        }
    });
}
