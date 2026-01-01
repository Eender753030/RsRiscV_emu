//! Memory define and implment for Risc-V

use crate::utils::exception::RiscVError;

/// Memory structure. Store `u8` data as Little Endian.
#[derive(Debug)]
pub struct Memory {
    size: usize,
    space: Vec<u8>,
}

impl Memory {
    /// Create a `Memory` instance. Force the `size` divides exactly by 4 for align. 
    pub fn new(size: usize) -> Self {
        if size < 4 {
            Memory{size: 4, space: vec![0; 4]}
        } else {
            let ture_size = size + (4 - size % 4); 
            Memory{size: ture_size, space: vec![0; size]}
        }
    }

    /// Load binary into `Memory`'s `space` start from `start_address`. 
    /// The `data_container`'s size must divides exactly by 4.
    /// Return `OutOfBoundMemory` if the `end_address` is greater than `Memory`'s `size`.
    pub fn load(&mut self, start_address:usize, data_container: &[u8]) -> Result<(), RiscVError> {
        let end_address = start_address + data_container.len();

        if end_address > self.size {
            return Err(RiscVError::OutOfBoundMemory);
        }

        self.space[start_address..end_address].copy_from_slice(data_container);
 
        Ok(())
    } 
    
    /// Fetch `u32` instruction in `Memory`'s `space` start at `pc` pointer.
    /// The `pc` must divides exactly by 4, otherwise return `InstructionAddressMisaligned`
    /// Return `OutOfBoundMemory` if the `end_address` is greater than `Memory`'s `size`.
    pub fn fetch(&self, pc: u32) -> Result<u32, RiscVError> {
        if !pc.is_multiple_of(4) {
            return Err(RiscVError::InstructionAddressMisaligned(pc));
        }

        let idx = pc as usize;

        if idx + 4 > self.size {
            return Err(RiscVError::OutOfBoundMemory);
        }

        let slice = &self.space[idx..idx+4];
        // Safe: Here the `slice`'s contents are definitely 4 `u8` by the check on above. 
        // `try_into` and `unwrap` is safe.
        Ok(u32::from_le_bytes(slice.try_into().unwrap()))
    } 

    /// Read a `u32` Little Endian data from `Memory` start at `address` for certain bytes. 
    /// The bytes less than `u32` will auto do sign extension and turn into `u32`.
    /// `bytes_amount` can only is 1 to 4, otherwise return `ReadInvalidByets`.
    /// Return `OutOfBoundMemory` if the `address + bytes_amount` is greater than or equal to `Memory`'s `size`.
    pub fn read(&self, address: u32, bytes_amount: usize, is_signed: bool) -> Result<u32, RiscVError> {
        let idx = address as usize;

        if !(1..=4).contains(&bytes_amount) {
            return Err(RiscVError::ReadInvalidByets);   
        }

        if idx + bytes_amount >= self.size  {
            return Err(RiscVError::OutOfBoundMemory);
        }

        let slice = &self.space[idx..idx+bytes_amount];
        let mut four_bytes = [0_u8; 4];

        four_bytes[..bytes_amount].copy_from_slice(slice);

        // Sign extension if MSB is 1.
        // Safe: The `slice` definitely has last element.
        if is_signed && (slice.last().unwrap() & 0x80 != 0) {
            four_bytes[bytes_amount..].fill(0xff);
        }

        Ok(u32::from_le_bytes(four_bytes))
    }

    /// Read a batch(slice) of 'u8' data by size of `bytes_amount` from `Memory` start at `address`.
    /// Return `OutOfBoundMemory` if the `address + bytes_amount` is greater than or equal to `Memory`'s `size`.
    pub fn read_batch(&self, address: usize, bytes_amount: usize) -> Result<&[u8], RiscVError> {
        if address + bytes_amount >= self.size  {
            Err(RiscVError::OutOfBoundMemory)
        } else {
            Ok(&self.space[address..address+bytes_amount])
        }
    } 

    /// Write a `u32` `data`` into `Memory` start at `address` for certain bytes. 
    /// The input `data`` will turn into `u32` Little Endian.
    /// Return `OutOfBoundMemory` if the `address + bytes_amount` is greater than or equal to `Memory`'s `size`.
    pub fn write(&mut self, address: u32, data: u32, bytes_amount: usize) -> Result<(), RiscVError> {
        let idx = address as usize;

        if idx + bytes_amount >= self.size {
            return Err(RiscVError::OutOfBoundMemory);
        }

        let write_data = data.to_le_bytes();
         
        self.space[idx..idx+bytes_amount].copy_from_slice(&write_data[0..bytes_amount]);
        
        Ok(())
    } 

    /// Reset `Memory`'s `space` by fill 0
    pub fn reset(&mut self) {
        self.space.fill(0);
    }

    /// Dump `Memory`'s data by a `Vec` of 4 size `u8` arrays.  
    pub fn dump(&self) -> Vec<[u8; 4]> {
        self.space.chunks(4).map(|slice| slice.try_into().unwrap()).collect()
    }
}