// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use std::thread;
use std::time::Duration;
use crate::matrix::{Data, SenseHatRunner};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut sh_runner = match SenseHatRunner::new() {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };

    let tx = sh_runner.get_tx().await;

    sh_runner.run().await;

    let mut d = matrix::Data::new();
    for i in 0..63 {
        d.r[i] = 1;
        d.g[i] = 3;
        d.b[i] = 5;
    }

    let _ = match tx.send(d) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };

    thread::sleep(Duration::from_millis(1000));


    println!("Bye!");
}
