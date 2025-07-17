use std::{
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStringExt,
    path::PathBuf,
};

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

use crate::error::{CryptError, Result};

pub struct ImpersonateGuard {
    handle: HANDLE,
}

impl Drop for ImpersonateGuard {
    fn drop(&mut self) {
        _ = self.stop_impersonate();
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
    pub fn start_impersonate() -> Result<Self> {
        Self::enable_privilege()?;
        let Some(pid) = Self::get_system_pid_list()?.next()
        else {
            return Err(CryptError::WindowsNotFoundProcess);
        };
        let system_handle = Self::get_process_handle(pid)?;
        let dup_token = Self::get_system_token(system_handle)?;
        unsafe {
            CloseHandle(system_handle)?;
            ImpersonateLoggedOnUser(dup_token)?;
        };
        Ok(Self { handle: dup_token })
    }

    pub fn stop_impersonate(&mut self) -> Result<()> {
        unsafe {
            CloseHandle(self.handle)?;
            RevertToSelf()?;
        };
        Ok(())
    }

    fn enable_privilege() -> Result<()> {
        let mut previous_value = BOOL(0);
        let status = unsafe {
            RtlAdjustPrivilege(SE_DEBUG_PRIVILEGE, BOOL(1), BOOL(0), &mut previous_value)
        };
        if status != STATUS_SUCCESS {
            return Err(CryptError::WindowsPrivilege);
        }
        Ok(())
    }

    fn get_system_token(handle: HANDLE) -> Result<HANDLE> {
        let mut token_handle = HANDLE::default();
        unsafe { OpenProcessToken(handle, TOKEN_DUPLICATE | TOKEN_QUERY, &mut token_handle)? };
        let mut duplicate_token = HANDLE::default();
        unsafe {
            DuplicateToken(
                token_handle,
                Security::SECURITY_IMPERSONATION_LEVEL(2),
                &mut duplicate_token,
            )?;
            CloseHandle(token_handle)?;
        };

        Ok(duplicate_token)
    }

    fn target_process_name<F>(pid: u32, name: F) -> Result<bool>
    where
        F: FnOnce(&OsStr) -> bool,
    {
        let hprocess = Self::get_process_handle(pid)?;

        let mut lpimagefilename = vec![0; 260];
        let length =
            unsafe { K32GetProcessImageFileNameW(hprocess, &mut lpimagefilename) } as usize;
        unsafe {
            CloseHandle(hprocess)?;
        };
        lpimagefilename.truncate(length);

        let fp = OsString::from_wide(&lpimagefilename);
        PathBuf::from(fp)
            .file_name()
            .map(name)
            .ok_or(CryptError::WindowsStr)
    }

    // https://learn.microsoft.com/en-us/windows/win32/psapi/enumerating-all-processes
    fn get_system_pid_list() -> Result<impl Iterator<Item = u32>> {
        let cap = 1024;
        let mut lpidprocess = Vec::with_capacity(cap);
        let mut lpcbneeded = 0;

        unsafe {
            EnumProcesses(lpidprocess.as_mut_ptr(), cap as u32 * 4, &mut lpcbneeded)?;
            let c_processes = lpcbneeded as usize / size_of::<u32>();
            lpidprocess.set_len(c_processes);
        };

        let filter = lpidprocess
            .into_iter()
            .filter(|&v| {
                v != 0
                    && Self::target_process_name(v, |n| n == "lsass.exe" || n == "winlogon.exe")
                        .unwrap_or(false)
            });
        Ok(filter)
    }

    fn get_process_handle(pid: u32) -> Result<HANDLE> {
        let hprocess =
            unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) }?;
        if hprocess.is_invalid() {
            return Err(windows::core::Error::from_win32().into());
        }
        Ok(hprocess)
    }
}
