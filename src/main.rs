use clap::{App, Arg};

fn main() {
  ffmpeg::init().unwrap();
  let cli = App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .arg(Arg::with_name("input").short("i").required(true).takes_value(true))
    .get_matches();
  if let Some(i) = cli.value_of("input") {
    println!("input: {}", i);
  }
}
