pub fn string_to_bits(s: &str) -> Vec<bool> {
    let mut result = Vec::with_capacity(s.len() * 8);
    for byte in s.as_bytes() {
        for i in (0..8).rev() {
            result.push((byte >> i) & 1 == 1);
        }
    }

    result
}

pub fn bits_to_string(bits: &[bool]) -> String {
    let mut result = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit {
                byte |= 1 << (7 - i);
            }
        }
        result.push(byte);
    }

    String::from_utf8(result).unwrap_or_else(|_| "Invalid UTF-8".to_string())
}

pub fn bytestring_to_bitvec(s: &str) -> anyhow::Result<Vec<bool>> {
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

pub fn bits_to_bytestring(bits: &[bool]) -> String {
    bits.iter()
        .map(|&bit| if bit { '1' } else { '0' })
        .collect()
}
