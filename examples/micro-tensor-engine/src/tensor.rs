/// Contiguous row-major dense tensor (`f32`) for micro-engine benchmarks.
#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    shape: Vec<usize>,
    data: Vec<f32>,
}

impl Tensor {
    pub fn zeros(shape: &[usize]) -> Self {
        let len = Self::checked_len(shape);
        Self {
            shape: shape.to_vec(),
            data: vec![0.0; len],
        }
    }

    pub fn from_vec(shape: &[usize], data: Vec<f32>) -> Self {
        let expected = Self::checked_len(shape);
        assert_eq!(
            data.len(),
            expected,
            "data length {} does not match shape {:?} (expected {})",
            data.len(),
            shape,
            expected
        );
        Self {
            shape: shape.to_vec(),
            data,
        }
    }

    /// Deterministic pseudo-random tensor in `[-1.0, 1.0]` using xorshift64* (no external deps).
    pub fn random(shape: &[usize], seed: u64) -> Self {
        let len = Self::checked_len(shape);
        let mut state = if seed == 0 { 0x9e37_79b9_7f4a_7c15 } else { seed };
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let bits = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            // Map top 24 bits to [-1.0, 1.0]
            let unit = ((bits >> 40) as f32) / ((1u32 << 24) as f32);
            data.push(unit * 2.0 - 1.0);
        }
        Self {
            shape: shape.to_vec(),
            data,
        }
    }

    fn checked_len(shape: &[usize]) -> usize {
        assert!(!shape.is_empty(), "tensor shape must have at least 1 dimension");
        shape
            .iter()
            .copied()
            .try_fold(1usize, usize::checked_mul)
            .expect("tensor element count overflow")
    }

    #[inline]
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    #[inline]
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.data
    }

    pub fn dims_2d(&self) -> (usize, usize) {
        assert_eq!(
            self.shape.len(),
            2,
            "expected a 2D tensor, got shape {:?}",
            self.shape
        );
        (self.shape[0], self.shape[1])
    }

    pub fn max_abs_diff(&self, other: &Self) -> f32 {
        assert_eq!(self.shape, other.shape, "tensor shapes must match");
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0_f32, f32::max)
    }
}
