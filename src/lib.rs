use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{HMODULE, FARPROC};
use windows::Win32::System::LibraryLoader::{LoadLibraryW, GetProcAddress};

pub unsafe fn load_d3d12_create_device() -> Option<unsafe extern "system" fn(
    *mut core::ffi::c_void,
    u32,
    *const windows::core::GUID,
    *mut *mut core::ffi::c_void
) -> windows::core::HRESULT> {
    // 1. Преобразуем строку в PCWSTR (UTF-16 + нуль-терминатор)
    let wide: Vec<u16> = "d3d12_original.dll".encode_utf16().chain([0]).collect();
    let hmodule: HMODULE = LoadLibraryW(PCWSTR(wide.as_ptr())).ok()?;

    // 2. Преобразуем имя функции в PCSTR (ASCII + \0)
    let func_name = b"D3D12CreateDevice\0";
    let func_ptr: FARPROC = GetProcAddress(hmodule, PCSTR(func_name.as_ptr())).ok()?;

    // 3. Приводим к нужному типу функции и возвращаем
    Some(std::mem::transmute(func_ptr))
}
