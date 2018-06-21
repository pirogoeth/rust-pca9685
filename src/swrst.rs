use env_logger;
use i2cdev::core::I2CDevice;
use i2cdev::linux::*;

use ::constants;

pub fn software_reset(dev: &mut LinuxI2CDevice, slave_addr: Option<u16>) -> Result<(), LinuxI2CError> {
    let _ = env_logger::try_init();

    let slave_addr = slave_addr.unwrap_or(constants::PCA9685_SLAVE_ADDRESS);

    // Switch slave address to the I2C master address
    debug!("switching to master communication, sending soft reset");
    dev.set_slave_address(constants::I2C_MASTER_ADDRESS);

    // Get the result from writing SOFT_RESET to master
    let result = dev.smbus_write_byte(constants::SOFT_RESET);

    // Revert to requested slave address after reset
    debug!("reverting to slave address {:#x}", slave_addr);
    dev.set_slave_address(slave_addr);

    return result;
}
