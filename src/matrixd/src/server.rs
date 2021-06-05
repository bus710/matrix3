use crate::matrix;
use rand::{self, Rng};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::http;
use warp::Filter;

// Serde doesn't support array format with 64 items -> go with 32 for now
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub r0: [u8; 32],
    pub r1: [u8; 32],
    pub g0: [u8; 32],
    pub g1: [u8; 32],
    pub b0: [u8; 32],
    pub b1: [u8; 32],
}

// Returns pong when /v1/ping gets hit
pub async fn pong_handler() -> Result<impl warp::Reply, warp::Rejection> {
    println!("pong");
    Ok(warp::reply::with_status("pong ", http::StatusCode::OK))
}

// Passes the given matrix value to SenseHat driver
pub async fn matrix_handler(
    d: Data,
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Check data lengths
    let mut len_vec = Vec::new();
    len_vec.push(d.r0.len());
    len_vec.push(d.r1.len());
    len_vec.push(d.g0.len());
    len_vec.push(d.g1.len());
    len_vec.push(d.b0.len());
    len_vec.push(d.b1.len());

    match len_vec.iter().find(|&&v| v != 32).take() {
        Some(v) => {
            return Ok(warp::reply::with_status(
                format!("{:?}", v),
                http::StatusCode::BAD_REQUEST,
            ));
        }
        None => (),
    }
    // Create a buffer to match the channel
    let mut d2 = matrix::Data::new();
    // Map the data from R32:32 to R64
    for i in 0..32 {
        d2.r[i] = d.r0[i];
        d2.r[i + 32] = d.r1[i];
        d2.g[i] = d.g0[i];
        d2.g[i + 32] = d.g1[i];
        d2.b[i] = d.b0[i];
        d2.b[i + 32] = d.b1[i];
    }
    // Send data
    matrix_tx.send(d2).unwrap();
    // Return result
    Ok(warp::reply::with_status(
        "".to_string(),
        http::StatusCode::OK,
    ))
}

// Passes a random matrix value to SenseHat driver
pub async fn random_handler(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Create a buffer to match the channel
    let mut d2 = matrix::Data::new();
    // Randomize
    let mut rng = rand::thread_rng();
    for i in 0..64 {
        d2.r[i] = rng.gen_range(0..=63) / 3;
        d2.g[i] = rng.gen_range(0..=63) / 3;
        d2.b[i] = rng.gen_range(0..=63) / 3;
    }
    // Send data
    matrix_tx.send(d2).unwrap();
    // Return result
    Ok(warp::reply::with_status("", http::StatusCode::OK))
}

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    mut server_rx: mpsc::Receiver<()>,
) {
    let matrix_tx_filter = warp::any().map(move || matrix_tx.clone());
    let body_size_filter = warp::body::content_length_limit(1024 * 32).and(warp::body::json());

    let ping_route = warp::any()
        .and(warp::path("v1"))
        .and(warp::path("ping"))
        .and(warp::path::end())
        .and_then(pong_handler);

    let matrix_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("matrix"))
        .and(body_size_filter)
        .and(warp::path::end())
        .and(matrix_tx_filter.clone())
        .and_then(matrix_handler);

    let random_route = warp::any()
        .and(warp::path("v1"))
        .and(warp::path("random"))
        .and(warp::path::end())
        .and(matrix_tx_filter.clone())
        .and_then(random_handler);

    let routes = ping_route.or(matrix_route).or(random_route);
    let routes = routes.with(warp::cors().allow_any_origin());

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            server_rx.recv().await;
        });

    println!("Server is running at {}", addr);
    tokio::task::spawn(server).await.unwrap();
}
