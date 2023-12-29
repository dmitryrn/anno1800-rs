use std::{mem::{transmute, size_of}, arch::x86_64::__m128, os::raw::c_void};

use log::{debug, error, info, trace};
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualQueryEx, VirtualQuery, MEMORY_BASIC_INFORMATION, VIRTUAL_ALLOCATION_TYPE, MEM_FREE, VirtualAlloc, MEM_COMMIT, MEM_RESERVE},
};

use self::class4::Class4;
pub mod class4;

const WORD_SIZE: usize = size_of::<usize>();
const HANDLE_UPDATE_POTENTIAL_PRODUCTION: u64 = 0x00007FF6F237E400;

pub extern "fastcall" fn handle_update_potential_production_hook(class4: *mut Class4) {
    debug!("{:#016x}", class4 as usize);
    let orig: extern "fastcall" fn(class4: *mut Class4) = unsafe { transmute(HANDLE_UPDATE_POTENTIAL_PRODUCTION) };
    orig(class4);
}

#[derive(Debug)]
pub enum HookError {
    CodecaveSearchFailed,
    VirtualAllocFailed(WIN32_ERROR),
    VirtualProtectFailed(WIN32_ERROR),
    VirtualQueryFailed(WIN32_ERROR),
}

pub unsafe fn hook_call_rel32(
    call_address: usize,
    new_address: usize,
) -> Result<(), HookError> {
    info!("Hooking rel32 call at {call_address:#016x} to {new_address:#016x}");
    let jump = build_far_jump_around(call_address, new_address)?;

    debug!("Patching {:#x} to call {:#x}", call_address, jump as usize);
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
    if !VirtualProtect(call_address as _, 8, PAGE_EXECUTE, &mut old_flags).as_bool() { //TODO restore flags instead of set X?
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return Err(HookError::VirtualProtectFailed(error));
    }

    Ok(())
}

#[cfg(target_pointer_width = "64")]
// Credits to https://stackoverflow.com/a/775124/1569755 and https://stackoverflow.com/a/36511513/1569755
unsafe fn build_far_jump_around(address: usize, destination_address: usize) -> Result<*mut c_void, HookError> {
    let cave = alloc_codecave(address, WORD_SIZE + 1)?;
    let mut code: [u8; 0x13] = [
        0x50,                                                           // push     rax
        0x50,                                                           // push     rax
        0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,     // movabs   rax,0x0000000000000000
        0x48, 0x89, 0x44, 0x24, 0x08,                                   // mov      QWORD PTR [rsp+0x8],rax
        0x58,                                                           // pop      rax
        0xc3,                                                           // ret
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

            return Ok(cave)
        } else {
            trace!("VirtualQuery returned state {:?}", info.State)
        }

        cave_address += 0x100_0000;
    }
    
    error!("Could not find suitable allocatable space for code cave");
    Err(HookError::CodecaveSearchFailed)
}
