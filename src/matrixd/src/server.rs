use std::time::Duration;

use crate::matrix;
use futures::SinkExt;
use futures::StreamExt;
use rand::{self, Rng};
use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::http;
use warp::ws::Message;
use warp::ws::WebSocket;
use warp::Filter;

// Serde doesn't support array format with 64 items -> go with 32 for now
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    pub r0: [u8; 32],
    pub r1: [u8; 32],
    pub g0: [u8; 32],
    pub g1: [u8; 32],
    pub b0: [u8; 32],
    pub b1: [u8; 32],
}

impl Data {
    fn new() -> Self {
        Data {
            r0: [0; 32],
            r1: [0; 32],
            g0: [0; 32],
            g1: [0; 32],
            b0: [0; 32],
            b1: [0; 32],
        }
    }
}

// Returns pong when /v1/ping gets hit
async fn pong_handler() -> Result<impl warp::Reply, warp::Rejection> {
    info!("pong");
    Ok(warp::reply::with_status("pong ", http::StatusCode::OK))
}

// Passes the given matrix value to SenseHat driver
async fn matrix_handler(
    d: Data,
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("matrix");
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
async fn random_handler(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("random");
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

// Broadcast the current value (of SenseHat) to connected web socket clients
async fn ws_handler(
    ws: warp::ws::Ws,
    ws_rx: crossbeam_channel::Receiver<matrix::Data>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("ws");
    ws.on_upgrade(move |websocket| ws_connected(websocket, ws_rx));

    tokio::time::sleep(Duration::from_millis(10000)).await;
    Ok(warp::reply::with_status("", http::StatusCode::OK))
}

async fn ws_connected(websocket: WebSocket, ws_rx: crossbeam_channel::Receiver<matrix::Data>) {
    info!("connected");
    let (mut tx, mut rx) = websocket.split();
    tokio::task::spawn(async move {
        loop {
            let d = ws_rx.recv().unwrap();
            // Create a buffer to match the channel
            let mut d2 = Data::new();
            // Map the data from R32:32 to R64
            for i in 0..32 {
                d2.r0[i] = d.r[i];
                d2.r1[i] = d.r[i + 32];
                d2.g0[i] = d.g[i];
                d2.g1[i] = d.g[i + 32];
                d2.b0[i] = d.b[i];
                d2.b1[i] = d.b[i + 32];
            }
            let v = serde_json::to_string(&d2).unwrap();
            info!("{:?}", v);
            let _ = tx.send(Message::text(v)).await;
        }
    });

    while let Some(result) = rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                info!("{:?}", e.to_string());
                break;
            }
        };
        info!("{:?}", msg);
    }
}

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    ws_rx: crossbeam_channel::Receiver<matrix::Data>,
    mut server_rx: mpsc::UnboundedReceiver<()>,
) -> Result<tokio::task::JoinHandle<()>, String> {
    // Create filters
    let with_matrix_tx = warp::any().map(move || matrix_tx.clone());
    let with_ws_rx = warp::any().map(move || ws_rx.clone());
    let body_size_filter = warp::body::content_length_limit(1024 * 32).and(warp::body::json());

    // Create routes
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
        .and(with_matrix_tx.clone())
        .and_then(matrix_handler);

    let random_route = warp::any()
        .and(warp::path("v1"))
        .and(warp::path("random"))
        .and(warp::path::end())
        .and(with_matrix_tx.clone())
        .and_then(random_handler);

    let ws_route = warp::path("v1")
        .and(warp::path("ws"))
        .and(warp::path::end())
        .and(warp::ws())
        .and(with_ws_rx.clone())
        .and_then(ws_handler);

    // Combine routes and add CORS rule
    let routes = ping_route.or(matrix_route).or(random_route).or(ws_route);
    let routes = routes.with(warp::cors().allow_any_origin());

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
            let _ = server_rx.recv().await.unwrap();
        });

    info!("Server is running at {}", addr);

    let handle = tokio::task::spawn(server);

    Ok(handle)
}
