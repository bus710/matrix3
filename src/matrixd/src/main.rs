// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use crate::matrix::SenseHat;
// use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    println!("Hello, world!");

    let _ = SenseHat::new();
    // let sh = match sh {
    //     Ok(r) => r,
    //     Err(e) => panic!("Error: {:?}", e),
    // };

    // sh.write_data(10);

    println!("Bye!");
}
