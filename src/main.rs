#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[repr(C)]
struct LUID {
    low_part: u32,
    high_part: i32,
}

#[repr(C)]
struct LUID_AND_ATTRIBUTES {
    luid: LUID,
    attributes: u32,
}

#[repr(C)]
struct TOKEN_PRIVILEGES {
    privilege_count: u32,
    privileges: [LUID_AND_ATTRIBUTES; 1],
}

type HANDLE = *mut c_void;
type BOOL = i32;
type DWORD = u32;
type NTSTATUS = i32;

const SE_PRIVILEGE_ENABLED: u32 = 0x00000002;
const TOKEN_ADJUST_PRIVILEGES: u32 = 0x00000020;
const POWER_ACTION_SHUTDOWN_RESET: i32 = 2;
const POWER_ACTION_SHUTDOWN: i32 = 4;
const POWER_LEVEL_SYSTEM_SHUTDOWN: i32 = 5;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> HANDLE;
    fn ExitProcess(exit_code: u32) -> !;
    fn GetCommandLineW() -> *const u16;
}

#[link(name = "advapi32")]
unsafe extern "system" {
    fn OpenProcessToken(
        process_handle: HANDLE,
        desired_access: DWORD,
        token_handle: *mut HANDLE,
    ) -> BOOL;
    fn LookupPrivilegeValueW(system_name: *const u16, name: *const u16, luid: *mut LUID) -> BOOL;
    fn AdjustTokenPrivileges(
        token_handle: HANDLE,
        disable_all: BOOL,
        new_state: *const TOKEN_PRIVILEGES,
        buffer_length: DWORD,
        previous_state: *mut TOKEN_PRIVILEGES,
        return_length: *mut DWORD,
    ) -> BOOL;
}

#[link(name = "ntdll")]
unsafe extern "system" {
    fn NtSetSystemPowerState(power_action: i32, system_state: i32, flags: u32) -> NTSTATUS;
}

unsafe fn has_shutdown_flag() -> bool {
    unsafe {
        let cmdline = GetCommandLineW();
        if cmdline.is_null() {
            return false;
        }

        let mut ptr = cmdline;
        let target = [b'-' as u16, b's' as u16];

        while *ptr != 0 {
            if *ptr == target[0] && *ptr.offset(1) == target[1] {
                let after = *ptr.offset(2);
                if after == 0 || after == b' ' as u16 || after == b'\t' as u16 {
                    return true;
                }
            }
            ptr = ptr.offset(1);
        }

        false
    }
}

unsafe fn enable_shutdown_privilege() -> bool {
    unsafe {
        let mut token: HANDLE = core::ptr::null_mut();

        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES, &mut token) == 0 {
            return false;
        }

        let privilege_name: [u16; 19] = [
            b'S' as u16,
            b'e' as u16,
            b'S' as u16,
            b'h' as u16,
            b'u' as u16,
            b't' as u16,
            b'd' as u16,
            b'o' as u16,
            b'w' as u16,
            b'n' as u16,
            b'P' as u16,
            b'r' as u16,
            b'i' as u16,
            b'v' as u16,
            b'i' as u16,
            b'l' as u16,
            b'e' as u16,
            b'g' as u16,
            b'e' as u16,
        ];

        let mut luid = LUID {
            low_part: 0,
            high_part: 0,
        };

        if LookupPrivilegeValueW(core::ptr::null(), privilege_name.as_ptr(), &mut luid) == 0 {
            return false;
        }

        let privileges = TOKEN_PRIVILEGES {
            privilege_count: 1,
            privileges: [LUID_AND_ATTRIBUTES {
                luid,
                attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        AdjustTokenPrivileges(
            token,
            0,
            &privileges,
            0,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ) != 0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() -> ! {
    unsafe {
        if !enable_shutdown_privilege() {
            ExitProcess(1);
        }

        let power_action = if has_shutdown_flag() {
            POWER_ACTION_SHUTDOWN
        } else {
            POWER_ACTION_SHUTDOWN_RESET
        };

        NtSetSystemPowerState(power_action, POWER_LEVEL_SYSTEM_SHUTDOWN, 0);

        ExitProcess(0);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { ExitProcess(2) }
}
