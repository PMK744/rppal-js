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

  /**
   * Sets the GPIO output pin to PWM mode with the specified frequency and duty cycle.
   * @param frequency The frequency of the PWM signal in Hz.
   * @param duty_cycle The duty cycle of the PWM signal as a fraction (0.0 to 1.0).
  */
  #[napi]
  pub fn set_pwm_frequency(&mut self, frequency: f64, duty_cycle: f64) {
    // Set the PWM frequency for the output pin
    self.0.set_pwm_frequency(frequency, duty_cycle)
      .expect("Failed to set PWM frequency");
  }

  /**
   * Clears the PWM settings for the GPIO output pin.
  */
  #[napi]
  pub fn clear_pwm(&mut self) {
    // Clear the PWM settings for the output pin
    self.0.clear_pwm()
      .expect("Failed to clear PWM settings");
  }
}