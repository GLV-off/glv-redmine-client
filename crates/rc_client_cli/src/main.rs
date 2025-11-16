#![allow(unused)]
#![allow(dead_code)]
pub mod cfg;
// Убрать после того как проект стабилизируется

pub use cfg::Cofnig;

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
