use clap::{App, Arg};
use ffmpeg::{format, frame, media::Type};
use plotters::prelude::*;

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

fn draw_graph<P: AsRef<std::path::Path>>(
  datas: &[i32],
  output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
  let root = BitMapBackend::new(&output_path, (1024, 768)).into_drawing_area();
  root.fill(&WHITE)?;

  let max = *datas.iter().max().unwrap() as f64;
  let mut chart = ChartBuilder::on(&root)
    .set_label_area_size(LabelAreaPosition::Left, 80)
    .set_label_area_size(LabelAreaPosition::Bottom, 60)
    .build_cartesian_2d(0..(datas.len() - 1), 0.0..max * 1.2)?;
  chart
    .configure_mesh()
    .disable_x_mesh()
    .disable_y_mesh()
    .y_desc("bit")
    .x_desc("Frame no")
    .draw()?;

  chart.draw_series(
    AreaSeries::new(
      (0..).zip(datas.iter()).map(|(x, y)| (x, *y as f64)),
      0.0,
      &BLUE.mix(0.2),
    )
    .border_style(&BLUE),
  )?;

  Ok(())
}

fn main() -> Result<(), String> {
  let cli = App::new(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .arg(Arg::with_name("input").short("i").required(true).takes_value(true))
    .arg(Arg::with_name("output").short("o").required(true).takes_value(true))
    .get_matches();
  let input_path = cli.value_of("input").unwrap();
  let output_path = cli.value_of("output").unwrap();
  let bitrates = get_bitrates(&input_path)?;
  draw_graph(&bitrates, &output_path).map_err(|err| err.to_string())?;
  Ok(())
}
