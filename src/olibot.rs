/// The code of module "olibot" was written by Olivier Fortier ("0l1b0t") and is licensed under the Apache 2.0 license.
/// Source: https://github.com/OlivierFortier/external-dx11-overlay/blob/f39f810176c44bb19be54891fc8037f2844d0dfe/nexus_integration/src/init.rs | Lines 19-25

use windows::Win32::{
    Foundation::HINSTANCE,
    System::LibraryLoader::GetModuleHandleW
};

/// Returns the HMODULE and casts it into HINSTANCE
/// On modern systems, HMODULE is pretty much the same as HINSTANCE, and can be safely cast
pub fn get_hinstance() -> HINSTANCE {
    unsafe { GetModuleHandleW(None).unwrap().into() }
}