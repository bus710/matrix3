use crate::matrix;

// use tokio::sync::oneshot;
use futures::channel::oneshot;
use tokio::time::{sleep, Duration};
use warp::Filter;

pub struct Server {
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
    rx: oneshot::Receiver<bool>,
}

impl Server {
    pub fn new(
        matrix_tx: crossbeam_channel::Sender<matrix::Data>,
        signal_rx: crossbeam_channel::Receiver<()>,
    ) -> Result<Server, String> {
        let (_, rx) = oneshot::channel();
        Ok(Server {
            matrix_tx,
            signal_rx,
            rx
        })
    }

    pub async fn run(self) {
        // Init the matrix
        let mut d = matrix::Data::new();
        for i in 0..64 {
            d.r[i] = 10;
            d.g[i] = 10;
            d.b[i] = 10;
        }
        self.matrix_tx.send(d).unwrap();
        sleep(Duration::from_millis(100)).await;

        //
        let index = warp::get()
            .and(warp::path::end())
            .and(warp::fs::file("www/index.html"));

        //
        let hello = warp::path!("hello" / String).map(|name| {
            println!("New request");
            format!("Hello, {:?}!", name)
        });

        // Routing
        let routes = index.or(hello);

        //
        let (_, server_) =
            warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
                self.rx.await.ok();
            });
        tokio::task::spawn(server_).await.unwrap();
    }

    pub async fn run2(self) {
        let routes = warp::any().map(|| "");
        warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    }
}
