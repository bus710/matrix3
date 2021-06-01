// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use crate::matrix::SenseHatRunner;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut sh_runner = match SenseHatRunner::new() {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };

    let tx = sh_runner.get_tx().await;


    thread::sleep(Duration::from_millis(1000));

    let mut d = matrix::Data::new();
    for i in 0..64 {
        d.r[i] = 1;
        d.g[i] = 3;
        d.b[i] = 10;
    }

    let _ = match tx.send(d) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };

    sh_runner.run().await;
    thread::sleep(Duration::from_millis(1000));

    println!("Bye!");
}
