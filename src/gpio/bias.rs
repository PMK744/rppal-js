use rppal::gpio;

use napi_derive::napi;

#[napi]
pub enum Bias {
  Off,
  PullDown,
  PullUp
}

// Conversion between rppal::gpio::Bias and Bias enum

impl From<gpio::Bias> for Bias {
  fn from(bias: gpio::Bias) -> Self {
    match bias {
      gpio::Bias::Off => Bias::Off,
      gpio::Bias::PullDown => Bias::PullDown,
      gpio::Bias::PullUp => Bias::PullUp
    }
  }
}

impl Into<gpio::Bias> for Bias {
  fn into(self) -> gpio::Bias {
    match self {
      Bias::Off => gpio::Bias::Off,
      Bias::PullDown => gpio::Bias::PullDown,
      Bias::PullUp => gpio::Bias::PullUp
    }
  } 
}