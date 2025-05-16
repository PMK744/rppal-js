use rppal::gpio;

use napi_derive::napi;

use super::level::Level;

#[napi]
pub struct OutputPin(gpio::OutputPin);

impl OutputPin {
  pub fn new(pin: gpio::OutputPin) -> Self {
    OutputPin(pin)
  }
}

#[napi]
impl OutputPin {
  /**
   * The pin number of the GPIO output pin.
  */
  #[napi(getter)]
  pub fn get_pin(&self) -> u8 {
    // Get the GPIO pin number
    self.0.pin()
  }

  /**
   * Sets the GPIO output pin to the specified value.
  */
  #[napi(setter)]
  pub fn set_state(&mut self, level: Level) {
    // Set the output pin to the specified value
    self.0.write(level.into());
  }

  /**
   * Whether the GPIO output pin is set to low.
  */
  #[napi(getter)]
  pub fn get_is_set_low(&self) -> bool {
    // Check if the output pin is set to low
    self.0.is_set_low()
  }

  /**
   * Whether the GPIO output pin is set to high.
  */
  #[napi(getter)]
  pub fn get_is_set_high(&self) -> bool {
    // Check if the output pin is set to high
    self.0.is_set_high()
  }

  /**
   * Sets the GPIO output pin to low.
  */
  #[napi]
  pub fn set_low(&mut self) {
    // Set the output pin to low
    self.0.set_low();
  }

  /**
   * Sets the GPIO output pin to high.
  */
  #[napi]
  pub fn set_high(&mut self) {
    // Set the output pin to low
    self.0.set_high();
  }

  /**
   * Toggles the GPIO output pin state.
  */
  #[napi]
  pub fn toggle(&mut self) {
    // Toggle the output pin state
    self.0.toggle();
  }
}