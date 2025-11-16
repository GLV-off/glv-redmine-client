pub struct Api {
  host: String
}

pub struct Issue {

}

pub struct Issues{
  pub items: Vec<Issue>
}

impl Api {
  pub fn new() -> Self {
    Self{
      host: String::from("https://redmine.org")
    }
  }

  pub fn get_issues(&mut self) -> Issues {
    let r = ureq::get(self.host.clone() + "/issues.json")
      .call()
      .and_then(|mut x| 
        x.body_mut()
        .read_to_string()
      )
      .map(|r| r.to_string());   
    Issues{
      items: vec![]
    }
  }
}