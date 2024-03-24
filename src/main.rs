#![doc(html_root_url = "https://docs.rs/crate/dxlib_sample_0001/0.0.4")]
//! sample dxlib dll for Rust
//!
//! see also https://docs.rs/dxlib/latest/dxlib/dx
//!
//! compile .hlsl to .vso and .pso by ShaderCompiler distributed with DxLib
//!
//! - ShaderCompiler /Tvs_4_0 shader_VS.hlsl
//! - ShaderCompiler /Tps_4_0 shader_PS.hlsl
//!

use std::error::Error;

use dxlib::demo;

pub fn main() -> Result<(), Box<dyn Error>> {
//  demo::typ::screen("./resource/")?;
  demo::dum::screen("./resource/");
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  // use super::*;
  use dxlib::demo;

  /// test screen
  #[test]
  fn test_screen() {
    // either typ or dum at once
//    assert_eq!(demo::typ::screen("./resource/").expect("init"), ());
    assert_eq!(demo::dum::screen("./resource/"), ());
  }
}
