// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

mod matrix;

use crate::matrix::SenseHat;

fn main() {
    println!("Hello, world!");

    let mut sh = match SenseHat::new() {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };

    sh.write_data(10).unwrap();
    sh.write_data(20).unwrap();
    sh.write_data(0).unwrap();

    println!("Bye!");
}
