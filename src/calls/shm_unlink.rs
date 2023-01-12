/// Removes a shared memory object named `path`.
pub unsafe fn shm_unlink<P: AsRef<Path>>(path: P) -> Result<(), Errno> {
    let path = CString::new(path.as_ref());
    let path_ptr = path.as_ptr() as usize;
    syscall1(SYS_SHM_UNLINK, path_ptr).map(drop)
}
