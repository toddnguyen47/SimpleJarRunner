mod exec_jar;
use exec_jar::ExecJar;

use std::env;

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  let exec_jar_obj = ExecJar::new();
  exec_jar_obj.execute(&args);
}
