use crate::structures::*;
use core::ffi::c_void;
use crate::arch::x86_64::multiboot::*;
use crate::arch::x86_64::boot_types::*;

use log::debug;

#[no_mangle]
#[link_section = ".boot.bss"]
pub static mut boot_state: BootState = BootState {
    avail_p_reg: p_region_t { start:0, end:0},
    ki_p_reg: p_region_t { start:0, end:0},
    ui_info: ui_info { p_reg: p_region_t {start:0, end:0}, pv_offset: 0, v_entry: 0},
    num_ioapic: 0,
    ioapic_paddr: [0; 100],
    num_drhu: 0,
    drhu_list: [0; 4096],
    mods_end_paddr: 0,
    boot_module_start: 0,
    num_cpus: 0,
    mem_lower: 0,
    cpus: [0;100],
    mem_p_regs: mem_p_regs {count:0,list:[p_region_t { start:0, end:0};16]},
    vbe_info: SeL4X86BootInfoVBE{
        header: seL4_BootInfoHeader{id:0,len:0},
        vbe_info_block: seL4_VBEInfoBlock{
            signature:[0;4],
            version:0,
            oem_string_ptr:0,
            capabilities:0,
            mode_list_ptr:0,
            total_memory:0,
            oem_software_rev:0,
            oem_vendor_name_ptr:0,
            oem_product_name_ptr:0,
            oem_product_rev_ptr:0,
            reserved:[0;222],
            oem_data:[0;256]
        },
        vbe_mode_info_block: SeL4VBEModeInfoBlock{
            vbe_common:seL4_VBEModeInfoCommon {
                mode_attr:0,
                win_a_attr:0,
                win_b_attr:0,
                win_granularity:0,
                win_size:0,
                win_a_seg:0,
                win_b_seg:0,
                win_func_ptr:0,
                bytes_per_scan_line:0
            },
            vbe12_part1: seL4_VBEInfo12Part1{
                x_res:0,
                y_res:0,
                x_char_size:0,
                y_char_size:0,
                planes:0,
                bits_per_pixel:0,
                banks:0,
                memory_model:0,
                bank_size:0,
                image_pages:0,
                reserved1:0
            },
            vbe12_part2:seL4_VBEInfo12Part2{
                red_len:0,
                red_off:0,
                green_len:0,
                green_off:0,
                blue_len:0,
                blue_off:0,
                rsvd_len:0,
                rsvd_off:0,
                direct_color_info:0
            },
            vbe20:seL4_VBEInfo20{phys_base_ptr:0,reserved2:[0;6]},
            vbe30:seL4_VBEInfo30{
                lin_blue_len:0,
                lin_bytes_per_scan_line:0,
                bnk_image_pages:0,
                lin_image_pages:0,
                lin_red_len:0,
                lin_red_off:0,
                lin_green_len:0,
                lin_green_off:0,
                lin_blue_off:0,
                lin_rsvd_len:0,
                lin_rsvd_off:0,
                max_pixel_clock:0,
                mode_id:0,
                depth:0
            },
            reserved3:[0;187]
        },
        vbeMode:0,
        vbe_interface_seg:0,
        vbe_interface_off:0,
        vbe_interface_len:0
    },
    mb_mmap_info:seL4_X86_BootInfo_mmap{
        header:seL4_BootInfoHeader{id:0,len:0},
        mmap_length:0,
        mmap:[seL4_X86_mb_mmap{size:0,base_addr:0,length:0,type_:0};50],
    },
    fb_info:Multiboot2Fb{addr:0,pitch:0,width:0,height:0,bpp:0,fb_type:0}
};

pub unsafe fn boot_sys(
    multiboot_magic: u64,
    mbi: *mut c_void,
) -> bool {
    let mut result = false;

    if multiboot_magic == MULTIBOOT2_MAGIC {
        result = try_boot_sys_mbi2(mbi);
    } else {
        debug!("Boot loader is not multiboot");
        return false;
    }
    
    if result {
        result = try_boot_sys();
    }

    


    return result;
}

fn round_up(n: u64, b: u64) -> u64 {
    (((n - 1) >> b) + 1) << b
}

pub unsafe fn add_mem_p_regs(reg:&mut p_region_t) -> bool {
    if reg.start == reg.end {
        // Return true here if asked to add an empty region.
        // Some of the callers round down the end address to
        return true;
    }
    if reg.end as u64 > PADDR_TOP && reg.start as u64 > PADDR_TOP {
        /* Return true here as it's not an error for there to exist memory outside the kernel window,
         * we're just going to ignore it and leave it to be given out as device memory */
        return true;
    }
    if boot_state.mem_p_regs.count == 16 {
        debug!("Dropping memory region 0x%lx-0x%lx, try increasing MAX_NUM_FREEMEM_REG\n");
        return false;
    }
    if reg.end as u64 > PADDR_TOP {
        assert!(reg.start as u64 <= PADDR_TOP);
        /* Clamp a region to the top of the kernel window if it extends beyond */
        reg.end = PADDR_TOP as usize;
    }
    debug!("Adding physical memory region 0x%lx-0x%lx\n");
    let index = boot_state.mem_p_regs.count as usize;
    boot_state.mem_p_regs.list[index] = reg.clone();
    boot_state.mem_p_regs.count += 1;
    return true;
}

pub unsafe fn try_boot_sys_mbi2(mbi2:*mut c_void) -> bool{
    let mut mod_count = 0;

    let mbi2_ref: &mut Multiboot2Header = &mut *(mbi2 as *mut Multiboot2Header);
    let mut tag:*mut Multiboot2Tag = (mbi2 as *mut u8).offset(size_of::<Multiboot2Header>() as isize) as *mut Multiboot2Tag;
    let mut tag_e:*mut Multiboot2Tag = (mbi2 as *mut u8).offset(mbi2_ref.total_size as isize) as *mut Multiboot2Tag;

    boot_state.mem_p_regs.count = 0;
    boot_state.mb_mmap_info.mmap_length = 0;
    boot_state.vbe_info.vbeMode = 0xFFFFFFFF;

    while (tag < tag_e && (*tag).tag_type != 0) {
        let behind_tag = tag as u64 + 8;

        if (*tag).tag_type == 1 {
            //cmdline

        } else if (*tag).tag_type == 14 {
            //ACPI_1

        } else if (*tag).tag_type == 15 {
            //ACPI_2

        } else if (*tag).tag_type == 3 {
            //MODULE
            let module:*const Multiboot2Module = behind_tag as *const Multiboot2Module;

            if mod_count == 0 {
                boot_state.boot_module_start = (*module).start as u64;
            }
            mod_count += 1;

            if ((*module).end - (*module).start) <= 0 {
                debug!("Invalid boot module size! Possible cause: boot module file not found\n");
                return false;
            }
            if boot_state.mods_end_paddr < (*module).end as u64 {
                boot_state.mods_end_paddr = (*module).end as u64;
            }
        } else if (*tag).tag_type == 6 {
            //MEMORY
            let mut s:*const Multiboot2Memory = (behind_tag + 8) as *const Multiboot2Memory;
            let e:*const Multiboot2Memory = (tag as u64 + (*tag).size as u64) as *const Multiboot2Memory;

            while s < e {
                if (*s).addr == 0 {
                    boot_state.mem_lower = (*s).size as u32;
                }
                //debug!("Physical Memory Region from {} size {} type {}\n",
                //    (*s).addr, (*s).size, (*s).tag_type);
                if !add_mem_p_regs(&mut p_region_t {start:(*s).addr as usize, end:((*s).addr + (*s).size) as usize}) {
                    return false;
                }
                s = s.add(1);
            }
        } else if (*tag).tag_type == 8 {
            //FB
            let fb:*const Multiboot2Fb = behind_tag as *const Multiboot2Fb;
            debug!("Got framebuffer info in multiboot2. Current video mode is at physical address=%llx pitch=%u resolution=%ux%u@%u type=%u\n");
            boot_state.fb_info = *fb;
        }

        tag = (tag as u64 + round_up((*tag).size as u64, 3)) as *mut Multiboot2Tag;
    }

    debug!("Detected {} boot module(s):\n", mod_count);

    if mod_count < 1 {
        debug!("Expect at least one boot module (containing a userland image)\n");
        return false;
    }

    return true;
}

pub fn try_boot_sys() -> bool {

    return true;
}

pub fn init_plat() {}
