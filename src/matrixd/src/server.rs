use crate::matrix;
use serde_derive::{Deserialize, Serialize};
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

fn json_body() -> impl Filter<Extract = (Data,), Error = warp::Rejection> + Clone {
    println!("json_body");
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn hello2(
    d: Data,
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("hello2 - {:?}", d);

    Ok(warp::reply::with_status("hello2", http::StatusCode::OK))
}

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let matrix_tx_filter = warp::any().map(move || matrix_tx.clone());

    let hello2 = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("data"))
        .and(warp::path::end())
        .and(json_body())
        .and(matrix_tx_filter.clone())
        .and_then(hello2);

    let routes = hello2.or(hello).with(warp::cors().allow_any_origin());

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            let _ = signal_rx.recv();
        });
    println!("{}", addr);
    tokio::task::spawn(server).await.unwrap();
}
