use core::fmt;

pub type EFIHandle = *const usize;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum EFIStatus {
    Success = 0,
}

pub struct EFITableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

#[repr(C)]
pub struct EFISystemTable<'a> {
    pub hdr: EFITableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    _padding: [u64; 3],
    pub con_out: &'a mut EFISimpleTextOutputProtocol,
}

#[repr(C)]
pub struct EFISimpleTextOutputProtocol {
    _padding: u64,
    output_string: unsafe extern "efiapi" fn(this: &EFISimpleTextOutputProtocol, string: *const u16) -> EFIStatus,
    _padding2: [u64; 4],
    clear_screen: unsafe extern "efiapi" fn(this: &EFISimpleTextOutputProtocol) -> EFIStatus,
}

impl EFISimpleTextOutputProtocol {
    pub fn output_string(&self, string: *const u16) -> EFIStatus {
        unsafe { (self.output_string)(self, string) }
    }

    pub fn clear_screen(&self) -> EFIStatus {
        unsafe { (self.clear_screen)(self) }
    }
}

impl fmt::Write for EFISimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        use core::fmt::Write;
        let mut buffer: [u16; 2] = [0; 2];
        for b in s.chars() {
            if b as u32 > 0xFFFF {
                continue;
            }
            if b == '\n' {
                buffer[0] = '\r' as u16;
                self.output_string(buffer.as_ptr());
            }
            buffer[0] = b as u16;
            self.output_string(buffer.as_ptr());
        }
        Ok(())
    }
}