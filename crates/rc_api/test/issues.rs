use rc_api::api::RedmineApi;

use googletest::prelude::*;
use ureq;

#[test]
fn testing() {
  assert_eq!(2,2);
}

#[gtest]
fn check_work_of_library() -> Result<(), ureq::Error> {
  let mut host = String::from("https://redmine.org/issues.json");
  let body: String = ureq::get(host)
      .call()?
      .into_string()?;
  assert_that!(body, empty());

  Ok(())
}