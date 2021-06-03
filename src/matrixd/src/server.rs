use crate::matrix;

use warp::Filter;
use tokio::time::{sleep,Duration};

pub struct Server {
    matrix_tx: crossbeam_channel::Sender<matrix::Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
}

impl Server {
    pub fn new(
        matrix_tx: crossbeam_channel::Sender<matrix::Data>,
        signal_rx: crossbeam_channel::Receiver<()>,
    ) -> Result<Server, String> {
        Ok(Server {
            matrix_tx,
            signal_rx,
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

        // Access to http://127.0.0.1:8080
        // try "../www/index.html" if this doesn't work
        let index = warp::get()
            .and(warp::path::end())
            .and(warp::fs::file("www/index.html"));

        // Access to http://127.0.0.1:8080/hello/$ANY_STRING
        let hello = warp::path!("hello" / String).map(|name| {
            println!("New request");
            format!("Hello, {:?}!", name)
        });

        // Routing
        let routes = index.or(hello);

        let (addr, server_) =
            warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async move {
                // self.signal_rx.recv().unwrap();
                println!("server goes down");
            });
        println!("{:?}", addr);
        let handler = tokio::task::spawn(server_);
        handler.await.unwrap();
    }
}
