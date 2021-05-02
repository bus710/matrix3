// https://github.com/bus710/matrix2/blob/master/src/back/mainSenseHat.go
// https://github.com/golemparts/rppal
// https://github.com/golemparts/rppal/blob/master/examples/i2c_ds3231.rs
// https://www.raspberrypi.org/documentation/hardware/sense-hat/README.md
// https://pinout.xyz/pinout/sense_hat

use std::error::Error;
use std::thread;
use std::time::Duration;

// use rppal::gpio::Gpio;
// use rppal::pwm::{Channel, Pwm};
// use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
// use rppal::uart::{Parity, Uart};

use rppal::i2c::I2c;
use rppal::system::DeviceInfo;

const ADDR_MATRIX: u16 = 0x0046;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!: {}", DeviceInfo::new()?.model());

    let mut sense_hat_matrix = I2c::new()?;
    println!("{:?}", sense_hat_matrix);
    // I2c { bus: 1, funcs: Capabilities {
    // addr_10bit: false,
    // i2c_block_read: true,
    // i2c_block_write: true,
    // smbus_quick_command: true,
    // smbus_receive_byte: true,
    // smbus_send_byte: true,
    // smbus_read_byte: true,
    // smbus_write_byte: true,
    // smbus_read_word: true,
    // smbus_write_word: true,
    // smbus_process_call: true,
    // smbus_block_read: false,
    // smbus_block_write: true,
    // smbus_block_process_call: false,
    // smbus_pec: true,
    // smbus_host_notify: false },
    // i2cdev: File { fd: 3, path: "/dev/i2c-1", read: true, write: true },
    // addr_10bit: false, address: 0, not_sync: PhantomData }

    sense_hat_matrix.set_slave_address(ADDR_MATRIX)?;

    println!("Bye!");
    Ok(())
}
