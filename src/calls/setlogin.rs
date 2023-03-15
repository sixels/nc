/// Set login name.
pub unsafe fn setlogin(name: &str) -> Result<(), Errno> {
    let name = CString::new(name.as_ref());
    let name_ptr = name.as_ptr() as usize;
    syscall1(SYS_SETLOGIN, name_ptr).map(drop)
}
