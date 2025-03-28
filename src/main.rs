mod consts;
mod structs;
mod utils;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, fs, process};

use clap::Parser;
use colour::{green_ln, yellow_ln, red_ln};
use rayon::prelude::*;

use crate::structs::{Args, KeyResult};

fn parse_pattern(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split_whitespace()
        .map(|b| {
            if b == "?" {
                None
            } else {
                // We already know our patterns contain valid hex codes.
                u8::from_str_radix(b, 16).ok()
            }
        })
        .collect()
}

fn find_pattern(buffer: &[u8], pattern: &[Option<u8>]) -> Vec<usize> {
    buffer.par_windows(pattern.len())
        .enumerate()
        .filter_map(|(i, window)| {
            if pattern.iter().zip(window).all(|(&pat_byte, &buf_byte)| pat_byte.is_none() || Some(buf_byte) == pat_byte) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn key_is_false_positive(key: &str) -> bool {
    consts::FALSE_POSITIVES.contains(&key)
}

fn concat_aes_type(key_addr: &[u8], type_idx: usize) -> String {
    let mut final_hex_string = String::new();

    for &offset in &consts::KEY_DWORD_OFFSETS[type_idx] {
        let hex_string = utils::bytes_to_hex(&key_addr[offset..offset + 4]);
        final_hex_string.push_str(&hex_string);
    }

    final_hex_string.to_uppercase()
}

fn calculate_entropy(key_string: &str) -> f64 {
    let mut frequencies: HashMap<char, i32> = HashMap::new();

    for c in key_string.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    let num_len = key_string.len() as f64;
    let mut info_content = 0.0;

    for &count in frequencies.values() {
        let freq = count as f64 / num_len;
        info_content += freq * freq.log2();
    }

    -info_content
}

fn extract_keys(binary_path: &PathBuf, min_entropy: f64) -> Result<Vec<KeyResult>, Box<dyn Error>> {
    let data = fs::read(binary_path)?;

    let results: Mutex<Vec<KeyResult>> = Mutex::new(Vec::new());

    for (idx, pattern_str) in consts::PATTERNS.iter().enumerate() {
        let pattern = parse_pattern(pattern_str);
        let matches = find_pattern(&data, &pattern);

        matches.par_iter().for_each(|&offset| {
            let chunk = &data[offset..offset + pattern.len() + 8];

            let key = concat_aes_type(chunk, idx);
            let entropy = calculate_entropy(&key);

            if !key_is_false_positive(&key) && entropy >= min_entropy {
                let res = KeyResult {
                    key,
                    entropy,
                };

                if let Ok(mut results) = results.lock() {
                    results.push(res);
                }

            }
        });
    }

    let mut final_results = results
        .into_inner()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    final_results.sort_by(|a, b| {
        b.entropy.partial_cmp(&a.entropy).unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(final_results.clone())

}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.no_colour {
        unsafe {
            env::set_var("NO_COLOR", "1");
        }
    }

    if !args.json {
        utils::print_version();
    }

    let dropped = args.dropped_in_path.clone();
    let in_path = args.in_path.or(args.dropped_in_path).ok_or("ue binary path wasn't provided")?;

    if !utils::path_buf_ends_with(&in_path, ".exe") {
        eprintln!("Bad input file extension. Exiting for safety...");
        process::exit(1)
    }

    if !args.json {
        println!("{}", in_path.to_string_lossy());
    }

    let min_entropy = args.entropy.unwrap_or(3.0);
    let keys = extract_keys(&in_path, min_entropy)?;

    if args.json {
        let serialised_keys = serde_json::to_string_pretty(&keys)?;
        print!("{}", serialised_keys);
        return Ok(());
    }

    for key in &keys {
        let line = format!("Key: 0x{} | Entropy: {:.6}", key.key, key.entropy);
        match key.entropy {
            e if e >= 3.5 => green_ln!("{}", line),
            e if e >= 3.1 => yellow_ln!("{}", line),
            _ => red_ln!("{}", line),
        }
    }

    if keys.is_empty() && !args.json {
        println!("No keys were found. Unsupported UE version or game doesn't encrypt its PAKs.")
    }

    if dropped.is_some() {
        println!("\nDone. Press enter to exit...");
        utils::wait_for_exit()?;
    }

    Ok(())
}