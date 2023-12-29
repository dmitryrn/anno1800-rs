use std::{mem::size_of, string::FromUtf8Error};

use clap::Parser;
use log::{debug, error, info, LevelFilter};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
use windows::{
    core::{Error, PCSTR},
    s,
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        System::{
            Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_NT_HEADERS32, IMAGE_NT_HEADERS64},
            LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
            SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY},
        },
    },
};

use crate::{remote_process::RemoteProcess, remote_thread::RemoteThread, remote_virtual_allocation::RemoteVirtualAllocation};

pub mod remote_process;
pub mod remote_thread;
pub mod remote_virtual_allocation;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Operation {
    Load,
    Unload,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(value_enum, default_value_t=Operation::Load)]
    operation: Operation,
}

#[derive(Clone, Debug)]
pub enum Anno1800AgentLoaderError {
    OpenProcessFailed(Error),
    GetModuleHandleAFailed(Error),
    GetProcAddressFailed(WIN32_ERROR),
    VirtualAllocExFailed(WIN32_ERROR),
    WriteProcessMemoryFailed(WIN32_ERROR),
    ReadProcessMemoryFailed(WIN32_ERROR),
    ReadStringFailed(FromUtf8Error),
    CreateRemoteThreadFailed(Error),
    GetExitCodeThreadFailed,
    LoadLibraryAFailed,
    FreeLibraryFailed(u32),
    GetProcAddressRemoteFailed,
    ExportedFunctionNotFound,
}

fn main() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let args = Args::parse();
    let s = System::new_all();
    let patricians: Vec<&Process> = s.processes_by_exact_name("Anno1800.exe").collect();
    if patricians.is_empty() {
        error!("Could not find Anno1800.exe");
        return;
    }

    match args.operation {
        Operation::Load => {
            for process in patricians {
                if let Err(e) = load(process.pid().as_u32()) {
                    error!("Load failed: {:?}", e);
                }
            }
        }
        Operation::Unload => {
            for process in patricians {
                if let Err(e) = unload(process.pid().as_u32()) {
                    error!("Unload failed: {:?}", e);
                }
            }
        }
    }
}

fn load(pid: u32) -> Result<(), Anno1800AgentLoaderError> {
    let path = PCSTR::from_raw(format!(r"{}\anno1800_agent.dll", std::env::current_dir().unwrap().display()).as_ptr());
    unsafe {
        println!("Loading");
        let remote_process = RemoteProcess::new(pid)?;
        let kernel_32 = GetModuleHandleA(s!("Kernel32")).map_err(Anno1800AgentLoaderError::GetModuleHandleAFailed)?;
        let load_library_a_address = GetProcAddress(kernel_32, s!("LoadLibraryA")).ok_or(Anno1800AgentLoaderError::GetProcAddressFailed(GetLastError()))?;

        let mut remote_path = RemoteVirtualAllocation::new(&remote_process, path.as_bytes().len())?;
        remote_path.write(path.as_bytes())?;
        let exit_code = RemoteThread::new(&remote_process, load_library_a_address as usize as _, Some(remote_path.ptr as _))?.wait()?;
        if exit_code == 0 {
            return Err(Anno1800AgentLoaderError::LoadLibraryAFailed);
        }

        let agent_module = LoadLibraryA(path).unwrap();
        run_exported_function(&remote_process, agent_module.0 as _, "start")?;

        info!("Module loaded sucessfully ({:x})", exit_code);
        Ok(())
    }
}

fn unload(pid: u32) -> Result<(), Anno1800AgentLoaderError> {
    let path = PCSTR::from_raw(format!(r"{}\anno1800_agent.dll", std::env::current_dir().unwrap().display()).as_ptr());
    unsafe {
        let remote_process = RemoteProcess::new(pid)?;
        let kernel_32 = GetModuleHandleA(s!("Kernel32")).map_err(Anno1800AgentLoaderError::GetModuleHandleAFailed)?;
        let module_name = s!(r"anno1800_agent.dll");
        let mut buf_ptr = RemoteVirtualAllocation::new(&remote_process, module_name.as_bytes().len())?;
        buf_ptr.write(module_name.as_bytes())?;

        let agent_module = LoadLibraryA(path).unwrap();
        run_exported_function(&remote_process, agent_module.0 as _, "stop")?;

        let free_library_a_address = GetProcAddress(kernel_32, s!("FreeLibrary")).ok_or(Anno1800AgentLoaderError::GetProcAddressFailed(GetLastError()))?;
        let exit_code = RemoteThread::new(&remote_process, free_library_a_address as usize as _, Some(agent_module.0 as _))?.wait()?;
        if exit_code == 0 {
            return Err(Anno1800AgentLoaderError::FreeLibraryFailed(exit_code));
        }

        info!("Module {:#x} freed sucessfully", agent_module.0);

        Ok(())
    }
}

fn run_exported_function(remote_process: &RemoteProcess, base_address: u64, function_name: &str) -> Result<u32, Anno1800AgentLoaderError> {
    debug!("Running {} in module {:016x}", function_name, base_address);
    unsafe {
        // We'd love to call GetProcAddress to obtain the function pointer, but it requires 2 arguments and thus cannot be run with a simple CreateRemoteThread.
        // Instead, we'll go through the EAT, and acquire the function pointer there. Credits to https://www.unknowncheats.me/forum/programming-for-beginners/176139-getprocaddressex.html
        let dos_header: IMAGE_DOS_HEADER = remote_process.read(base_address)?;
        assert_eq!(dos_header.e_magic, 0x5a4d);
        let new_exe_header: IMAGE_NT_HEADERS64 = remote_process.read(base_address + dos_header.e_lfanew as usize as u64)?;
        assert_eq!(new_exe_header.Signature, 0x4550);
        let image_export_directory: IMAGE_EXPORT_DIRECTORY =
            remote_process.read(base_address + new_exe_header.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress as u64)?;
        debug!("{:x?}", image_export_directory);

        let mut ordinal = None;
        for i in 0..image_export_directory.NumberOfNames {
            let name_offset: u32 = remote_process.read(base_address + image_export_directory.AddressOfNames as u64 + 4 * i as u64)?;
            let name = remote_process.read_string(base_address + name_offset as u64)?;
            if name == function_name {
                ordinal = Some(i);
                break;
            }
        }

        if let Some(ordinal) = ordinal {
            let function_offset: u32 = remote_process.read(base_address + image_export_directory.AddressOfFunctions as u64 + 4 * ordinal as u64)?;
            debug!("function_offset={:x}", function_offset);
            RemoteThread::new(remote_process, base_address + function_offset as u64, None)?.wait()
        } else {
            Err(Anno1800AgentLoaderError::ExportedFunctionNotFound)
        }
    }
}
