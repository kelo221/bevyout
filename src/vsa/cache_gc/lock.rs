use anyhow::{Context, Result, bail};
use std::{fs, fs::File, fs::OpenOptions, io::Write, path::PathBuf};

pub(crate) const CACHE_GC_LOCK_NAME: &str = ".cache-gc.lock";

pub(crate) struct CacheGcLock {
    path: PathBuf,
    file: File,
}

impl CacheGcLock {
    pub(crate) fn acquire(cache_root: &std::path::Path) -> Result<Self> {
        let path = cache_root.join(CACHE_GC_LOCK_NAME);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .with_context(|| format!("opening cache GC lock {}", path.display()))?;
        if let Err(error) = try_lock_exclusive(&file) {
            bail!(
                "cache garbage collection lock is held by another process: {}: {error}",
                path.display()
            );
        }
        file.set_len(0)?;
        writeln!(file, "pid={}", std::process::id())?;
        file.sync_all()?;
        Ok(Self { path, file })
    }
}

impl Drop for CacheGcLock {
    fn drop(&mut self) {
        let _ = unlock(&self.file);
        let _ = fs::remove_file(&self.path);
    }
}

#[cfg(windows)]
fn try_lock_exclusive(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY, LockFileEx,
    };
    let mut overlapped = unsafe { std::mem::zeroed() };
    // SAFETY: the handle is valid and `overlapped` remains writable for the
    // synchronous nonblocking lock request.
    let result = unsafe {
        LockFileEx(
            file.as_raw_handle(),
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            u32::MAX,
            u32::MAX,
            &mut overlapped,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn unlock(file: &File) -> std::io::Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::UnlockFileEx;
    let mut overlapped = unsafe { std::mem::zeroed() };
    // SAFETY: this uses the same handle and byte range acquired above.
    let result =
        unsafe { UnlockFileEx(file.as_raw_handle(), 0, u32::MAX, u32::MAX, &mut overlapped) };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn try_lock_exclusive(file: &File) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    // SAFETY: `file` owns a valid descriptor for the duration of this call.
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(unix)]
fn unlock(file: &File) -> std::io::Result<()> {
    use std::os::fd::AsRawFd;
    // SAFETY: `file` owns the descriptor whose advisory lock is released.
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(not(any(windows, unix)))]
fn try_lock_exclusive(_file: &File) -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(any(windows, unix)))]
fn unlock(_file: &File) -> std::io::Result<()> {
    Ok(())
}
