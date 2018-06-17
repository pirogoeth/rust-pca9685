extern crate rust_pca9685;

use rust_pca9685::led_register::{ LEDRegister };

#[test]
fn test_new_lr_over_max() {
    let result = LEDRegister::new(16);
    match result {
        Ok(_) => panic!("expected error, received ok"),
        Err(_) => return,
    }
}