use rppal::gpio;

use napi_derive::napi;

#[napi]
pub enum Level {
  Low = 0,
  High = 1,
}

// Conversion between rppal::gpio::Level and Level enum

impl From<gpio::Level> for Level {
  fn from(level: gpio::Level) -> Self {
    match level {
      gpio::Level::Low => Level::Low,
      gpio::Level::High => Level::High
    }
  }
}

impl Into<gpio::Level> for Level {
  fn into(self) -> gpio::Level {
    match self {
      Level::Low => gpio::Level::Low,
      Level::High => gpio::Level::High
    }
  } 
}