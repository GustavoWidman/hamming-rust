use std::time::Instant;

use bitvec::prelude::*;

fn compute_parity_positions(k: usize) -> usize {
    println!("Data bits: {}", k);
    // We use positions 1, 10, 100, etc. (in binary) as the error-correcting bits
    let mut r = 0;
    // shift 00000001 to the left by r bits (how many parity bits we expect to have by now)
    // compare to the length of our final bitvec if we were to add this and all the other parity bits
    while (1 << r) < (k + r + 1) {
        r += 1;
    }
    println!("Parity bit count: {}", r);
    r
}

/// Encode the given data using Hamming code.
/// The data is provided as a BitVec and the output codeword contains both data and parity bits.
fn encode_hamming(data: &BitVec) -> BitVec {
    let k = data.len();
    let r = compute_parity_positions(k);
    let total_len = k + r;
    // Initialize codeword with all bits set to false (0).
    let mut codeword = bitvec![0; total_len];

    // Insert data bits into codeword at positions that are not powers of two.
    let mut data_index = 0;
    for i in 0..total_len {
        // Positions that are powers of 2 (i.e. indices 0,1,3,7,...) are reserved for parity bits.
        if (i + 1).is_power_of_two() {
            continue;
        }
        codeword.set(i, data[data_index]);
        data_index += 1;
    }

    // Calculate parity bits for each parity position.
    for i in 0..r {
        // Parity bit position (using 0-indexed; i.e. 1<<i position corresponds to index (1<<i)-1).
        let parity_pos = (1 << i) - 1;
        let mut parity = false;
        // For each bit in the codeword, if its (1-indexed) position has the i-th bit set, include it in parity.
        for j in 0..total_len {
            if ((j + 1) & (1 << i)) != 0 {
                parity ^= codeword[j];
            }
        }
        codeword.set(parity_pos, parity);
    }
    codeword
}

/// Decode the Hamming code, correct a single-bit error if present, and extract the original data.
/// Returns the data as a BitVec and the position (1-indexed) of the detected error (or 0 if none).
fn decode_hamming(codeword: &BitVec) -> (BitVec, usize) {
    let n = codeword.len();
    // Determine the number of parity bits, r, such that 2^r >= n + 1.
    let r = (0..).find(|&r| (1 << r) >= n + 1).unwrap();
    let mut error_pos = 0;

    // Check parity for each parity bit.
    for i in 0..r {
        let mut parity = false;
        for j in 0..n {
            if ((j + 1) & (1 << i)) != 0 {
                parity ^= codeword[j];
            }
        }
        if parity {
            // If parity is non-zero, add the corresponding value.
            error_pos += 1 << i;
        }
    }

    // Correct the error if one was detected (error_pos is 1-indexed).
    let mut corrected = codeword.clone();
    if error_pos > 0 && error_pos - 1 < corrected.len() {
        // corrected.toggle(error_pos - 1);
        let current = corrected[error_pos - 1];
        corrected.set(error_pos - 1, !current);
    }

    // Extract the data bits (positions that are not powers of two).
    let mut data = BitVec::new();
    for i in 0..corrected.len() {
        if !(i + 1).is_power_of_two() {
            data.push(corrected[i]);
        }
    }
    (data, error_pos)
}

fn main2() {
    // Example data: a BitVec of bits (here we use 30 bits for illustration).
    // deserialize from a bitstring
    let stdin_input = "1011010110101101011010110101101010010100101001101010010101010101010101001011010000000000111111111111111111111111101000000001010101010100100100010010101110101110100";

    let time_start = Instant::now();
    let bytes = bytestring_to_bitvec(stdin_input).unwrap();
    let time_end = Instant::now();
    println!("Time:      {:?}", time_end - time_start);
    println!("Bytes:     {:?}", bytes);

    let data: BitVec = BitVec::from_iter(bytes);
    println!("Data:      {:?}", data);

    let time_start = Instant::now();
    let encoded = encode_hamming(&data);
    let time_end = Instant::now();
    println!("Time:      {:?}", time_end - time_start);
    println!("Encoded:   {:?}", encoded);

    // Introduce a single-bit error for demonstration.
    let mut corrupted = encoded.clone();
    // Toggle the bit at position 3 (0-indexed) as an example error.
    corrupted.set(2, false);
    println!("Corrupted: {:?}", corrupted);

    let time_start = Instant::now();
    let (decoded, error_pos) = decode_hamming(&corrupted);
    let time_end = Instant::now();
    println!("Time:      {:?}", time_end - time_start);
    println!("Decoded:   {:?}, Error Position: {}", decoded, error_pos);
}

fn main() {
    // Example data: a BitVec of bits (here we use 30 bits for illustration).
    // deserialize from a bitstring
    let stdin_input = bytes_to_bits("hello".as_bytes());
}

fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
    bytes
        .iter()
        .map(|byte| byte_to_bits(*byte))
        .flatten()
        .collect()
}

fn byte_to_bits(byte: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[7 - i] = (byte >> i) & 1 == 1;
    }

    bits
}

fn bytestring_to_bitvec(s: &str) -> anyhow::Result<Vec<bool>> {
    s.chars().fold(Ok(Vec::with_capacity(s.len())), |acc, c| {
        acc.and_then(|mut vec| match c.to_digit(2).map(|digit| digit != 0) {
            Some(digit) => {
                vec.push(digit);
                Ok(vec)
            }
            None => Err(anyhow::anyhow!("Invalid digit: {}", c)),
        })
    })
}
