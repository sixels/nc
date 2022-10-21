// Code generated by mksysnum_linux.py; DO NOT EDIT.

use crate::syscalls::Errno;

/// Operation not permitted
pub const EPERM: Errno = 1;
/// No such file or directory
pub const ENOENT: Errno = 2;
/// No such process
pub const ESRCH: Errno = 3;
/// Interrupted system call
pub const EINTR: Errno = 4;
/// I/O error
pub const EIO: Errno = 5;
/// No such device or address
pub const ENXIO: Errno = 6;
/// Argument list too long
pub const E2BIG: Errno = 7;
/// Exec format error
pub const ENOEXEC: Errno = 8;
/// Bad file number
pub const EBADF: Errno = 9;
/// No child processes
pub const ECHILD: Errno = 10;
/// Try again
pub const EAGAIN: Errno = 11;
/// Out of memory
pub const ENOMEM: Errno = 12;
/// Permission denied
pub const EACCES: Errno = 13;
/// Bad address
pub const EFAULT: Errno = 14;
/// Block device required
pub const ENOTBLK: Errno = 15;
/// Device or resource busy
pub const EBUSY: Errno = 16;
/// File exists
pub const EEXIST: Errno = 17;
/// Cross-device link
pub const EXDEV: Errno = 18;
/// No such device
pub const ENODEV: Errno = 19;
/// Not a directory
pub const ENOTDIR: Errno = 20;
/// Is a directory
pub const EISDIR: Errno = 21;
/// Invalid argument
pub const EINVAL: Errno = 22;
/// File table overflow
pub const ENFILE: Errno = 23;
/// Too many open files
pub const EMFILE: Errno = 24;
/// Not a typewriter
pub const ENOTTY: Errno = 25;
/// Text file busy
pub const ETXTBSY: Errno = 26;
/// File too large
pub const EFBIG: Errno = 27;
/// No space left on device
pub const ENOSPC: Errno = 28;
/// Illegal seek
pub const ESPIPE: Errno = 29;
/// Read-only file system
pub const EROFS: Errno = 30;
/// Too many links
pub const EMLINK: Errno = 31;
/// Broken pipe
pub const EPIPE: Errno = 32;
/// Math argument out of domain of func
pub const EDOM: Errno = 33;
/// Math result not representable
pub const ERANGE: Errno = 34;
/// Resource deadlock would occur
pub const EDEADLK: Errno = 35;
/// File name too long
pub const ENAMETOOLONG: Errno = 36;
/// No record locks available
pub const ENOLCK: Errno = 37;
/// Invalid system call number
pub const ENOSYS: Errno = 38;
/// Directory not empty
pub const ENOTEMPTY: Errno = 39;
/// Too many symbolic links encountered
pub const ELOOP: Errno = 40;
pub const EWOULDBLOCK: Errno = EAGAIN;
/// No message of desired type
pub const ENOMSG: Errno = 42;
/// Identifier removed
pub const EIDRM: Errno = 43;
/// Channel number out of range
pub const ECHRNG: Errno = 44;
/// Level 2 not synchronized
pub const EL2NSYNC: Errno = 45;
/// Level 3 halted
pub const EL3HLT: Errno = 46;
/// Level 3 reset
pub const EL3RST: Errno = 47;
/// Link number out of range
pub const ELNRNG: Errno = 48;
/// Protocol driver not attached
pub const EUNATCH: Errno = 49;
/// No CSI structure available
pub const ENOCSI: Errno = 50;
/// Level 2 halted
pub const EL2HLT: Errno = 51;
/// Invalid exchange
pub const EBADE: Errno = 52;
/// Invalid request descriptor
pub const EBADR: Errno = 53;
/// Exchange full
pub const EXFULL: Errno = 54;
/// No anode
pub const ENOANO: Errno = 55;
/// Invalid request code
pub const EBADRQC: Errno = 56;
/// Invalid slot
pub const EBADSLT: Errno = 57;
pub const EDEADLOCK: Errno = EDEADLK;
/// Bad font file format
pub const EBFONT: Errno = 59;
/// Device not a stream
pub const ENOSTR: Errno = 60;
/// No data available
pub const ENODATA: Errno = 61;
/// Timer expired
pub const ETIME: Errno = 62;
/// Out of streams resources
pub const ENOSR: Errno = 63;
/// Machine is not on the network
pub const ENONET: Errno = 64;
/// Package not installed
pub const ENOPKG: Errno = 65;
/// Object is remote
pub const EREMOTE: Errno = 66;
/// Link has been severed
pub const ENOLINK: Errno = 67;
/// Advertise error
pub const EADV: Errno = 68;
/// Srmount error
pub const ESRMNT: Errno = 69;
/// Communication error on send
pub const ECOMM: Errno = 70;
/// Protocol error
pub const EPROTO: Errno = 71;
/// Multihop attempted
pub const EMULTIHOP: Errno = 72;
/// RFS specific error
pub const EDOTDOT: Errno = 73;
/// Not a data message
pub const EBADMSG: Errno = 74;
/// Value too large for defined data type
pub const EOVERFLOW: Errno = 75;
/// Name not unique on network
pub const ENOTUNIQ: Errno = 76;
/// File descriptor in bad state
pub const EBADFD: Errno = 77;
/// Remote address changed
pub const EREMCHG: Errno = 78;
/// Can not access a needed shared library
pub const ELIBACC: Errno = 79;
/// Accessing a corrupted shared library
pub const ELIBBAD: Errno = 80;
/// .lib section in a.out corrupted
pub const ELIBSCN: Errno = 81;
/// Attempting to link in too many shared libraries
pub const ELIBMAX: Errno = 82;
/// Cannot exec a shared library directly
pub const ELIBEXEC: Errno = 83;
/// Illegal byte sequence
pub const EILSEQ: Errno = 84;
/// Interrupted system call should be restarted
pub const ERESTART: Errno = 85;
/// Streams pipe error
pub const ESTRPIPE: Errno = 86;
/// Too many users
pub const EUSERS: Errno = 87;
/// Socket operation on non-socket
pub const ENOTSOCK: Errno = 88;
/// Destination address required
pub const EDESTADDRREQ: Errno = 89;
/// Message too long
pub const EMSGSIZE: Errno = 90;
/// Protocol wrong type for socket
pub const EPROTOTYPE: Errno = 91;
/// Protocol not available
pub const ENOPROTOOPT: Errno = 92;
/// Protocol not supported
pub const EPROTONOSUPPORT: Errno = 93;
/// Socket type not supported
pub const ESOCKTNOSUPPORT: Errno = 94;
/// Operation not supported on transport endpoint
pub const EOPNOTSUPP: Errno = 95;
/// Protocol family not supported
pub const EPFNOSUPPORT: Errno = 96;
/// Address family not supported by protocol
pub const EAFNOSUPPORT: Errno = 97;
/// Address already in use
pub const EADDRINUSE: Errno = 98;
/// Cannot assign requested address
pub const EADDRNOTAVAIL: Errno = 99;
/// Network is down
pub const ENETDOWN: Errno = 100;
/// Network is unreachable
pub const ENETUNREACH: Errno = 101;
/// Network dropped connection because of reset
pub const ENETRESET: Errno = 102;
/// Software caused connection abort
pub const ECONNABORTED: Errno = 103;
/// Connection reset by peer
pub const ECONNRESET: Errno = 104;
/// No buffer space available
pub const ENOBUFS: Errno = 105;
/// Transport endpoint is already connected
pub const EISCONN: Errno = 106;
/// Transport endpoint is not connected
pub const ENOTCONN: Errno = 107;
/// Cannot send after transport endpoint shutdown
pub const ESHUTDOWN: Errno = 108;
/// Too many references: cannot splice
pub const ETOOMANYREFS: Errno = 109;
/// Connection timed out
pub const ETIMEDOUT: Errno = 110;
/// Connection refused
pub const ECONNREFUSED: Errno = 111;
/// Host is down
pub const EHOSTDOWN: Errno = 112;
/// No route to host
pub const EHOSTUNREACH: Errno = 113;
/// Operation already in progress
pub const EALREADY: Errno = 114;
/// Operation now in progress
pub const EINPROGRESS: Errno = 115;
/// Stale file handle
pub const ESTALE: Errno = 116;
/// Structure needs cleaning
pub const EUCLEAN: Errno = 117;
/// Not a XENIX named type file
pub const ENOTNAM: Errno = 118;
/// No XENIX semaphores available
pub const ENAVAIL: Errno = 119;
/// Is a named type file
pub const EISNAM: Errno = 120;
/// Remote I/O error
pub const EREMOTEIO: Errno = 121;
/// Quota exceeded
pub const EDQUOT: Errno = 122;
/// No medium found
pub const ENOMEDIUM: Errno = 123;
/// Wrong medium type
pub const EMEDIUMTYPE: Errno = 124;
/// Operation Canceled
pub const ECANCELED: Errno = 125;
/// Required key not available
pub const ENOKEY: Errno = 126;
/// Key has expired
pub const EKEYEXPIRED: Errno = 127;
/// Key has been revoked
pub const EKEYREVOKED: Errno = 128;
/// Key was rejected by service
pub const EKEYREJECTED: Errno = 129;
/// Owner died
pub const EOWNERDEAD: Errno = 130;
/// State not recoverable
pub const ENOTRECOVERABLE: Errno = 131;
/// Operation not possible due to RF-kill
pub const ERFKILL: Errno = 132;
/// Memory page has hardware error
pub const EHWPOISON: Errno = 133;

/// Get errno description.
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn strerror(errno: Errno) -> &'static str {
    match errno {
        EPERM => "Operation not permitted",
        ENOENT => "No such file or directory",
        ESRCH => "No such process",
        EINTR => "Interrupted system call",
        EIO => "I/O error",
        ENXIO => "No such device or address",
        E2BIG => "Argument list too long",
        ENOEXEC => "Exec format error",
        EBADF => "Bad file number",
        ECHILD => "No child processes",
        EAGAIN => "Try again",
        ENOMEM => "Out of memory",
        EACCES => "Permission denied",
        EFAULT => "Bad address",
        ENOTBLK => "Block device required",
        EBUSY => "Device or resource busy",
        EEXIST => "File exists",
        EXDEV => "Cross-device link",
        ENODEV => "No such device",
        ENOTDIR => "Not a directory",
        EISDIR => "Is a directory",
        EINVAL => "Invalid argument",
        ENFILE => "File table overflow",
        EMFILE => "Too many open files",
        ENOTTY => "Not a typewriter",
        ETXTBSY => "Text file busy",
        EFBIG => "File too large",
        ENOSPC => "No space left on device",
        ESPIPE => "Illegal seek",
        EROFS => "Read-only file system",
        EMLINK => "Too many links",
        EPIPE => "Broken pipe",
        EDOM => "Math argument out of domain of func",
        ERANGE => "Math result not representable",
        EDEADLK => "Resource deadlock would occur",
        ENAMETOOLONG => "File name too long",
        ENOLCK => "No record locks available",
        ENOSYS => "Invalid system call number",
        ENOTEMPTY => "Directory not empty",
        ELOOP => "Too many symbolic links encountered",
        ENOMSG => "No message of desired type",
        EIDRM => "Identifier removed",
        ECHRNG => "Channel number out of range",
        EL2NSYNC => "Level 2 not synchronized",
        EL3HLT => "Level 3 halted",
        EL3RST => "Level 3 reset",
        ELNRNG => "Link number out of range",
        EUNATCH => "Protocol driver not attached",
        ENOCSI => "No CSI structure available",
        EL2HLT => "Level 2 halted",
        EBADE => "Invalid exchange",
        EBADR => "Invalid request descriptor",
        EXFULL => "Exchange full",
        ENOANO => "No anode",
        EBADRQC => "Invalid request code",
        EBADSLT => "Invalid slot",
        EBFONT => "Bad font file format",
        ENOSTR => "Device not a stream",
        ENODATA => "No data available",
        ETIME => "Timer expired",
        ENOSR => "Out of streams resources",
        ENONET => "Machine is not on the network",
        ENOPKG => "Package not installed",
        EREMOTE => "Object is remote",
        ENOLINK => "Link has been severed",
        EADV => "Advertise error",
        ESRMNT => "Srmount error",
        ECOMM => "Communication error on send",
        EPROTO => "Protocol error",
        EMULTIHOP => "Multihop attempted",
        EDOTDOT => "RFS specific error",
        EBADMSG => "Not a data message",
        EOVERFLOW => "Value too large for defined data type",
        ENOTUNIQ => "Name not unique on network",
        EBADFD => "File descriptor in bad state",
        EREMCHG => "Remote address changed",
        ELIBACC => "Can not access a needed shared library",
        ELIBBAD => "Accessing a corrupted shared library",
        ELIBSCN => ".lib section in a.out corrupted",
        ELIBMAX => "Attempting to link in too many shared libraries",
        ELIBEXEC => "Cannot exec a shared library directly",
        EILSEQ => "Illegal byte sequence",
        ERESTART => "Interrupted system call should be restarted",
        ESTRPIPE => "Streams pipe error",
        EUSERS => "Too many users",
        ENOTSOCK => "Socket operation on non-socket",
        EDESTADDRREQ => "Destination address required",
        EMSGSIZE => "Message too long",
        EPROTOTYPE => "Protocol wrong type for socket",
        ENOPROTOOPT => "Protocol not available",
        EPROTONOSUPPORT => "Protocol not supported",
        ESOCKTNOSUPPORT => "Socket type not supported",
        EOPNOTSUPP => "Operation not supported on transport endpoint",
        EPFNOSUPPORT => "Protocol family not supported",
        EAFNOSUPPORT => "Address family not supported by protocol",
        EADDRINUSE => "Address already in use",
        EADDRNOTAVAIL => "Cannot assign requested address",
        ENETDOWN => "Network is down",
        ENETUNREACH => "Network is unreachable",
        ENETRESET => "Network dropped connection because of reset",
        ECONNABORTED => "Software caused connection abort",
        ECONNRESET => "Connection reset by peer",
        ENOBUFS => "No buffer space available",
        EISCONN => "Transport endpoint is already connected",
        ENOTCONN => "Transport endpoint is not connected",
        ESHUTDOWN => "Cannot send after transport endpoint shutdown",
        ETOOMANYREFS => "Too many references: cannot splice",
        ETIMEDOUT => "Connection timed out",
        ECONNREFUSED => "Connection refused",
        EHOSTDOWN => "Host is down",
        EHOSTUNREACH => "No route to host",
        EALREADY => "Operation already in progress",
        EINPROGRESS => "Operation now in progress",
        ESTALE => "Stale file handle",
        EUCLEAN => "Structure needs cleaning",
        ENOTNAM => "Not a XENIX named type file",
        ENAVAIL => "No XENIX semaphores available",
        EISNAM => "Is a named type file",
        EREMOTEIO => "Remote I/O error",
        EDQUOT => "Quota exceeded",
        ENOMEDIUM => "No medium found",
        EMEDIUMTYPE => "Wrong medium type",
        ECANCELED => "Operation Canceled",
        ENOKEY => "Required key not available",
        EKEYEXPIRED => "Key has expired",
        EKEYREVOKED => "Key has been revoked",
        EKEYREJECTED => "Key was rejected by service",
        EOWNERDEAD => "Owner died",
        ENOTRECOVERABLE => "State not recoverable",
        ERFKILL => "Operation not possible due to RF-kill",
        EHWPOISON => "Memory page has hardware error",

        _ => "Unknown errno!",
    }
}
