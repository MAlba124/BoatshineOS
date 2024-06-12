fn main() {
    // choose whether to start the UEFI or BIOS image
    let uefi = false;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-debugcon").arg("stdio");
    if uefi {
        // let uefi_path = env!("UEFI_PATH");
        // cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        // cmd.arg("-drive")
        //     .arg(format!("format=raw,file={uefi_path}"));
    } else {
        let bios_path = env!("BIOS_PATH");
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
