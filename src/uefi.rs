use core::ffi::c_void;

#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

pub struct EfiHandle(*mut c_void);

#[allow(dead_code)]
#[repr(usize)]
pub enum EfiStatus {
    Sucess = 0,
    LoadError = 1,
    InvalidParameter = 2,
    Unsupported = 3,
    BadBufferSize = 4,
}

#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    _con_in: usize,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    _std_err: usize, // TBD
    _runtime_services: usize,
    pub boot_services: *mut EfiBootServices,
    _number_of_table_entries: usize,
    _configuration_table: usize,
}

#[repr(C)]
pub struct EfiMemoryDescriptor {}

#[repr(C)]
pub struct EfiBootServices {
    pub header: EfiTableHeader,
    _2: unsafe extern "C" fn(),
    _3: unsafe extern "C" fn(),
    _4: unsafe extern "C" fn(),
    _5: unsafe extern "C" fn(),
    get_memory_map: unsafe extern "C" fn(
        memory_map_size: usize,
        memory_map: EfiMemoryDescriptor,
        map_key: usize,
        descriptor_size: usize,
        descriptor_version: u32,
    ) -> EfiStatus,
    _7: unsafe extern "C" fn(),
    _8: unsafe extern "C" fn(),
    _9: unsafe extern "C" fn(),
    _10: unsafe extern "C" fn(),
    _11: unsafe extern "C" fn(),
    _12: unsafe extern "C" fn(),
    _13: unsafe extern "C" fn(),
    _14: unsafe extern "C" fn(),
    _15: unsafe extern "C" fn(),
    _16: unsafe extern "C" fn(),
    _17: unsafe extern "C" fn(),
    _18: unsafe extern "C" fn(),
    _19: unsafe extern "C" fn(),
    _20: unsafe extern "C" fn(),
    _21: unsafe extern "C" fn(),
    _22: unsafe extern "C" fn(),
    _23: unsafe extern "C" fn(),
    _24: unsafe extern "C" fn(),
    _25: unsafe extern "C" fn(),
    pub exit_boot_services: unsafe extern "C" fn(handle: EfiHandle, map_key: usize) -> EfiStatus,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, extended: bool) -> EfiStatus,
    pub output_string:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, string: *const u16) -> usize,
    _test_string:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, extended: bool) -> EfiStatus,
    _query_mode:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, mode_number: usize) -> EfiStatus,
    _set_mode:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, mode_number: usize) -> EfiStatus,
    _set_attribute:
        unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol, mode_number: usize) -> EfiStatus,
    pub clear_screen: unsafe extern "C" fn(this: &EfiSimpleTextOutputProtocol) -> EfiStatus,
}


pub fn init(system_table: EfiSystemTable){
    let stdout = system_table;
}