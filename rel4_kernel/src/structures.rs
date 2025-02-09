use crate::config::{CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS, MAX_NUM_FREEMEM_REG, MAX_NUM_RESV_REG};
use sel4_common::sel4_config::seL4_MsgMaxExtraCaps;
use sel4_common::structures::{exception_t, seL4_IPCBuffer};
use sel4_common::structures_gen::{cap, cap_null_cap};
use sel4_cspace::interface::cte_t;
use sel4_vspace::pptr_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_BootInfoHeader {
    pub id: usize,
    pub len: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct region_t {
    pub start: usize,
    pub end: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct p_region_t {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v_region_t {
    pub start: usize,
    pub end: usize,
}

#[allow(non_camel_case_types)]
pub type seL4_SlotPos = usize;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct seL4_SlotRegion {
    pub start: seL4_SlotPos,
    pub end: seL4_SlotPos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_UntypedDesc {
    pub paddr: usize,
    pub sizeBits: u8,
    pub isDevice: u8,
    pub padding: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_BootInfo {
    pub extraLen: usize,
    pub nodeID: usize,
    pub numNodes: usize,
    pub numIOPTLevels: usize,
    pub ipcBuffer: *const seL4_IPCBuffer,
    pub empty: seL4_SlotRegion,
    pub sharedFrames: seL4_SlotRegion,
    pub userImageFrames: seL4_SlotRegion,
    pub userImagePaging: seL4_SlotRegion,
    pub ioSpaceCaps: seL4_SlotRegion,
    pub extraBIPages: seL4_SlotRegion,
    pub initThreadCNodeSizeBits: usize,
    pub initThreadDomain: usize,
    #[cfg(feature = "KERNEL_MCS")]
    pub schedcontrol: seL4_SlotRegion,
    pub untyped: seL4_SlotRegion,
    pub untypedList: [seL4_UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndks_boot_t {
    pub reserved: [p_region_t; MAX_NUM_RESV_REG],
    pub resv_count: usize,
    pub freemem: [region_t; MAX_NUM_FREEMEM_REG],
    pub bi_frame: *mut seL4_BootInfo,
    pub slot_pos_cur: seL4_SlotPos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rootserver_mem_t {
    pub cnode: usize,
    pub vspace: usize,
    pub asid_pool: usize,
    pub ipc_buf: usize,
    pub boot_info: usize,
    pub extra_bi: usize,
    pub tcb: usize,
    #[cfg(feature = "KERNEL_MCS")]
    pub sc: usize,
    pub paging: region_t,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct create_frames_of_region_ret_t {
    pub region: seL4_SlotRegion,
    pub success: bool,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct lookupCap_ret_t {
    pub status: exception_t,
    pub capability: cap,
}

impl Default for lookupCap_ret_t {
    fn default() -> Self {
        lookupCap_ret_t {
            status: exception_t::EXCEPTION_NONE,
            capability: cap_null_cap::new().unsplay(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct lookupCapAndSlot_ret_t {
    pub status: exception_t,
    pub capability: cap,
    pub slot: *mut cte_t,
}

impl Default for lookupCapAndSlot_ret_t {
    fn default() -> Self {
        lookupCapAndSlot_ret_t {
            status: exception_t::EXCEPTION_NONE,
            capability: cap_null_cap::new().unsplay(),
            slot: 0 as *mut cte_t,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct syscall_error_t {
    pub invalidArgumentNumber: usize,
    pub invalidCapNumber: usize,
    pub rangeErrorMin: usize,
    pub rangeErrorMax: usize,
    pub memoryLeft: usize,
    pub failedLookupWasSource: usize,
    pub _type: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct extra_caps_t {
    pub excaprefs: [pptr_t; seL4_MsgMaxExtraCaps],
}


#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEInfoBlock {
    pub signature: [u8; 4],           // 签名，4个字节
    pub version: u16,                 // 版本号
    pub oem_string_ptr: u32,          // OEM字符串指针
    pub capabilities: u32,            // 功能
    pub mode_list_ptr: u32,           // 模式列表指针
    pub total_memory: u16,            // 总内存
    pub oem_software_rev: u16,        // OEM软件版本
    pub oem_vendor_name_ptr: u32,     // OEM供应商名称指针
    pub oem_product_name_ptr: u32,    // OEM产品名称指针
    pub oem_product_rev_ptr: u32,     // OEM产品版本指针
    pub reserved: [u8; 222],          // 保留字段，222个字节
    pub oem_data: [u8; 256],          // OEM数据，256个字节
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEModeInfoCommon {
    pub mode_attr: u16,
    pub win_a_attr: u8,
    pub win_b_attr: u8,
    pub win_granularity: u16,
    pub win_size: u16,
    pub win_a_seg: u16,
    pub win_b_seg: u16,
    pub win_func_ptr: u32,
    pub bytes_per_scan_line: u16,
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEInfo12Part1 {
    pub x_res: u16,
    pub y_res: u16,
    pub x_char_size: u8,
    pub y_char_size: u8,
    pub planes: u8,
    pub bits_per_pixel: u8,
    pub banks: u8,
    pub memory_model: u8,
    pub bank_size: u8,
    pub image_pages: u8,
    pub reserved1: u8,
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEInfo12Part2 {
    pub red_len: u8,
    pub red_off: u8,
    pub green_len: u8,
    pub green_off: u8,
    pub blue_len: u8,
    pub blue_off: u8,
    pub rsvd_len: u8,
    pub rsvd_off: u8,
    pub direct_color_info: u8, // 直接颜色模式属性
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEInfo20 {
    pub phys_base_ptr: u32,
    pub reserved2: [u8; 6],
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct seL4_VBEInfo30 {
    pub lin_bytes_per_scan_line: u16,
    pub bnk_image_pages: u8,
    pub lin_image_pages: u8,
    pub lin_red_len: u8,
    pub lin_red_off: u8,
    pub lin_green_len: u8,
    pub lin_green_off: u8,
    pub lin_blue_len: u8,
    pub lin_blue_off: u8,
    pub lin_rsvd_len: u8,
    pub lin_rsvd_off: u8,
    pub max_pixel_clock: u32,
    pub mode_id: u16,
    pub depth: u8,
}

#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct SeL4VBEModeInfoBlock {
    // 所有VBE版本
    pub vbe_common: seL4_VBEModeInfoCommon,
    // VBE 1.2+
    pub vbe12_part1: seL4_VBEInfo12Part1,
    pub vbe12_part2: seL4_VBEInfo12Part2,
    // VBE 2.0+
    pub vbe20: seL4_VBEInfo20,
    // VBE 3.0+
    pub vbe30: seL4_VBEInfo30,
    pub reserved3: [u8; 187],
}

// _seL4_X86_BootInfo_VBE 结构体
#[repr(C, packed)] // 保证与C语言结构体布局一致
pub struct SeL4X86BootInfoVBE {
    pub header: seL4_BootInfoHeader,        // 引导信息头
    pub vbe_info_block: seL4_VBEInfoBlock,  // VBE信息块
    pub vbe_mode_info_block: SeL4VBEModeInfoBlock, // VBE模式信息块
    pub vbeMode: u32,                     // VBE模式
    pub vbe_interface_seg: u32,            // VBE接口段
    pub vbe_interface_off: u32,            // VBE接口偏移量
    pub vbe_interface_len: u32,            // VBE接口长度
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct seL4_X86_mb_mmap {
    pub size: u32,        // 该结构体的大小（字节）
    pub base_addr: u64,   // 该内存区域的起始物理地址
    pub length: u64,      // 该内存区域的长度（字节）
    pub type_: u32,       // 该内存区域的类型，类型1表示RAM
}

#[repr(C, packed)]
pub struct seL4_X86_BootInfo_mmap {
    pub header: seL4_BootInfoHeader, // 引导信息头
    pub mmap_length: u32,           // mmap数组的长度
    pub mmap: [seL4_X86_mb_mmap; 50], // 多重引导内存映射条目
}