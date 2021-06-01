// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

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

    let d = matrix::Data::new();

    // let mut sh = match SenseHat::new() {
    //     Ok(v) => v,
    //     Err(e) => panic!("{}", e.to_string()),
    // };

    // sh.write_data(10).unwrap();
    // sh.write_data(20).unwrap();
    // sh.write_data(0).unwrap();

    // sh.run().await;

    println!("Bye!");
}
