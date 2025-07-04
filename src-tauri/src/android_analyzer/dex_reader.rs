use std::io::{Read, Seek, SeekFrom};
use crate::android_analyzer::error::{AndroidAnalyzeError, Result};

/// A utility for reading binary data from APK/DEX files
pub struct DexReader<R: Read + Seek> {
    reader: R,
}

impl<R: Read + Seek> DexReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read_exact(buf)?;
        Ok(())
    }

    pub fn seek(&mut self, pos: u64) -> Result<()> {
        self.reader.seek(SeekFrom::Start(pos))?;
        Ok(())
    }

    pub fn stream_position(&mut self) -> Result<u64> {
        Ok(self.reader.stream_position()?)
    }

    /// Read a variable-length unsigned integer (LEB128)
    pub fn read_uleb128(&mut self) -> Result<u32> {
        let mut result = 0u32;
        let mut shift = 0;
        
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7f) as u32) << shift;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            shift += 7;
            if shift >= 32 {
                return Err(AndroidAnalyzeError::ParseError("Invalid ULEB128 value".to_string()));
            }
        }
        
        Ok(result)
    }

    /// Read a variable-length signed integer (LEB128)
    pub fn read_sleb128(&mut self) -> Result<i32> {
        let mut result = 0i32;
        let mut shift = 0;
        let mut byte;
        
        loop {
            byte = self.read_u8()?;
            result |= ((byte & 0x7f) as i32) << shift;
            shift += 7;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            if shift >= 32 {
                return Err(AndroidAnalyzeError::ParseError("Invalid SLEB128 value".to_string()));
            }
        }
        
        // Sign extend if necessary
        if (shift < 32) && ((byte & 0x40) != 0) {
            result |= !0i32 << shift;
        }
        
        Ok(result)
    }

    /// Read a null-terminated string
    pub fn read_cstring(&mut self) -> Result<String> {
        let mut bytes = Vec::new();
        
        loop {
            let byte = self.read_u8()?;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        
        String::from_utf8(bytes)
            .map_err(|e| AndroidAnalyzeError::ParseError(format!("Invalid UTF-8 string: {}", e)))
    }

    /// Read a fixed-length byte array
    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; len];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    /// Skip a number of bytes
    pub fn skip(&mut self, count: usize) -> Result<()> {
        let current_pos = self.stream_position()?;
        self.seek(current_pos + count as u64)?;
        Ok(())
    }
}
