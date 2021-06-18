use serde_derive::{Deserialize, Serialize};
use std::net::TcpListener;
use std::thread::spawn;

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
    fn all_1(&mut self) {
        for i in 0..32 {
            self.r0[i] = 1;
            self.r1[i] = 1;
            self.g0[i] = 1;
            self.g1[i] = 1;
            self.b0[i] = 1;
            self.b1[i] = 1;
        }
    }
}

pub async fn random() {
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/v1/random")
        .body("")
        .send()
        .await;
    // println!("{:#?}", res);
}

pub async fn all_1() {
    let mut d = Data::new();
    d.all_1();
    let v = serde_json::to_string(&d).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/v1/matrix")
        .body(v)
        .send()
        .await;
}

pub fn listen() {

}