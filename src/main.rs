#![doc(html_root_url = "https://docs.rs/crate/dxlib_sample_0001/0.0.1")]
//! sample dxlib dll for Rust
//!

use std::error::Error;

use dxlib::{*, dx::*};

pub fn main() -> Result<(), Box<dyn Error>> {
  dum_screen();
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use dxlib::dum_screen;

  /// test a
  #[test]
  fn test_a() {
    assert_eq!(dum_screen(), ());
  }
}
