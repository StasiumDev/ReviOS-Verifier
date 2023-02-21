extern crate winapi;
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();

    // Set the icon and release to v1.4.0.0
    res.set_icon("revi.ico")
        .set("ReviOS Verifier", "revios_verifier.EXE")
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.set_language(winapi::um::winnt::MAKELANGID(
        winapi::um::winnt::LANG_ENGLISH,
        winapi::um::winnt::SUBLANG_ENGLISH_US,
    ));
    res.compile().unwrap();
}
