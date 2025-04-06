use std::fmt::format;

pub trait RedmineApi {
  
}

pub struct RedmineApiImpl {
  host: String,
  token: String
}

impl RedmineApiImpl {
  pub fn new() -> Box<dyn RedmineApi> {
    Box::new(Self{
      host: String::from(""),
      token: String::from("")
    })
  }
}

impl RedmineApi for RedmineApiImpl {

}