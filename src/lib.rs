use windows::Win32::{
    Foundation::HINSTANCE,
    System::LibraryLoader::GetModuleHandleW
};

fn get_hinstance() -> HINSTANCE { unsafe { GetModuleHandleW(None).unwrap().into() } }

fn load_wrapper() {
    let hinstance = get_hinstance();
    external_dx11_overlay::attach(hinstance);
}

fn unload_wrapper() {
    external_dx11_overlay::detatch();
}

nexus::export! {
    name: "External DX11 Overlay for Nexus",
    signature: -0x7A8B9C2E,
    load: load_wrapper,
    unload: unload_wrapper,
    flags: nexus::AddonFlags::DisableHotloading,
    provider: nexus::UpdateProvider::GitHub,
    update_link: "https://github.com/herrfleischladen/external-dx11-overlay-for-nexus"
}