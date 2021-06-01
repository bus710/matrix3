use std::sync::Arc;
use std::sync::Mutex;
// use std::thread;
// use std::time::Duration;

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

const ADDR_MATRIX: u16 = 0x0046;
const DATA_LEN: usize = 193;

pub struct SenseHat<'a> {
    matrix: Arc<Mutex<&'a mut I2c>>,
}

impl<'a> SenseHat<'a> {
    pub fn new() -> SenseHat<'a> {
        let r = DeviceInfo::new();
        match r {
            Ok(v) => println!("Device Info: {:?}", v),
            Err(_) => panic!(),
        };

        let r = &mut I2c::new();
        let r = match r {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let s = r.set_slave_address(ADDR_MATRIX);
        match s {
            Ok(_) => (),
            Err(_) => panic!(),
        }

        SenseHat {
            matrix: Arc::new(Mutex::new(r)),
        }
    }

    // pub fn write_data(&mut self, level: u8) -> Result<usize, rppal::i2c::Error> {
    //     let mut data: [u8; DATA_LEN] = [0; DATA_LEN];

    //     thread::sleep(Duration::from_millis(1000));

    //     // https://dev.to/anilkhandei/mutable-arrays-in-rust-1k5o
    //     for (_, v) in data.iter_mut().enumerate() {
    //         *v = level;
    //     }
    //     data[0] = 0;
    //     data[1] = 63;
    //     data[10] = 63;
    //     data[19] = 63;
    //     let res = self.matrix.write(&data);
    //     res
    // }
}
