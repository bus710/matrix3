use std::sync::Arc;

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;
use tokio::sync::Mutex;

const ADDR_MATRIX: u16 = 0x0046;
const I2C_DATA_LEN: usize = 193;
const COLOR_DATA_LEN: usize = 64;

#[derive(Clone)]
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
    matrix_rx: crossbeam_channel::Receiver<Data>,
    ws_tx: crossbeam_channel::Sender<Data>,
    signal_rx: crossbeam_channel::Receiver<()>,
}

impl SenseHat {
    pub fn new(
        signal_rx: crossbeam_channel::Receiver<()>,
        matrix_rx: crossbeam_channel::Receiver<Data>,
        ws_tx: crossbeam_channel::Sender<Data>,
    ) -> Result<SenseHat, String> {
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
        Ok(SenseHat {
            matrix: r,
            buffer: [0; I2C_DATA_LEN],
            matrix_rx,
            ws_tx,
            signal_rx,
        })
    }

    fn write_data(&mut self, mut data: Data) -> Result<usize, String> {
        // Normalize
        data.r.iter_mut().find(|&&mut v| v > 64).map(|_v| 63);
        data.g.iter_mut().find(|&&mut v| v > 64).map(|_v| 63);
        data.b.iter_mut().find(|&&mut v| v > 64).map(|_v| 63);

        // Map from 64*3 to R8/G8/B8 order
        // - buffer[ 1.. 9] <= r[0..8]
        // - buffer[10..18] <= g[0..8]
        // - buffer[19..27] <= b[0..8]
        // - buffer[28..36] <= r[9..17]
        let mut _j = 0;
        for (i, _) in data.r.iter().enumerate() {
            _j = (i / 8) * 8;
            _j = _j * 2;
            self.buffer[i + _j + 1] = data.r[i];
            self.buffer[i + _j + 9] = data.g[i];
            self.buffer[i + _j + 17] = data.b[i];
        }

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
    pub fn new(
        matrix_rx: crossbeam_channel::Receiver<Data>,
        ws_tx: crossbeam_channel::Sender<Data>,
        signal_rx: crossbeam_channel::Receiver<()>,
    ) -> Result<SenseHatRunner, String> {
        // Create a new SenseHat instance
        let sh = match SenseHat::new(signal_rx, matrix_rx, ws_tx) {
            Ok(v) => v,
            Err(e) => panic!("{}", e.to_string()),
        };
        // Wrap the instance with mutex, rc, and self
        Ok(SenseHatRunner {
            sense_hat: Arc::new(Mutex::new(sh)),
        })
    }

    // pub async fn get_matrix_tx(&mut self) -> crossbeam_channel::Sender<Data> {
    //     let sh = self.sense_hat.clone();
    //     let sh = sh.lock().await;
    //     sh.tx.clone()
    // }

    pub async fn run(&mut self) -> Result<tokio::task::JoinHandle<()>, String> {
        // Clone RC
        let sh = self.sense_hat.clone();
        // Spawn
        let handle = tokio::task::spawn(async move {
            // Lock
            let mut sh = sh.lock().await;
            // Loop
            loop {
                crossbeam_channel::select! {
                    recv(sh.matrix_rx) -> v => {
                        match v {
                            Ok(v) => {
                                let _ = sh.write_data(v.clone()).unwrap();
                                let _ = sh.ws_tx.send(v);
                            },
                            Err(_) => (),
                        };
                    },
                    recv(sh.signal_rx) -> _ => {
                        let d = Data::new();
                        sh.write_data(d).unwrap();
                        break
                    },
                }
            }
        });
        Ok(handle)
    }
}
