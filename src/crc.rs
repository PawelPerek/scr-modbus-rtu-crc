pub struct CRC {
    hi_byte: usize,
    lo_byte: usize,
}

use crate::lut;

impl CRC {
    pub fn new() -> Self {
        CRC { hi_byte: 0xFF, lo_byte: 0xFF }
    }

    pub fn calculate(&mut self, data: &[usize]) -> usize {
        for &b in data {
            let index = (self.hi_byte ^ b) as usize;
            self.hi_byte = self.lo_byte ^ unsafe { lut::A_CRC_HI.get_unchecked(index) };
            self.lo_byte = unsafe { *lut::A_CRC_LO.get_unchecked(index) };
        }
        (self.hi_byte as usize) << 8 | self.lo_byte as usize
    }

}
