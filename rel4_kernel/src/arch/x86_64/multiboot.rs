pub static MULTIBOOT2_MAGIC: u64 = 0x36d76289;
pub const PPTR_BASE:u64 = 0xffffff8000000000;
pub const PADDR_BASE:u64 = 0;
pub const PPTR_BASE_OFFSET:u64 = PPTR_BASE-PADDR_BASE;
pub const PPTR_TOP:u64 = 0xffffffff80000000;
pub const PADDR_TOP:u64 = PPTR_TOP-PPTR_BASE_OFFSET;

#[repr(C, packed)]
pub struct Multiboot2Header {
    pub total_size: u32,
    pub unknown: u32,
}

#[repr(C, packed)]
pub struct Multiboot2Tag {
    pub tag_type: u32,
    pub size: u32,
}

#[repr(C, packed)]
pub struct Multiboot2Memory {
    pub addr: u64,
    pub size: u64,
    pub tag_type: u32,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct Multiboot2Module {
    pub start: u32,
    pub end: u32,
    pub string: [u8; 1],  // In Rust, we need to represent the "flexible array member" in this way.
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Multiboot2Fb {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub fb_type: u8,
}

#[repr(u32)]  // Specifying the underlying type of the enum to ensure it aligns with C's u32.
pub enum Multiboot2Tags {
    End = 0,
    Cmdline = 1,
    Module = 3,
    Memory = 6,
    Fb = 8,
    Acpi1 = 14,
    Acpi2 = 15,
}
