use rppal::gpio;

use napi::Result;
use napi_derive::napi;

use super::output_pin::OutputPin;
use super::input_pin::InputPin;

#[napi]
pub struct Pin {
  pin: Option<gpio::Pin>,
}

impl Pin {
  pub fn new(pin: gpio::Pin) -> Self {
    Pin { pin: Some(pin) }
  }
}

#[napi]
impl Pin {
  /**
   * The pin number of the GPIO input pin.
  */
  #[napi(getter)]
  pub fn get_pin(&self) -> u8 {
    // Get the GPIO pin number
    self.pin.as_ref().expect("Pin is already consumed").pin()
  }

  /**
   * Converts the GPIO pin to an `InputPin` and disables the pull-up/pull-down resistors.
  */
  #[napi]
  pub fn into_input(&mut self) -> Result<InputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an InputPin
    Ok(InputPin::new(pin.into_input()))
  }

  /**
   * Converts the GPIO pin to an `InputPin` and enables the pull-down resistor.
  */
  #[napi]
  pub fn into_input_pulldown(&mut self) -> Result<InputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an InputPin with pull-down resistor
    Ok(InputPin::new(pin.into_input_pulldown()))
  }

  /**
   * Converts the GPIO pin to an `InputPin` and enables the pull-up resistor.
  */
  #[napi]
  pub fn into_input_pullup(&mut self) -> Result<InputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an InputPin with pull-up resistor
    Ok(InputPin::new(pin.into_input_pullup()))
  }

  /**
   * Converts the GPIO pin to an `OutputPin`.
  */
  #[napi]
  pub fn into_output(&mut self) -> Result<OutputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an OutputPin
    Ok(OutputPin::new(pin.into_output()))
  }

  /**
   * Converts the GPIO pin to an `OutputPin` and sets the initial state to low.
  */
  #[napi]
  pub fn into_output_low(&mut self) -> Result<OutputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an OutputPin with initial state low
    Ok(OutputPin::new(pin.into_output_low()))
  }

  /**
   * Converts the GPIO pin to an `OutputPin` and sets the initial state to high.
  */
  #[napi]
  pub fn into_output_high(&mut self) -> Result<OutputPin> {
    // Take the pin from the Option
    let pin = self.pin.take().expect("Pin is already consumed");

    // Convert the pin to an OutputPin with initial state high
    Ok(OutputPin::new(pin.into_output_high()))
  }
}
