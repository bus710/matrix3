// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

const ADDR_MATRIX: u16 = 0x0046;
const LEN: usize = 193;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!: {}", DeviceInfo::new()?.model());

    // https://users.rust-lang.org/t/questions-about-mut-t-and-move-semantics-mut-t-is-move-only/37484/13
    // let mut sh= I2c::new()?;
    // let sensehat_matrix = &mut sh;
    let sensehat_matrix = &mut I2c::new()?;

    sensehat_matrix.set_slave_address(ADDR_MATRIX)?;
    // println!("{:?}", sensehat_matrix);

    let res = write_data(sensehat_matrix, 10);
    println!("{:?}", res);
    let res = write_data(sensehat_matrix, 20);
    println!("{:?}", res);
    let res = write_data(sensehat_matrix, 0);
    println!("{:?}", res);

    println!("Bye!");
    Ok(())
}

fn write_data(port: &mut I2c, level: u8) -> Result<usize, rppal::i2c::Error> {
    let mut data: [u8; LEN] = [0; LEN];

    thread::sleep(Duration::from_millis(1000));

    // https://dev.to/anilkhandei/mutable-arrays-in-rust-1k5o
    for (_, v) in data.iter_mut().enumerate() {
        *v = level;
    }
    data[0] = 0;
    data[1] = 63;
    data[10] = 63;
    data[19] = 63;
    let res = port.write(&data);
    res
}
