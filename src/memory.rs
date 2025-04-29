use std::ptr;

pub struct VulnerableBuffer {
    data: *mut u8,
    size: usize,
}

impl VulnerableBuffer {
    pub fn new(size: usize) -> Self {
        // 취약점: 크기 검증 없음
        let data = unsafe {
            let ptr = libc::malloc(size) as *mut u8;
            ptr::write_bytes(ptr, 0, size);
            ptr
        };
        
        Self { data, size }
    }

    pub fn write(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.size {
            return Err("Input size exceeds buffer capacity");
        }
        
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), self.data, input.len());
        }
        Ok(())
    }

    pub fn read(&self, len: usize) -> Result<Vec<u8>, &'static str> {
        if len > self.size {
            return Err("Requested read length exceeds buffer size");
        }
        
        unsafe {
            let mut result = Vec::with_capacity(len);
            ptr::copy_nonoverlapping(self.data, result.as_mut_ptr(), len);
            result.set_len(len);
            Ok(result)
        }
    }

    // 취약점: private 필드에 대한 직접 접근 메서드
    pub fn get_data_ptr(&self) -> *mut u8 {
        self.data
    }
}

impl Drop for VulnerableBuffer {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.data as *mut libc::c_void);
        }
    }
} 