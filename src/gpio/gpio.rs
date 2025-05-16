use rppal::gpio;

use napi_derive::napi;
use napi::Result;

use super::pin::Pin;

#[napi]
pub struct Gpio(gpio::Gpio);

#[napi]
impl Gpio {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    // Attempt to create a new GPIO instance
    let gpio = gpio::Gpio::new()
      .expect("Failed to initialize GPIO");

    // Return the Gpio instance wrapped in the Gpio struct
    Ok(Gpio(gpio))
  }
}

#[napi]
impl Gpio {
  /**
   * Get a GPIO pin by its number.
  */
  #[napi]
  pub fn get(&self, pin: u8) -> Result<Pin> {
    // Get the GPIO pin using the provided pin number
    let base = self.0.get(pin)
      .expect("Failed to get GPIO pin");

    // Return the Pin instance wrapped in the Pin struct
    Ok(Pin::new(base))
  }
}