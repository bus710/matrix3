use crate::matrix;
use serde_derive::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

pub async fn run(
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let cleanup = warp::path::end().map(|| {
        let d = matrix::Data::new();
        matrix_tx.send(d).unwrap();
    });

    // let routes = hello.with(warp::cors().allow_any_origin());
    let routes = warp::any().and(hello2());

    let (_, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 3030), async move {
            signal_rx.recv().unwrap();
        });
    tokio::task::spawn(server).await.unwrap();
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn hello2() -> impl Filter<Extract = impl Reply> + Clone {
    warp::path!("hello" / String).map(|name| format!("Hello, {}!", name))
}
