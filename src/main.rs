extern crate nom;
use std::io;
use std::{fs::File, io::Read};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;
use nom::IResult;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Wav {
    file_head: String,
    file_len: u32,
    file_type: String,
    file_format: String,
    header_len: u32,
    data_type: u16,
    channels: u16,
    sample_rate: u16,
    bitrate1: u32,
    bitrate2: u32,
    bits_sample: u16,
    dataformat: String,
    data_size: u32,
}

fn main() -> io::Result<()> {
    // Open the input file
    let mut f = File::open("0.wav")?;

    // Read the entire contents of the file into memory
    let mut buf = Vec::<u8>::new();
    f.read_to_end(&mut buf)?;

    // Parse the buffer using our parser
    match parse_riff(&buf) {
        Ok((_, wave)) => println!("Parsed Wave data:\n{:?}", wave),
        Err(e) => panic!("Error parsing WAVE data: {}", e),
    }

    Ok(())
}

fn parse_riff(input: &[u8]) -> IResult<&[u8], Wav> {
    let (input, data) = tuple((tag(b"RIFF"), le_u32))(input)?;
    let (input, iwav) = alt((parse_wave_chunk, parse_unknown_chunk))(input)?;
    Ok((
        input,
        Wav {
            file_head: "RIFF".to_string(),
            file_len: data.1,
            ..iwav
        },
    ))
}

fn parse_wave_chunk(input: &[u8]) -> IResult<&[u8], Wav> {
    let (input, _) = tag(b"WAVE")(input)?;
    let (input, iwav) = parse_wave_fmt(input)?;
    Ok((
        input,
        Wav {
            file_type: "WAVE".to_string(),
            ..iwav
        },
    ))
}

fn parse_wave_fmt(input: &[u8]) -> IResult<&[u8], Wav> {
    let (input, _) = tag(b"fmt ")(input)?;
    let (input, parsed_data) = tuple((
        le_u32, // Length of format data as listed above
        le_u16, // Type of format (1 is PCM)
        le_u16, // Number of channels
        le_u16, // Sample Rate
        le_u32, // (Sample rate * BPS * Channels)/8
        le_u32, // (BitsPerSample * Channels) / 8.1
        le_u16, // Bits per sample
        tag(b"data"),
        le_u32,
    ))(input)?;

    println!("len field {}", parsed_data.0);

    Ok((
        input,
        Wav {
            file_format: "fmt ".to_string(),
            header_len: parsed_data.0,
            data_type: parsed_data.1,
            channels: parsed_data.2,
            sample_rate: parsed_data.3,
            bitrate1: parsed_data.4,
            bitrate2: parsed_data.5,
            bits_sample: parsed_data.6,
            // dataformat: std::str::from_utf8(len.7).unwrap().to_string(),
            dataformat: "data".to_string(),
            data_size: parsed_data.8,
            ..Default::default()
        },
    ))
}

fn parse_unknown_chunk(input: &[u8]) -> IResult<&[u8], Wav> {
    let (input, _) = tag(b"UNKN")(input)?;
    Ok((
        input,
        Wav {
            ..Default::default()
        },
    ))
}
