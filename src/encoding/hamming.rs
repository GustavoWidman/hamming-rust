use std::fmt::{Display, Formatter};

use crate::encoding::bitvec::BitVec;

#[derive(Debug)]
pub enum HammingError {
    UnexpectedOutOfBounds,
}
impl Display for HammingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HammingError::UnexpectedOutOfBounds => write!(f, "Unexpected out of bounds error"),
        }
    }
}

/// Trait defining the interface for Hamming code implementations
pub trait HammingCode {
    fn encode(&self, data: &BitVec) -> Result<BitVec, HammingError>;
    fn decode(&self, codeword: &BitVec) -> Result<(BitVec, usize), HammingError>;
    fn calculate_parity(&self, codeword: &BitVec, parity_mask: usize)
    -> Result<bool, HammingError>;
}

/// Base implementation containing shared functionality
pub struct HammingCodeBase;

impl HammingCodeBase {
    pub fn calculate_parity_count(data_len_bits: usize) -> usize {
        let mut parity_count = 0;
        while (1 << parity_count) < (data_len_bits + parity_count + 1) {
            parity_count += 1;
        }
        parity_count
    }

    pub fn basic_compute_parity(
        codeword: &BitVec,
        parity_mask: usize,
    ) -> Result<bool, HammingError> {
        let mut parity_sum = 0u32;
        for i in 0..codeword.len() {
            if ((i + 1) & parity_mask) != 0 {
                if codeword.get(i).ok_or(HammingError::UnexpectedOutOfBounds)? {
                    parity_sum += 1;
                }
            }
        }
        Ok(parity_sum % 2 == 1)
    }
}

pub struct Hamming;

impl HammingCode for Hamming {
    fn calculate_parity(
        &self,
        codeword: &BitVec,
        parity_mask: usize,
    ) -> Result<bool, HammingError> {
        HammingCodeBase::basic_compute_parity(codeword, parity_mask)
    }

    fn encode(&self, data: &BitVec) -> Result<BitVec, HammingError> {
        let data_len_bits = data.len();
        let parity_count = HammingCodeBase::calculate_parity_count(data_len_bits);
        let total_len = data_len_bits + parity_count;
        let mut codeword = BitVec::zeros(total_len);

        let mut data_index = 0;
        for i in 0..total_len {
            if (i + 1).is_power_of_two() {
                continue;
            }
            codeword
                .set(
                    i,
                    data.get(data_index)
                        .ok_or(HammingError::UnexpectedOutOfBounds)?,
                )
                .map_err(|_| HammingError::UnexpectedOutOfBounds)?;
            data_index += 1;
        }

        for i in 0..parity_count {
            let parity_pos = (1 << i) - 1;
            let parity = self.calculate_parity(&codeword, 1 << i)?;
            codeword
                .set(parity_pos, parity)
                .map_err(|_| HammingError::UnexpectedOutOfBounds)?;
        }

        Ok(codeword)
    }

    fn decode(&self, codeword: &BitVec) -> Result<(BitVec, usize), HammingError> {
        let n = codeword.len();
        let r = (0..).find(|&r| (1 << r) >= n + 1).unwrap();
        let mut error_pos = 0;

        for i in 0..r {
            if self.calculate_parity(codeword, 1 << i)? {
                error_pos += 1 << i;
            }
        }

        let mut corrected = BitVec::clone(codeword);
        if error_pos > 0 && error_pos - 1 < corrected.len() {
            let current = corrected
                .get(error_pos - 1)
                .ok_or(HammingError::UnexpectedOutOfBounds)?;
            corrected
                .set(error_pos - 1, !current)
                .map_err(|_| HammingError::UnexpectedOutOfBounds)?;
        }

        let mut data = BitVec::with_capacity(n - r);
        for i in 0..corrected.len() {
            if !((i + 1).is_power_of_two()) {
                data.push(
                    corrected
                        .get(i)
                        .ok_or(HammingError::UnexpectedOutOfBounds)?,
                );
            }
        }

        Ok((data, error_pos))
    }
}
