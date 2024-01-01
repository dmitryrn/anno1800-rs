use std::{
    mem::{size_of, transmute},
    net::UdpSocket,
    os::raw::c_void,
    sync::OnceLock,
};

use log::{debug, error, info, trace};
use windows::{
    core::PCSTR,
    s,
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        System::{
            LibraryLoader::GetModuleHandleA,
            Memory::{
                VirtualAlloc, VirtualProtect, VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_FREE, MEM_RESERVE, PAGE_EXECUTE, PAGE_EXECUTE_READWRITE,
                PAGE_PROTECTION_FLAGS,
            },
        },
    },
};

use crate::api::{class4::Class4, BuildingType, class33::Class33, class34::Class34};

static CELL: OnceLock<UdpSocket> = OnceLock::new();

pub unsafe extern "fastcall" fn handle_update_potential_production_hook(class4_ptr: u64) {
    let class4 = Class4::new(class4_ptr);
    let socket = CELL.get_or_init(|| {
        debug!("creating udp socket");
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        debug!("creating udp socket done");
        socket
    });
    socket.send_to(format!("{:?}\n", class4).as_bytes(), "192.168.178.33:1800").unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xd4e400;
    let orig: extern "fastcall" fn(class4: u64) = unsafe { transmute(call_address) };
    orig(class4_ptr);
}

pub fn exec_get_class34(class33: &Class33, building_type: &BuildingType) -> Class34 {
    unsafe {
        let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
        let call_address = call_base.0 as usize + 0xd63fb0;
        let orig: extern "fastcall" fn(class33_ptr: u64, building_type_ptr: u64) -> u64 = transmute(call_address);
        Class34::new(orig(class33.address, building_type as *const BuildingType as u64))
    }
}

#[derive(Debug)]
pub enum HookError {
    CodecaveSearchFailed,
    VirtualAllocFailed(WIN32_ERROR),
    VirtualProtectFailed(WIN32_ERROR),
    VirtualQueryFailed(WIN32_ERROR),
}

pub unsafe fn hook_call_rel32(call_module: PCSTR, call_offset: usize, new_address: usize) -> Result<(), HookError> {
    let call_base = GetModuleHandleA(call_module).unwrap();
    let call_address = call_base.0 as usize + call_offset;
    info!("Hooking rel32 call at {call_address:#016x} to {new_address:#016x}");
    let jump = build_far_jump_around(call_address, new_address)?;

    debug!("Patching {:#x} to call cave {:#x}", call_address, jump as usize);
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(call_address as _, 8, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return Err(HookError::VirtualProtectFailed(error));
    }
    let new_value = jump.wrapping_sub(call_address + 5); // +5 for the size of the call
    let rel32_ptr: *mut u32 = (call_address + 1) as _;
    debug!("write_volatile {:#016x}", rel32_ptr as usize);
    rel32_ptr.write_volatile(new_value as _);
    if !VirtualProtect(call_address as _, 8, PAGE_EXECUTE, &mut old_flags).as_bool() {
        //TODO restore flags instead of set X?
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return Err(HookError::VirtualProtectFailed(error));
    }

    Ok(())
}

pub unsafe fn unhook_call_rel32(call_module: PCSTR, call_offset: usize, old_value: u32) -> Result<(), HookError> {
    let call_base = GetModuleHandleA(call_module).unwrap();
    let call_address = call_base.0 as usize + call_offset;

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(call_address as _, 8, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return Err(HookError::VirtualProtectFailed(error));
    }
    let rel32_ptr: *mut u32 = (call_address + 1) as _;
    debug!("write_volatile {:#016x}", rel32_ptr as usize);
    rel32_ptr.write_volatile(old_value);
    if !VirtualProtect(call_address as _, 8, PAGE_EXECUTE, &mut old_flags).as_bool() {
        //TODO restore flags instead of set X?
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return Err(HookError::VirtualProtectFailed(error));
    }

    //TODO free cave

    Ok(())
}

#[cfg(target_pointer_width = "64")]
// Credits to https://stackoverflow.com/a/775124/1569755 and https://stackoverflow.com/a/36511513/1569755
unsafe fn build_far_jump_around(address: usize, destination_address: usize) -> Result<*mut c_void, HookError> {
    let cave = alloc_codecave(address, 0x13)?;
    let mut code: [u8; 0x13] = [
        0x50, // push     rax
        0x50, // push     rax
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // movabs   rax, 0x0000000000000000
        0x48, 0x89, 0x44, 0x24, 0x08, // mov      QWORD PTR [rsp+0x8], rax
        0x58, // pop      rax
        0xc3, // ret
    ];
    code[4..12].copy_from_slice(&destination_address.to_le_bytes());
    std::ptr::copy_nonoverlapping(code.as_ptr(), cave as _, code.len());
    Ok(cave)
}

// Credits to https://stackoverflow.com/a/60921721/1569755
unsafe fn alloc_codecave(close_address: usize, size: usize) -> Result<*mut c_void, HookError> {
    let mut cave_address = close_address;
    let mut info: MEMORY_BASIC_INFORMATION;
    info = std::mem::zeroed();

    while cave_address - close_address < 0xffff_ffff {
        trace!("VirtualQuery {:#016x}", cave_address);
        let q = VirtualQuery(Some(cave_address as _), &mut info, size_of::<MEMORY_BASIC_INFORMATION>());
        if q == 0 {
            let error: WIN32_ERROR = GetLastError();
            error!("VirtualQuery {:#016x} failed: {:?}", close_address, error);
            return Err(HookError::VirtualQueryFailed(error));
        }

        if info.State == MEM_FREE {
            debug!("Found free memory at {:#016x}", cave_address);
            let cave = VirtualAlloc(Some(cave_address as _), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
            if cave.is_null() {
                let error: WIN32_ERROR = GetLastError();
                error!("VirtualAlloc {:#016x} failed: {:?}", close_address, error);
                return Err(HookError::VirtualAllocFailed(error));
            }
            debug!("Cave allocated at {:016x}", cave as usize);

            return Ok(cave);
        } else {
            trace!("VirtualQuery returned state {:?}", info.State)
        }

        cave_address += 0x100_0000;
    }

    error!("Could not find suitable allocatable space for code cave");
    Err(HookError::CodecaveSearchFailed)
}
