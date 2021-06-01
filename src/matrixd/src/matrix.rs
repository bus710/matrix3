use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

use crossbeam_channel::unbounded;
use tokio::sync::Mutex;

const ADDR_MATRIX: u16 = 0x0046;
const I2C_DATA_LEN: usize = 193;
const COLOR_DATA_LEN: usize = 64;

#[derive(Debug)]
pub struct Data {
    pub r: [u8; COLOR_DATA_LEN],
    pub g: [u8; COLOR_DATA_LEN],
    pub b: [u8; COLOR_DATA_LEN],
}

impl Data {
    pub fn new() -> Self {
        Data {
            r: [0; COLOR_DATA_LEN],
            g: [0; COLOR_DATA_LEN],
            b: [0; COLOR_DATA_LEN],
        }
    }
}

struct SenseHat {
    matrix: I2c,
    buffer: [u8; I2C_DATA_LEN],
    tx: crossbeam_channel::Sender<Data>,
    rx: crossbeam_channel::Receiver<Data>,
}

impl SenseHat {
    pub fn new() -> Result<SenseHat, String> {
        // Check the platform
        let r = DeviceInfo::new();
        match r {
            Ok(v) => println!("{:?}", v),
            Err(e) => return Err(e.to_string()),
        };
        // Get an I2C handler
        let r = I2c::new();
        let r = match r {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        // Set the I2C address
        let mut r = r;
        let s = r.set_slave_address(ADDR_MATRIX);
        match s {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        // Set channels
        let (tx, rx) = unbounded();

        Ok(SenseHat {
            matrix: r,
            buffer: [0; I2C_DATA_LEN],
            tx,
            rx,
        })
    }

    fn write_data(&mut self, data: Data) -> Result<usize, String> {
        // Iterate over the R channel (0..63)
        // buffer[ 1.. 9] <= r[0..8]
        // buffer[10..18] <= g[0..8]
        // buffer[19..27] <= b[0..8]
        // buffer[28..36] <= r[9..17]

        let mut j;
        // https://dev.to/anilkhandei/mutable-arrays-in-rust-1k5o
        for (i, _) in data.r.iter().enumerate() {
            j = (i / 8) * 8;
            j = j * 2;
            self.buffer[i + j + 1] = data.r[i] / 4;
            self.buffer[i + j + 10] = data.g[i] / 4;
            self.buffer[i + j + 19] = data.b[i] / 4;
        }

        // self.buffer[0] = 0;
        // self.buffer[1] = 63;
        // self.buffer[10] = 63;
        // self.buffer[19] = 63;

        match self.matrix.write(&self.buffer) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct SenseHatRunner {
    sense_hat: Arc<Mutex<SenseHat>>,
}

impl SenseHatRunner {
    pub fn new() -> Result<SenseHatRunner, String> {
        //
        let sh = match SenseHat::new() {
            Ok(v) => v,
            Err(e) => panic!("{}", e.to_string()),
        };
        //
        Ok(SenseHatRunner {
            sense_hat: Arc::new(Mutex::new(sh)),
        })
    }

    pub async fn get_tx(&mut self) -> crossbeam_channel::Sender<Data> {
        let sh = self.sense_hat.clone();
        let sh = sh.lock().await;
        sh.tx.clone()
    }

    pub async fn run(&mut self) {
        // Clone RC
        let sh = self.sense_hat.clone();
        // Spawn
        tokio::task::spawn(async move {
            // Lock
            let mut sh = sh.lock().await;
            // Loop
            loop {
                let rv = sh.rx.recv();
                match rv {
                    Ok(v) => {
                        let _ = sh.write_data(v);
                        thread::sleep(Duration::from_millis(16));
                    }
                    Err(e) => println!("{}", e),
                }
            }
        })
        .await
        .unwrap();
    }
}
