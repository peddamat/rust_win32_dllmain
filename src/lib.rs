use std::ffi::c_void;
use windows::{
    Win32::Foundation::*,
    Win32::System::SystemServices::*,
    Win32::System::LibraryLoader::*,
    Win32::System::Threading::*,
    Win32::System::Console::*,
};

static DLL_HANDLE: HINSTANCE = HINSTANCE(0);

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => lib_test(dll_module),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    true
}

#[no_mangle]
pub extern fn lib_test(dll_h: HINSTANCE) {
    unsafe {
        DisableThreadLibraryCalls(dll_h);
        match CreateThread(None, 0, Some(main_wrapper), None, THREAD_CREATION_FLAGS(0), None) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

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