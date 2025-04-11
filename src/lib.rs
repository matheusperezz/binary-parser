use std::fs::File;
use std::io::{Read, Seek};

#[derive(Debug)]
struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    sub_chunk1_id: [u8; 4],
    sub_chunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    sub_chunk2_id: [u8; 4],
    sub_chunk2_size: u32,
}

fn read_wav_header(file: &mut File) -> Result<WavHeader, std::io::Error> {
    let mut header = [0u8; 44];
    file.read_exact(&mut header)?;

    Ok(WavHeader {
        chunk_id: header[0..4].iter().try_into().unwrap(),
        chunk_size: u32::from_le_bytes(header[4..8].iter().try_into().unwrap()),
        format: header[8..12].iter().try_into().unwrap(),
        sub_chunk1_id: header[12..16].iter().try_into().unwrap(),
        sub_chunk1_size: u32::from_le_bytes(header[16..20].try_into().unwrap()),
        audio_format: u16::from_le_bytes(header[20..22].try_into().unwrap()),
        num_channels: u16::from_le_bytes(header[22..24].try_into().unwrap()),
        sample_rate: u32::from_le_bytes(header[24..28].try_into().unwrap()),
        byte_rate: u32::from_le_bytes(header[28..32].try_into().unwrap()),
        block_align: u16::from_le_bytes(header[32..34].try_into().unwrap()),
        bits_per_sample: u16::from_le_bytes(header[34..36].try_into().unwrap()),
        sub_chunk2_id: header[36..40].try_into().unwrap(),
        sub_chunk2_size: u32::from_le_bytes(header[40..44].try_into().unwrap()),
    })
}