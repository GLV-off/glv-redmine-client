// Убрать после того как проект стабилизируется
#![allow(unused)]
#![allow(dead_code)]

#[derive(Default)]
pub struct Config{
  pub host: String,
  pub token: String,
  pub username: String, 
  pub password: String
}

impl Config {
  pub fn assign(&mut self, cfg:&Config) -> Config {
    Config::default()
  }
}

fn main() {
  let mut cli_cfg = parse_cli_configuration();
  let mut env_cfg = load_env_configuration();
  let cfg = cli_cfg.assign(&env_cfg);
  run_app(cfg);
}

pub fn parse_cli_configuration() -> Config {
  Config::default()
}

pub fn load_env_configuration() -> Config {
  Config::default()
}

pub fn run_app(cfg: Config) {

}