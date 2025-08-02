use std::ffi::c_void;
use std::ptr;
use windows::Win32::Foundation::{HRESULT, HMODULE};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};

type D3D12CreateDeviceFn = unsafe extern "system" fn(
    *mut c_void,
    D3D_FEATURE_LEVEL,
    *const windows::core::GUID,
    *mut *mut c_void
) -> HRESULT;

#[no_mangle]
pub extern "system" fn D3D12CreateDevice(
    adapter: *mut c_void,
    min_feature_level: D3D_FEATURE_LEVEL,
    riid: *const windows::core::GUID,
    device_out: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        let dll = LoadLibraryW("d3d12_original.dll");
        if dll.0 == 0 {
            println!("[hook] d3d12_original.dll not loaded");
            return HRESULT(-1);
        }

        let orig = GetProcAddress(dll, "D3D12CreateDevice\0");
        if orig.is_null() {
            println!("[hook] D3D12CreateDevice not found");
            return HRESULT(-1);
        }

        let real_fn: D3D12CreateDeviceFn = std::mem::transmute(orig);
        let result = real_fn(adapter, min_feature_level, riid, device_out);

        println!("[hook] D3D12CreateDevice called — hooked in Rust!");

        result
    }
}
