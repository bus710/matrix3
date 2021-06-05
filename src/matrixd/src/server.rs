use crate::matrix;
use rand::{self, Rng};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::http;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub r0: [u8; 3],
    pub r1: [u8; 3],
    pub g0: [u8; 3],
    pub g1: [u8; 3],
    pub b0: [u8; 3],
    pub b1: [u8; 3],
}

pub async fn pong_handler() -> Result<impl warp::Reply, warp::Rejection> {
    println!("pong");
    Ok(warp::reply::with_status("pong", http::StatusCode::OK))
}

pub async fn matrix_handler(
    d: Data,
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    //
    let mut d = matrix::Data::new();
    let mut rng = rand::thread_rng();
    for i in 0..64 {
        d.r[i] = rng.gen_range(0..=63);
        d.g[i] = rng.gen_range(0..=63);
        d.b[i] = rng.gen_range(0..=63);
        // d.r[i] = rand::random();
        // d.g[i] = rand::random();
        // d.b[i] = rand::random();
    }
    matrix_tx.send(d).unwrap();

    Ok(warp::reply::with_status("", http::StatusCode::OK))
}

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    mut server_rx: mpsc::Receiver<()>,
) {
    let matrix_tx_filter = warp::any().map(move || matrix_tx.clone());
    let body_size_filter = warp::body::content_length_limit(1024 * 32).and(warp::body::json());

    let ping = warp::any()
        .and(warp::path!("v1"))
        .and(warp::path("ping"))
        .and(warp::path::end())
        .and_then(pong_handler);

    let matrix = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("data"))
        .and(body_size_filter)
        .and(warp::path::end())
        .and(matrix_tx_filter.clone())
        .and_then(matrix_handler);

    let routes = ping.or(matrix);
    let routes = routes.with(warp::cors().allow_any_origin());

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            server_rx.recv().await;
        });
    println!("Server is running at {}", addr);
    tokio::task::spawn(server).await.unwrap();
}
