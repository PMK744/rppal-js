use rppal::gpio;

use napi_derive::napi;

use super::{bias::Bias, level::Level};

#[napi]
pub struct InputPin(gpio::InputPin);

impl InputPin {
  pub fn new(pin: gpio::InputPin) -> Self {
    InputPin(pin)
  }
}

#[napi]
impl InputPin {
  /**
   * The pin number of the GPIO input pin.
  */
  #[napi(getter)]
  pub fn get_pin(&self) -> u8 {
    // Get the GPIO pin number
    self.0.pin()
  }

  /**
   * The state of the GPIO input pin.
  */
  #[napi(getter)]
  pub fn get_state(&self) -> Level {
    // Get the state of the input pin
    self.0.read().into()
  }

  /**
   * Whether the GPIO input pin is set to low.
  */
  #[napi(getter)]
  pub fn get_is_set_low(&self) -> bool {
    // Check if the input pin is set to low
    self.0.is_low()
  }

  /**
   * Whether the GPIO input pin is set to high.
  */
  #[napi(getter)]
  pub fn get_is_set_high(&self) -> bool {
    // Check if the input pin is set to high
    self.0.is_high()
  }

  /**
   * Configures the built-in pull-up or pull-down resistor for the GPIO input pin.
  */
  #[napi]
  pub fn set_bias(&mut self, bias: Bias) {
    // Set the bias for the input pin
    self.0.set_bias(bias.into());
  }
}