use rand::Rng;

use crate::{
    encoding::bitvec::BitVec,
    encoding::hamming::{Hamming, HammingCode, HammingError},
};

const USIZE_SIZE: usize = std::mem::size_of::<usize>();

/// Length structure to hold the length of data and bits
///
/// Should take up either 8 or 16 bytes depending on the architecture (32 or 64 bits)
#[derive(Debug, Clone)]
pub struct Length {
    pub data_length: usize,
    pub bits_length: usize,
}

impl Length {
    pub fn to_le_bytes(&self) -> [u8; USIZE_SIZE * 2] {
        let mut bytes = [0u8; USIZE_SIZE * 2];
        bytes[0..USIZE_SIZE].copy_from_slice(&self.data_length.to_le_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE * 2].copy_from_slice(&self.bits_length.to_le_bytes());
        bytes
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        let data_length = usize::from_le_bytes(bytes[0..USIZE_SIZE].try_into()?);
        let bits_length = usize::from_le_bytes(bytes[USIZE_SIZE..USIZE_SIZE * 2].try_into()?);

        Ok(Self {
            data_length,
            bits_length,
        })
    }
}

/// Great Unused Standard Protocol (imaginary ;P)
#[derive(Debug, Clone)]
pub struct GUSProtocol {
    pub protocol_name: Vec<u8>,
    pub version: u8,
    pub data: BitVec,
}

impl GUSProtocol {
    const CURRENT_VERSION: u8 = 1;
    const PROTOCOL_NAME: &[u8; 3] = b"GUS";

    pub fn new(data: BitVec) -> Result<Self, HammingError> {
        let protocol_name = Self::PROTOCOL_NAME.to_vec();
        let version = Self::CURRENT_VERSION;

        Ok(Self {
            data,
            protocol_name,
            version,
        })
    }

    pub fn encode(self) -> Result<Vec<u8>, HammingError> {
        let mut encoded_data = Hamming.encode(&self.data)?;

        // give a 50% chance of corrupting a bit
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            let bit_to_corrupt = rng.random_range(0..encoded_data.len());
            encoded_data
                .toggle(bit_to_corrupt)
                .map_err(|_| HammingError::UnexpectedOutOfBounds)?;
        }

        let length = Length {
            data_length: encoded_data.true_len(),
            bits_length: encoded_data.len(),
        };

        let mut encoded = Vec::with_capacity(length.data_length + 4 + USIZE_SIZE * 2);
        let length_bytes = length.to_le_bytes();

        encoded.extend(self.protocol_name);
        encoded.push(self.version);
        encoded.extend(length_bytes);
        encoded.extend(encoded_data.into_inner());

        Ok(encoded)
    }

    pub fn decode(encoded_data: Vec<u8>) -> Result<(Self, bool), anyhow::Error> {
        if encoded_data.len() < (4 + USIZE_SIZE * 2) {
            return Err(anyhow::anyhow!("Invalid data length"));
        }

        let protocol_name = &encoded_data[0..3];
        if protocol_name != Self::PROTOCOL_NAME {
            return Err(anyhow::anyhow!("Invalid protocol name"));
        }

        let version = encoded_data[3];
        if version != Self::CURRENT_VERSION {
            return Err(anyhow::anyhow!("Unsupported version"));
        }

        let length_bytes = &encoded_data[4..(4 + USIZE_SIZE * 2)];
        let length = Length::from_le_bytes(length_bytes)?;

        let data =
            encoded_data[(4 + USIZE_SIZE * 2)..(4 + USIZE_SIZE * 2 + length.data_length)].to_vec();
        if data.len() != length.data_length as usize {
            return Err(anyhow::anyhow!("Data length mismatch"));
        }
        let data = BitVec::from_bytes(data, length.bits_length as usize);

        // decode the hamming code
        let decoded_data = Hamming
            .decode(&data)
            .map_err(|_| anyhow::anyhow!("Failed to decode Hamming code"))?;

        let corrected_error = decoded_data.1 != 0;

        Ok((
            Self {
                protocol_name: protocol_name.to_vec(),
                version,
                data: decoded_data.0,
            },
            corrected_error,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gus_protocol() {
        let original_data = BitVec::from_vec(vec![
            true, false, true, false, true, false, true, false, true, true, false, true, false,
        ]);

        let gus = GUSProtocol::new(original_data).unwrap();
        let encoded = gus.clone().encode().unwrap();

        let decoded = GUSProtocol::decode(encoded).unwrap().0;

        assert_eq!(gus.protocol_name, decoded.protocol_name);
        assert_eq!(gus.version, decoded.version);
        assert_eq!(gus.data.len(), decoded.data.len());
        assert_eq!(gus.data.into_inner(), decoded.data.into_inner());
    }
}
