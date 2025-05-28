use core::arch::asm;

pub trait PortRead {
    unsafe fn read_from_port(port: u16) -> Self;
}

pub trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u8 {
        let mut val: u8;
        unsafe {
            asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }
}

impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u16 {
        let mut val: u16;
        unsafe {
            asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }
}

impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u32 {
        let mut val: u32;
        unsafe {
            asm!("in eax, dx", out("eax") val, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        val
    }
}

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u8) {
        unsafe {
            asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
        }
    }
}

impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
        }
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u32) {
        unsafe {
            asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
        }
    }
}
