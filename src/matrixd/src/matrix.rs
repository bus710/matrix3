use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

const ADDR_MATRIX: u16 = 0x0046;
const LEN: usize = 193;

pub struct SenseHat<'a> {
    matrix: &'a mut I2c,
}

impl<'a> SenseHat<'a> {
    pub fn new() -> Self {
        // println!("Device Info: {}", DeviceInfo::new()?.model());

        let matrix = &mut I2c::new()?;
        matrix.set_slave_address(ADDR_MATRIX)?;

        // let res = write_data(matrix, 10);
        // println!("{:?}", res);
        // let res = write_data(matrix, 20);
        // println!("{:?}", res);
        // let res = write_data(matrix, 0);
        // println!("{:?}", res);

        println!("Bye!");

        SenseHat { matrix }
    }

    pub fn write_data(&mut self, port: &mut I2c, level: u8) -> Result<usize, rppal::i2c::Error> {
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
}
