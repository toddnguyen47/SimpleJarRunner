mod exec_jar;
use exec_jar::ExecJar;

mod java_command;

use std::env;

use env_logger::Env;

fn main() {
  let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "trace")
    .write_style_or("MY_LOG_STYLE", "always");

  env_logger::init_from_env(env);

  let args: Vec<String> = env::args().skip(1).collect();
  let exec_jar_obj = ExecJar::new();
  exec_jar_obj.execute(&args);
}
