extern crate nom;
use std::io;
use std::{fs::File, io::Read};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
use nom::IResult;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Wav {
    file_head: String,
    file_len: u32,
    file_type: String,
    file_format: String,
    chunk_len: u32,
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
    let (input, len) = le_u32(input)?;

    println!("len field {}", len);

    Ok((
        input,
        Wav {
            file_format: "fmt ".to_string(),
            chunk_len: len,
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
