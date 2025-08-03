use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{HMODULE, FARPROC};
use windows::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};

pub unsafe fn load_d3d12_create_device() -> Option<
    unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows::core::GUID,
        *mut *mut core::ffi::c_void
    ) -> windows::core::HRESULT
> {
    let wide: Vec<u16> = "d3d12_original.dll".encode_utf16().chain([0]).collect();
    let hmodule = LoadLibraryW(PCWSTR(wide.as_ptr())).ok()?; // Result -> Option

    let func_name = b"D3D12CreateDevice\0";
    let func_ptr = GetProcAddress(hmodule, PCSTR(func_name.as_ptr()))?; // Option -> unwrap через ?

    Some(std::mem::transmute(func_ptr))
}
