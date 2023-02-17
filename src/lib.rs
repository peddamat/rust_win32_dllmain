use std::ffi::c_void;
use windows::{
    Win32::Foundation::*,
    Win32::System::SystemServices::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Threading::*,
    Win32::System::Console::*,
};

static DLL_HANDLE: HINSTANCE = HINSTANCE(0);

unsafe extern "system" fn main_wrapper(_: *mut c_void) -> u32 {
    match std::panic::catch_unwind(|| main()) {
        Ok(_) => 0,
        Err(_) => 1,
    };

    FreeLibraryAndExitThread(DLL_HANDLE, 0)
}

pub fn main() -> Result<(), String> {
  unsafe {
    if AttachConsole(GetCurrentProcessId()).as_bool() || AllocConsole().as_bool() {
      println!("Saying hello from the DLL!");
    }
  }

  return Ok(());
}

#[no_mangle]
pub extern "stdcall" fn DllMain(dll_h: HINSTANCE, reason: u32, _: *mut ()) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(dll_h);
            match CreateThread(None, 0, Some(main_wrapper), None, THREAD_CREATION_FLAGS(0), None) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    true
}

#[no_mangle]
pub extern fn lib_test() {
    println!("Hello from the library!");
}