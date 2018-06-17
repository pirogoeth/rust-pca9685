//
// pub constants.rs
//
// Useful pub constants specific to the PCA9685.
//

// Default I2C bus address for the PCA9685
pub const I2C_BASE_ADDRESS: u8 = 0x40;

// PCA9685 internal clock oscillation frequency
pub const OSCILLATION_FREQ: f32 = 25000000.0;

// Number of steps available on a channel
pub const STEP_SIZE: f32 = 4096.0;

//
// REGISTERS
//

// Mode registers
pub const MODE_1: u8 = 0x00;
pub const MODE_2: u8 = 0x01;

// I2C bus subaddresses
pub const SUBADDR_1: u8 = 0x02;
pub const SUBADDR_2: u8 = 0x03;
pub const SUBADDR_3: u8 = 0x04;
pub const PRE_SCALE: u8 = 0xFE;

// Base values for a single LED register
pub const BASE_LED_ON_LOW: u8 = 0x06;
pub const BASE_LED_ON_HIGH: u8 = 0x07;
pub const BASE_LED_OFF_LOW: u8 = 0x08;
pub const BASE_LED_OFF_HIGH: u8 = 0x09;

// Registers for controlling state of all LEDs
pub const ALL_LED_ON_LOW: u8 = 0xFA;
pub const ALL_LED_ON_HIGH: u8 = 0xFB;
pub const ALL_LED_OFF_LOW: u8 = 0xFC;
pub const ALL_LED_OFF_HIGH: u8 = 0xFD;

//
// COMMAND BITS
//

// Tell the controller to persist the register values when SLEEP mode is set
// (MODE_1 register -- bit 7 is set)
pub const RESTART: u8 = 0x80;

// Signal a sleep to the controller, puts the oscillator in a low-power state
// and turns off the oscillator (MODE_1 register -- bit 4 is set)
pub const SLEEP: u8 = 0x10;

// Signal for the controller to either output change on STOP command (default) or
// output change on ACK (MODE_2 register -- bit 3 is set)
pub const OUTPUT_CHANGE: u8 = 0x0C;

pub const ALL_CALL: u8 = 0x01;
pub const INVRT: u8 = 0x10;
pub const OUTDRV: u8 = 0x04;