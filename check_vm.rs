#![feature(asm)]
/*

b1tg @ 2021/05/27 20:55

detect method from: 
- https://github.com/LordNoteworthy/al-khaser
- https://www.freebuf.com/articles/system/202717.html

*/
use failure::Fallible;

fn is_vm() -> bool {
    let mut x: i32 = 0;
    unsafe {
        asm!(
            "mov eax, 1",
            "cpuid",
            "and ecx, 0x80000000",
            "test ecx, ecx",
            "setz al",
             out("ax") x,
        );
    }
    return x & 0x0000_0001 != 1;
}

fn cpuid_is_hypervisor() -> bool {
    let mut ecx: u32 = 0;
    unsafe {
        asm!(
            "mov eax, 1",
            "cpuid",
            out("ecx") ecx,
        )
    }
    return ecx >> 31 & 1 == 1;
}

fn cpuid_hypervisor_vendor() -> bool {
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;
    unsafe {
        asm!(
            "mov eax, 0x40000000",
            "cpuid",
            "mov r8, rbx",
            out("r8") ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
        )
    }
    let mut id_bytes: Vec<u8> = vec![];
    for r in &[&ebx, &ecx, &edx] {
        let mut bts = hex::decode(format!("{:x}", r)).unwrap_or(vec![]);
        bts.reverse();
        id_bytes.append(&mut bts);
    }
    let btstr = String::from_utf8(id_bytes).unwrap_or("".to_owned());

    return [
        "KVMKVMKVM\0\0\0",
        "Microsoft Hv",
        "VMwareVMware",
        "XenVMMXenVMM",
        "prl hyperv  ",
        "VBoxVBoxVBox",
    ]
    .contains(&btstr.as_str());
}

fn main() -> Fallible<()> {
    dbg!(is_vm());
    dbg!(cpuid_is_hypervisor());
    dbg!(cpuid_hypervisor_vendor());
    Ok(())
}
