#[derive(Default)]
pub struct Config{
  pub host: String,
  pub token: String,
  pub username: String, 
  pub password: String
}

impl Config {
  pub fn assign(&mut self, cfg:&Config) -> Config {
    self.host = cfg.host;
    self.token = cfg.token;
    self.username = cfg.username;
    self.password = cfg.password;
    Config::default()
  }
}

