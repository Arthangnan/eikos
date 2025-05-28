use core::arch::asm;
use core::fmt;
use core::marker::PhantomData;

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

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    #[inline]
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }
}

// Reading and writing to an I/O port can have side effects.
// We enforce mutability to align with this behaviour.

impl<T: PortRead> Port<T> {
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        unsafe { T::read_from_port(self.port) }
    }
}

impl<T: PortWrite> Port<T> {
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        unsafe { T::write_to_port(self.port, value) }
    }
}

impl<T> fmt::Debug for Port<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Port")
            .field("port", &self.port)
            .field("size", &core::mem::size_of::<T>())
            .finish()
    }
}

impl<T> Clone for Port<T> {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            phantom: PhantomData,
        }
    }
}

impl<T> PartialEq for Port<T> {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
    }
}

impl<T> Eq for Port<T> {}
