/// Create a child process.
pub unsafe fn clone(
    clone_flags: usize,
    new_sp: *const core::ffi::c_void,
    parent_tid: Option<&mut pid_t>,
    child_tid: Option<&mut pid_t>,
    tls: Option<*const core::ffi::c_void>,
) -> Result<pid_t, Errno> {
    use core::ptr::null_mut;
    let new_sp = new_sp as usize;
    let parent_tid_ptr = parent_tid.map_or(null_mut::<pid_t>() as usize, |parent_tid| {
        parent_tid as *mut pid_t as usize
    });
    let child_tid_ptr = child_tid.map_or(null_mut::<pid_t>() as usize, |child_tid| {
        child_tid as *mut pid_t as usize
    });
    let tls_ptr = tls.map_or(core::ptr::null::<u8>() as usize, |tls| tls as usize);
    syscall5(
        SYS_CLONE,
        clone_flags,
        new_sp,
        parent_tid_ptr,
        child_tid_ptr,
        tls_ptr,
    )
    .map(|ret| ret as pid_t)
}
