use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::core::{PCSTR, Result, s, w};
use windows::Win32::Foundation::HMODULE;

fn main() {
    // example of showing how one might manually build a PCSTR
    let path_as_slice = "rust_dll.dll\0";
    let _dll_file_path: PCSTR = PCSTR::from_raw(path_as_slice.as_ptr());

    // A literal UTF-8 string with a trailing null terminator
    // If you want to use this, make yourself a simple DLL and change the below string to point to
    // your DLL.
    let dll_file_path = s!("rust_dll.dll");

    println!("[i] Loading string at location: {:?}", dll_file_path);

    // use the Win32 API for LoadLibraryA and match the result
    let hmod: Result<HMODULE> = unsafe { LoadLibraryA(dll_file_path) };
    match hmod {
        Ok(h) => { println!("[+] Module injected! Handle: {:?}", h); loop {} },
        Err(e) => println!("Error[-] injecting module, {e}"),
    }
}
