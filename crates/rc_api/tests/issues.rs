use rc_api::{Api, Issue, Issues};
use googletest::prelude::*;
use ureq;

#[gtest]
fn get_issues() -> std::result::Result<(), ureq::Error> {
  let mut api = Api::new();
  let issues = api.get_issues();
  
  assert_that!(issues.items.len(), gt(0));

  Ok(())
}