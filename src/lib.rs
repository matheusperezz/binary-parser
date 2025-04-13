use std::fs::File;
use std::io::{Read, Seek, Write};
use lame_sys::{lame_close, lame_encode_buffer_interleaved, lame_init, lame_init_params, lame_set_brate, lame_set_in_samplerate, lame_set_mode, lame_set_quality, MPEG_mode};

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

pub fn read_wav_header(file: &mut File) -> Result<WavHeader, std::io::Error> {
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

pub fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) -> Result<(), std::io::Error> {
    // Open the wav file
    let mut wav_file = File::open(wav_path)?;
    let mut wav_data = Vec::new();
    wav_file.read_to_end(&mut wav_data)?;

    // Initialize LAME encoder
    unsafe {
        let lame = lame_init();
        if lame.is_null() {
            return Err("Failed to initialize lame encoder".into());
        }

        // Set LAME parameters (e.g., sample rate, bitrate)
        lame_set_in_samplerate(lame, 44100);
        lame_set_brate(lame, 128);
        lame_set_mode(lame, MPEG_mode::STEREO);
        lame_set_quality(lame, 2);

        if lame_init_params(lame) < 0 {
            lame_close(lame);
            return Err("Failed to set LAME parameters".into());
        }

        // Prepare MP3 Output buffer
        let mut mp3_buffer = vec![0; wav_data.len()];

        // Encode WAV to MP3
        let wav_data_i16 = unsafe {
            std::slice::from_raw_parts(wav_data.as_ptr() as *mut i16, wav_data.len() / 2)
        };

        let mp3_size = lame_encode_buffer_interleaved(
            lame,
            wav_data_i16.as_ptr() as *mut std::os::raw::c_short,
            wav_data_i16.len() as i32 / 2,
            mp3_buffer.as_mut_ptr(),
            mp3_buffer.len() as i32,
        );

        if mp3_size < 0 {
            lame_close(lame);
            return Err("Failed to encode MP3".into());
        }

        // Write MP3 data to file
        let mut mp3_file = File::create(mp3_path)?;
        mp3_file.write_all(&mp3_buffer[..mp3_size as usize])?;

        // Finalize and close LAME
        lame_close(lame);
    }

    println!("MP3 file saved to {}", mp3_path);
    Ok(())
}