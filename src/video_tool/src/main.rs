use ffmpeg;
use std::env;
use std::process;

fn main() {
    ffmpeg::init().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: {} <input file> <output file> <frame rate> <resolution>", args[0]);
        process::exit(1);
    }
    let input_file = &args[1];
    let output_file = &args[2];
    let frame_rate: f32 = args[3].parse().unwrap();
    let resolution: Vec<&str> = args[4].split('x').collect();
    let width: i32 = resolution[0].parse().unwrap();
    let height: i32 = resolution[1].parse().unwrap();

    //输入输出上下文
    let mut ictx = ffmpeg::format::input(&input_file).unwrap();
    let mut octx = ffmpeg::format::output(&output_file).unwrap();

    // gif输出流
    let mut stream = octx.add_stream(ffmpeg::encoder::find(ffmpeg::codec::Id::GIF).unwrap()).unwrap();

    // 帧率参数
    let time_base = ffmpeg::Rational::from((frame_rate.round() as i32, 1));
    stream.set_time_base(time_base);

    //设置编码器参数
    let codec = stream.codec().encoder().unwrap();
    codec.set_width(width);
    codec.set_height(height);
    codec.set_rate(frame_rate as i32);

    let mut output = octx.output().unwrap();
    output.write_header().unwrap();

    for (stream, packet) in ictx.packets() {
        output.interleaved_write_packet(&packet).unwrap();
    }

    output.write_trailer().unwrap();
}
