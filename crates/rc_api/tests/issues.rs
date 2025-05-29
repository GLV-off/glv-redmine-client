use googletest::prelude::*;
use ureq;

#[gtest]
fn nothing() -> std::result::Result<(), ureq::Error> {

  Ok(())
}