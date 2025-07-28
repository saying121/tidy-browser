use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStringExt,
    path::PathBuf,
};

use snafu::ResultExt;
use windows::{
    core::BOOL,
    Wdk::System::SystemServices::SE_DEBUG_PRIVILEGE,
    Win32::{
        Foundation::{CloseHandle, HANDLE, NTSTATUS, STATUS_SUCCESS},
        Security::{
            self, DuplicateToken, ImpersonateLoggedOnUser, RevertToSelf, TOKEN_DUPLICATE,
            TOKEN_QUERY,
        },
        System::{
            ProcessStatus::{EnumProcesses, K32GetProcessImageFileNameW},
            Threading::{
                OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            },
        },
    },
};

use crate::error::{self, Result};

pub struct ImpersonateGuard {
    sys_token_handle: HANDLE,
}

impl Drop for ImpersonateGuard {
    fn drop(&mut self) {
        _ = Self::stop();
    }
}

#[link(name = "ntdll")]
extern "system" {
    fn RtlAdjustPrivilege(
        privilege: i32,
        enable: BOOL,
        current_thread: BOOL,
        previous_value: *mut BOOL,
    ) -> NTSTATUS;
}

impl ImpersonateGuard {
    pub fn start(pid: Option<u32>, sys_handle: Option<HANDLE>) -> Result<(Self, u32)> {
        Self::enable_privilege()?;
        let pid = if let Some(pid) = pid {
            pid
        }
        else if let Some(pid) = Self::get_system_pid_list()?.next() {
            pid
        }
        else {
            return Err(error::NotFoundProcessSnafu.build());
        };
        let sys_token = if let Some(handle) = sys_handle {
            handle
        }
        else {
            let system_handle = Self::get_process_handle(pid)?;
            let sys_token = Self::get_system_token(system_handle)?;
            unsafe {
                CloseHandle(system_handle).context(error::CryptUnprotectDataSnafu)?;
            };

            sys_token
        };
        unsafe {
            ImpersonateLoggedOnUser(sys_token).context(error::CryptUnprotectDataSnafu)?;
        };
        Ok((Self { sys_token_handle: sys_token }, pid))
    }

    pub fn stop() -> Result<()> {
        unsafe {
            RevertToSelf().context(error::CryptUnprotectDataSnafu)?;
        };
        Ok(())
    }

    /// stop impersonate and return sys token handle
    pub fn stop_sys_handle(self) -> Result<HANDLE> {
        unsafe { RevertToSelf() }.context(error::CryptUnprotectDataSnafu)?;
        Ok(self.sys_token_handle)
    }

    pub fn close_sys_handle(&self) -> Result<()> {
        unsafe { CloseHandle(self.sys_token_handle) }.context(error::CryptUnprotectDataSnafu)?;
        Ok(())
    }

    fn enable_privilege() -> Result<()> {
        let mut previous_value = BOOL(0);
        let status = unsafe {
            RtlAdjustPrivilege(SE_DEBUG_PRIVILEGE, BOOL(1), BOOL(0), &mut previous_value)
        };
        if status != STATUS_SUCCESS {
            return Err(error::PrivilegeSnafu.build());
        }
        Ok(())
    }

    fn get_system_token(handle: HANDLE) -> Result<HANDLE> {
        let token_handle = unsafe {
            let mut token_handle = HANDLE::default();
            OpenProcessToken(handle, TOKEN_DUPLICATE | TOKEN_QUERY, &mut token_handle)
                .context(error::CryptUnprotectDataSnafu)?;
            token_handle
        };
        let duplicate_token = unsafe {
            let mut duplicate_token = HANDLE::default();
            DuplicateToken(
                token_handle,
                Security::SECURITY_IMPERSONATION_LEVEL(2),
                &mut duplicate_token,
            )
            .context(error::CryptUnprotectDataSnafu)?;
            CloseHandle(token_handle).context(error::CryptUnprotectDataSnafu)?;
            duplicate_token
        };

        Ok(duplicate_token)
    }

    fn process_name_is<F>(pid: u32, name_is: F) -> Result<bool>
    where
        F: FnOnce(&OsStr) -> bool,
    {
        let hprocess = Self::get_process_handle(pid)?;

        let image_file_name = {
            let mut lpimagefilename = vec![0; 260];
            let length =
                unsafe { K32GetProcessImageFileNameW(hprocess, &mut lpimagefilename) } as usize;
            unsafe {
                CloseHandle(hprocess).context(error::CryptUnprotectDataSnafu)?;
            };
            lpimagefilename.truncate(length);
            lpimagefilename
        };

        let fp = OsString::from_wide(&image_file_name);
        PathBuf::from(fp)
            .file_name()
            .map(name_is)
            .ok_or_else(|| error::ProcessPathSnafu.build())
    }

    // https://learn.microsoft.com/en-us/windows/win32/psapi/enumerating-all-processes
    fn get_system_pid_list() -> Result<impl Iterator<Item = u32>> {
        let cap = 1024;
        let mut lpidprocess = Vec::with_capacity(cap);
        let mut lpcbneeded = 0;

        unsafe {
            EnumProcesses(lpidprocess.as_mut_ptr(), cap as u32 * 4, &mut lpcbneeded)
                .context(error::CryptUnprotectDataSnafu)?;
            let c_processes = lpcbneeded as usize / size_of::<u32>();
            lpidprocess.set_len(c_processes);
        };

        let filter = lpidprocess
            .into_iter()
            .filter(|&v| {
                v != 0
                    && Self::process_name_is(v, |n| n == "lsass.exe" || n == "winlogon.exe")
                        .unwrap_or(false)
            });
        Ok(filter)
    }

    fn get_process_handle(pid: u32) -> Result<HANDLE> {
        let hprocess =
            unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) }
                .context(error::CryptUnprotectDataSnafu)?;
        Ok(hprocess)
    }
}
