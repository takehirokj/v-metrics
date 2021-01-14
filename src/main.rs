use clap::{App, Arg};
use ffmpeg::{format, frame, media::Type};

fn get_bitrates<P: AsRef<str>>(input_path: P) -> Result<Vec<i32>, String> {
  ffmpeg::init().map_err(|e| e.to_string())?;
  let mut ictx = format::input(&input_path).map_err(|e| e.to_string())?;
  let input = ictx
    .streams()
    .best(Type::Video)
    .ok_or_else(|| "Failed to find video stream".to_string())?;
  let input_stream_idx = input.index();
  let mut decoder =
    input.codec().decoder().video().map_err(|e| e.to_string())?;
  decoder.set_parameters(input.parameters()).map_err(|e| e.to_string())?;

  let mut decoded_frame = frame::Video::empty();
  let mut packets = ictx.packets();
  let mut bitrates = Vec::new();
  while let Some(Ok((stream, packet))) = packets.next() {
    if stream.index() == input_stream_idx {
      let res = decoder.decode(&packet, &mut decoded_frame);
      if res.is_ok() {
        let bitrate = decoded_frame.packet().size as i32;
        bitrates.push(bitrate);
      }
    }
  }

  Ok(bitrates)
}

fn main() {
  let cli = App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .arg(Arg::with_name("input").short("i").required(true).takes_value(true))
    .get_matches();
  let input_path = cli.value_of("input").unwrap();
  let bitrates = match get_bitrates(&input_path) {
    Ok(res) => res,
    Err(msg) => {
      println!("Failed: {}", msg);
      return;
    }
  };
  for b in bitrates {
    print!("{}, ", b);
  }
}
