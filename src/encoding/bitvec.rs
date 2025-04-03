/// Our own BitVec struct storing bits in a Vec<u8> (packed 8 bits per byte)
#[derive(Debug, Clone)]
pub struct BitVec {
    pub(crate) data: Vec<u8>,
    pub(crate) len: usize, // number of bits stored
}

#[derive(Debug)]
pub enum BitVecError {
    IndexOutOfBounds,
}

#[allow(unused)]
impl BitVec {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    pub fn from_bytes(bytes: Vec<u8>, bit_length: usize) -> Self {
        Self {
            data: bytes,
            len: bit_length,
        }
    }

    /// Allocate enough capacity to store `bits` bits.
    pub fn with_capacity(bits: usize) -> Self {
        let byte_capacity = (bits + 7) / 8;
        Self {
            data: Vec::with_capacity(byte_capacity),
            len: 0,
        }
    }

    /// Creates a BitVec of a given length (in bits), initialized to 0.
    pub fn zeros(bits: usize) -> Self {
        let byte_capacity = (bits + 7) / 8;
        Self {
            data: vec![0; byte_capacity],
            len: bits,
        }
    }

    /// Creates a BitVec of a given length (in bits), initialized to 1.
    pub fn ones(bits: usize) -> Self {
        let byte_capacity = (bits + 7) / 8;
        Self {
            data: vec![255; byte_capacity], // 255 (b10) == 11111111 (b2)
            len: bits,
        }
    }

    /// Push a new bit onto the BitVec.
    pub fn push(&mut self, bit: bool) {
        if self.len % 8 == 0 {
            self.data.push(0);
        }
        let byte_index = self.len / 8;
        let bit_index = 7 - (self.len % 8);
        if bit {
            self.data[byte_index] |= 1 << bit_index;
        }
        self.len += 1;
    }

    /// Get the bit at the given index.
    /// Returns None if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);
        Some((self.data[byte_index] >> bit_index) & 1 == 1)
    }

    /// Returns a vector of bits.
    pub fn to_vec(&self) -> Vec<bool> {
        let mut vec = Vec::with_capacity(self.len);
        for i in 0..self.len {
            vec.push(self.get(i).unwrap());
        }
        vec
    }

    /// Set the bit at the given index.
    pub fn set(&mut self, index: usize, value: bool) -> Result<(), BitVecError> {
        if index >= self.len {
            return Err(BitVecError::IndexOutOfBounds);
        }
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);
        if value {
            self.data[byte_index] |= 1 << bit_index;
        } else {
            self.data[byte_index] &= !(1 << bit_index);
        }

        Ok(())
    }

    /// Toggle the bit at the given index.
    pub fn toggle(&mut self, index: usize) -> Result<(), BitVecError> {
        if index >= self.len {
            return Err(BitVecError::IndexOutOfBounds);
        }
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);

        self.data[byte_index] ^= 1 << bit_index;

        Ok(())
    }

    /// Constructs a new bit-vector from a vector of bools.
    pub fn from_vec(vec: Vec<bool>) -> Self {
        let byte_capacity = vec.len() / 8;
        let mut bytes = Vec::with_capacity(byte_capacity);

        // aggregate the bits into a byte by chunking the iterator
        for chunk in vec.chunks(8) {
            let mut byte = 0u8;
            for (i, bit) in chunk.into_iter().enumerate() {
                if *bit {
                    byte |= 1 << 7 - i;
                }
            }
            bytes.push(byte);
        }

        Self {
            data: bytes,
            len: vec.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn true_len(&self) -> usize {
        self.data.len()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_push() {
        let mut bv = BitVec::new();
        // make sure we start with an empty bitvec
        assert_eq!(bv.len(), 0);
        assert_eq!(bv.data.len(), 0);

        bv.push(true);
        assert_eq!(bv.len(), 1);
        bv.push(false);
        assert_eq!(bv.len(), 2);
        bv.push(true);
        assert_eq!(bv.len(), 3);

        // make sure we only allocated one byte
        assert_eq!(bv.data.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut bv = BitVec::new();
        bv.push(true);
        assert_eq!(bv.get(0), Some(true));
        assert_eq!(bv.get(1), None);
        assert_eq!(bv.get(2), None);
        assert_eq!(bv.get(3), None);
    }

    #[test]
    fn test_set() {
        let mut bv = BitVec::new();
        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(false);

        assert_eq!(bv.get(0), Some(true));
        assert!(bv.set(0, false).is_ok());
        assert_eq!(bv.get(0), Some(false));
        assert!(bv.set(0, true).is_ok());
        assert_eq!(bv.get(0), Some(true));

        // make sure other bytes are untouched
        assert_eq!(bv.get(1), Some(false));
        assert_eq!(bv.get(2), Some(true));
        assert_eq!(bv.get(3), Some(false));

        // purposefully out of bounds
        assert!(bv.set(10, false).is_err());
    }

    #[test]
    fn test_toggle() {
        let mut bv = BitVec::new();
        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(false);

        assert_eq!(bv.get(0), Some(true));
        assert!(bv.toggle(0).is_ok());
        assert_eq!(bv.get(0), Some(false));
        assert!(bv.toggle(0).is_ok());
        assert_eq!(bv.get(0), Some(true));

        // make sure other bytes are untouched
        assert_eq!(bv.get(1), Some(false));
        assert_eq!(bv.get(2), Some(true));
        assert_eq!(bv.get(3), Some(false));

        // purposefully out of bounds
        assert!(bv.toggle(10).is_err());
    }

    #[test]
    fn test_to_vec() {
        let mut bv = BitVec::new();
        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(false);
        bv.push(true);
        bv.push(false);

        // exactly one byte bitvec (1 byte usage, 8 bits)
        assert_eq!(bv.data.len(), 1);
        assert_eq!(
            bv.to_vec(),
            vec![true, false, true, false, true, false, true, false]
        );

        let bv_2_vec = vec![true, false, true, false, true, false, true, false, false];
        let bv_2 = BitVec::from_vec(bv_2_vec.clone());

        // two byte bitvec (1.125 byte usage, 9 bits)
        assert_eq!(bv_2.data.len(), 2);
        assert_eq!(bv_2.to_vec(), bv_2_vec)
    }

    #[test]
    fn test_zeros() {
        let bv = BitVec::zeros(10);
        assert_eq!(bv.len(), 10);

        (0..10).for_each(|i| {
            assert_eq!(bv.get(i), Some(false));
        });
    }

    #[test]
    fn test_ones() {
        let bv = BitVec::ones(10);
        assert_eq!(bv.len(), 10);

        (0..10).for_each(|i| {
            assert_eq!(bv.get(i), Some(true));
        });
    }

    #[test]
    fn test_from_vec() {
        let bv = BitVec::from_vec(vec![
            true, false, true, false, true, false, true, false, true, false,
        ]);
        assert_eq!(bv.len(), 10);
        assert_eq!(bv.data.len(), 2);

        (0..10).for_each(|i| {
            assert_eq!(bv.get(i), Some(i % 2 == 0));
        });
    }
}
