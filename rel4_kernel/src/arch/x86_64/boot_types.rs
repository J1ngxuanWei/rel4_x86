use crate::structures::*;
use crate::arch::x86_64::multiboot::*;

// mem_p_regs 结构体
#[repr(C)] // 保证与C语言结构体布局一致
pub struct mem_p_regs {
    pub count: u64,                       // count字段
    pub list: [p_region_t; 16], // list数组，最大数量为MAX_NUM_FREEMEM_REG
}

// ui_info 结构体
#[repr(C)] // 保证与C语言结构体布局一致
pub struct ui_info {
    pub p_reg: p_region_t,  // 用户空间映像所在的物理区域
    pub pv_offset: u64, // UI虚拟地址 + pv_offset = UI物理地址
    pub v_entry: u64,    // 用户空间映像的入口点（虚拟地址）
}

#[repr(C)] // 保证与C语言结构体布局一致
pub struct BootState {
    pub avail_p_reg: p_region_t,    // 可用物理内存区域
    pub ki_p_reg: p_region_t,       // 内核映像所在区域
    pub ui_info: ui_info,         // 用户空间映像信息
    pub num_ioapic: u32,         // 检测到的IOAPIC数量
    pub ioapic_paddr: [u64; 100], // IOAPIC的物理地址列表 //WJXTODOWJX数量不一定是100
    pub num_drhu: u32,           // IOMMU的数量
    pub drhu_list: [u64; 4096], // IOMMU物理地址列表
    //pub rmrr_list: AcpiRmrrList, // RMRR列表
    //pub acpi_rsdp: AcpiRsdp,     // rsdp副本
    pub mods_end_paddr: u64,   // 引导模块结束的物理地址
    pub boot_module_start: u64,// 第一个引导模块的物理地址
    pub num_cpus: u32,           // 检测到的CPU数量
    pub mem_lower: u32,          // AP运行实模式的引导代码所需的下方内存大小
    pub cpus: [u64; 100], // CPU列表   //WJXTODOWJX数量不一定是100
    pub mem_p_regs: mem_p_regs,    // 物理内存区域
    pub vbe_info: SeL4X86BootInfoVBE, // 来自multiboot的潜在VBE信息
    pub mb_mmap_info: seL4_X86_BootInfo_mmap, // 来自multiboot的内存映射信息
    pub fb_info: Multiboot2Fb, // 启动加载器设置的帧缓冲信息
}