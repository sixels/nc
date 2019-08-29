//
// Code generated by mksysno_linux.py; DO NOT EDIT.
//

pub type Sysno = usize;

pub const SYS_IO_SETUP: Sysno = 0;
pub const SYS_IO_DESTROY: Sysno = 1;
pub const SYS_IO_SUBMIT: Sysno = 2;
pub const SYS_IO_CANCEL: Sysno = 3;
pub const SYS_IO_GETEVENTS: Sysno = 4;
pub const SYS_SETXATTR: Sysno = 5;
pub const SYS_LSETXATTR: Sysno = 6;
pub const SYS_FSETXATTR: Sysno = 7;
pub const SYS_GETXATTR: Sysno = 8;
pub const SYS_LGETXATTR: Sysno = 9;
pub const SYS_FGETXATTR: Sysno = 10;
pub const SYS_LISTXATTR: Sysno = 11;
pub const SYS_LLISTXATTR: Sysno = 12;
pub const SYS_FLISTXATTR: Sysno = 13;
pub const SYS_REMOVEXATTR: Sysno = 14;
pub const SYS_LREMOVEXATTR: Sysno = 15;
pub const SYS_FREMOVEXATTR: Sysno = 16;
pub const SYS_GETCWD: Sysno = 17;
pub const SYS_LOOKUP_DCOOKIE: Sysno = 18;
pub const SYS_EVENTFD2: Sysno = 19;
pub const SYS_EPOLL_CREATE1: Sysno = 20;
pub const SYS_EPOLL_CTL: Sysno = 21;
pub const SYS_EPOLL_PWAIT: Sysno = 22;
pub const SYS_DUP: Sysno = 23;
pub const SYS_DUP3: Sysno = 24;
pub const SYS_INOTIFY_INIT1: Sysno = 26;
pub const SYS_INOTIFY_ADD_WATCH: Sysno = 27;
pub const SYS_INOTIFY_RM_WATCH: Sysno = 28;
pub const SYS_IOCTL: Sysno = 29;
pub const SYS_IOPRIO_SET: Sysno = 30;
pub const SYS_IOPRIO_GET: Sysno = 31;
pub const SYS_FLOCK: Sysno = 32;
pub const SYS_MKNODAT: Sysno = 33;
pub const SYS_MKDIRAT: Sysno = 34;
pub const SYS_UNLINKAT: Sysno = 35;
pub const SYS_SYMLINKAT: Sysno = 36;
pub const SYS_LINKAT: Sysno = 37;
pub const SYS_RENAMEAT: Sysno = 38;
pub const SYS_UMOUNT2: Sysno = 39;
pub const SYS_MOUNT: Sysno = 40;
pub const SYS_PIVOT_ROOT: Sysno = 41;
pub const SYS_NFSSERVCTL: Sysno = 42;
pub const SYS_FALLOCATE: Sysno = 47;
pub const SYS_FACCESSAT: Sysno = 48;
pub const SYS_CHDIR: Sysno = 49;
pub const SYS_FCHDIR: Sysno = 50;
pub const SYS_CHROOT: Sysno = 51;
pub const SYS_FCHMOD: Sysno = 52;
pub const SYS_FCHMODAT: Sysno = 53;
pub const SYS_FCHOWNAT: Sysno = 54;
pub const SYS_FCHOWN: Sysno = 55;
pub const SYS_OPENAT: Sysno = 56;
pub const SYS_CLOSE: Sysno = 57;
pub const SYS_VHANGUP: Sysno = 58;
pub const SYS_PIPE2: Sysno = 59;
pub const SYS_QUOTACTL: Sysno = 60;
pub const SYS_GETDENTS64: Sysno = 61;
pub const SYS_READ: Sysno = 63;
pub const SYS_WRITE: Sysno = 64;
pub const SYS_READV: Sysno = 65;
pub const SYS_WRITEV: Sysno = 66;
pub const SYS_PREAD64: Sysno = 67;
pub const SYS_PWRITE64: Sysno = 68;
pub const SYS_PREADV: Sysno = 69;
pub const SYS_PWRITEV: Sysno = 70;
pub const SYS_PSELECT6: Sysno = 72;
pub const SYS_PPOLL: Sysno = 73;
pub const SYS_SIGNALFD4: Sysno = 74;
pub const SYS_VMSPLICE: Sysno = 75;
pub const SYS_SPLICE: Sysno = 76;
pub const SYS_TEE: Sysno = 77;
pub const SYS_READLINKAT: Sysno = 78;
pub const SYS_SYNC: Sysno = 81;
pub const SYS_FSYNC: Sysno = 82;
pub const SYS_FDATASYNC: Sysno = 83;
pub const SYS_SYNC_FILE_RANGE: Sysno = 84;
pub const SYS_TIMERFD_CREATE: Sysno = 85;
pub const SYS_TIMERFD_SETTIME: Sysno = 86;
pub const SYS_TIMERFD_GETTIME: Sysno = 87;
pub const SYS_UTIMENSAT: Sysno = 88;
pub const SYS_ACCT: Sysno = 89;
pub const SYS_CAPGET: Sysno = 90;
pub const SYS_CAPSET: Sysno = 91;
pub const SYS_PERSONALITY: Sysno = 92;
pub const SYS_EXIT: Sysno = 93;
pub const SYS_EXIT_GROUP: Sysno = 94;
pub const SYS_WAITID: Sysno = 95;
pub const SYS_SET_TID_ADDRESS: Sysno = 96;
pub const SYS_UNSHARE: Sysno = 97;
pub const SYS_FUTEX: Sysno = 98;
pub const SYS_SET_ROBUST_LIST: Sysno = 99;
pub const SYS_GET_ROBUST_LIST: Sysno = 100;
pub const SYS_NANOSLEEP: Sysno = 101;
pub const SYS_GETITIMER: Sysno = 102;
pub const SYS_SETITIMER: Sysno = 103;
pub const SYS_KEXEC_LOAD: Sysno = 104;
pub const SYS_INIT_MODULE: Sysno = 105;
pub const SYS_DELETE_MODULE: Sysno = 106;
pub const SYS_TIMER_CREATE: Sysno = 107;
pub const SYS_TIMER_GETTIME: Sysno = 108;
pub const SYS_TIMER_GETOVERRUN: Sysno = 109;
pub const SYS_TIMER_SETTIME: Sysno = 110;
pub const SYS_TIMER_DELETE: Sysno = 111;
pub const SYS_CLOCK_SETTIME: Sysno = 112;
pub const SYS_CLOCK_GETTIME: Sysno = 113;
pub const SYS_CLOCK_GETRES: Sysno = 114;
pub const SYS_CLOCK_NANOSLEEP: Sysno = 115;
pub const SYS_SYSLOG: Sysno = 116;
pub const SYS_PTRACE: Sysno = 117;
pub const SYS_SCHED_SETPARAM: Sysno = 118;
pub const SYS_SCHED_SETSCHEDULER: Sysno = 119;
pub const SYS_SCHED_GETSCHEDULER: Sysno = 120;
pub const SYS_SCHED_GETPARAM: Sysno = 121;
pub const SYS_SCHED_SETAFFINITY: Sysno = 122;
pub const SYS_SCHED_GETAFFINITY: Sysno = 123;
pub const SYS_SCHED_YIELD: Sysno = 124;
pub const SYS_SCHED_GET_PRIORITY_MAX: Sysno = 125;
pub const SYS_SCHED_GET_PRIORITY_MIN: Sysno = 126;
pub const SYS_SCHED_RR_GET_INTERVAL: Sysno = 127;
pub const SYS_RESTART_SYSCALL: Sysno = 128;
pub const SYS_KILL: Sysno = 129;
pub const SYS_TKILL: Sysno = 130;
pub const SYS_TGKILL: Sysno = 131;
pub const SYS_SIGALTSTACK: Sysno = 132;
pub const SYS_RT_SIGSUSPEND: Sysno = 133;
pub const SYS_RT_SIGACTION: Sysno = 134;
pub const SYS_RT_SIGPROCMASK: Sysno = 135;
pub const SYS_RT_SIGPENDING: Sysno = 136;
pub const SYS_RT_SIGTIMEDWAIT: Sysno = 137;
pub const SYS_RT_SIGQUEUEINFO: Sysno = 138;
pub const SYS_RT_SIGRETURN: Sysno = 139;
pub const SYS_SETPRIORITY: Sysno = 140;
pub const SYS_GETPRIORITY: Sysno = 141;
pub const SYS_REBOOT: Sysno = 142;
pub const SYS_SETREGID: Sysno = 143;
pub const SYS_SETGID: Sysno = 144;
pub const SYS_SETREUID: Sysno = 145;
pub const SYS_SETUID: Sysno = 146;
pub const SYS_SETRESUID: Sysno = 147;
pub const SYS_GETRESUID: Sysno = 148;
pub const SYS_SETRESGID: Sysno = 149;
pub const SYS_GETRESGID: Sysno = 150;
pub const SYS_SETFSUID: Sysno = 151;
pub const SYS_SETFSGID: Sysno = 152;
pub const SYS_TIMES: Sysno = 153;
pub const SYS_SETPGID: Sysno = 154;
pub const SYS_GETPGID: Sysno = 155;
pub const SYS_GETSID: Sysno = 156;
pub const SYS_SETSID: Sysno = 157;
pub const SYS_GETGROUPS: Sysno = 158;
pub const SYS_SETGROUPS: Sysno = 159;
pub const SYS_UNAME: Sysno = 160;
pub const SYS_SETHOSTNAME: Sysno = 161;
pub const SYS_SETDOMAINNAME: Sysno = 162;
pub const SYS_GETRLIMIT: Sysno = 163;
pub const SYS_SETRLIMIT: Sysno = 164;
pub const SYS_GETRUSAGE: Sysno = 165;
pub const SYS_UMASK: Sysno = 166;
pub const SYS_PRCTL: Sysno = 167;
pub const SYS_GETCPU: Sysno = 168;
pub const SYS_GETTIMEOFDAY: Sysno = 169;
pub const SYS_SETTIMEOFDAY: Sysno = 170;
pub const SYS_ADJTIMEX: Sysno = 171;
pub const SYS_GETPID: Sysno = 172;
pub const SYS_GETPPID: Sysno = 173;
pub const SYS_GETUID: Sysno = 174;
pub const SYS_GETEUID: Sysno = 175;
pub const SYS_GETGID: Sysno = 176;
pub const SYS_GETEGID: Sysno = 177;
pub const SYS_GETTID: Sysno = 178;
pub const SYS_SYSINFO: Sysno = 179;
pub const SYS_MQ_OPEN: Sysno = 180;
pub const SYS_MQ_UNLINK: Sysno = 181;
pub const SYS_MQ_TIMEDSEND: Sysno = 182;
pub const SYS_MQ_TIMEDRECEIVE: Sysno = 183;
pub const SYS_MQ_NOTIFY: Sysno = 184;
pub const SYS_MQ_GETSETATTR: Sysno = 185;
pub const SYS_MSGGET: Sysno = 186;
pub const SYS_MSGCTL: Sysno = 187;
pub const SYS_MSGRCV: Sysno = 188;
pub const SYS_MSGSND: Sysno = 189;
pub const SYS_SEMGET: Sysno = 190;
pub const SYS_SEMCTL: Sysno = 191;
pub const SYS_SEMTIMEDOP: Sysno = 192;
pub const SYS_SEMOP: Sysno = 193;
pub const SYS_SHMGET: Sysno = 194;
pub const SYS_SHMCTL: Sysno = 195;
pub const SYS_SHMAT: Sysno = 196;
pub const SYS_SHMDT: Sysno = 197;
pub const SYS_SOCKET: Sysno = 198;
pub const SYS_SOCKETPAIR: Sysno = 199;
pub const SYS_BIND: Sysno = 200;
pub const SYS_LISTEN: Sysno = 201;
pub const SYS_ACCEPT: Sysno = 202;
pub const SYS_CONNECT: Sysno = 203;
pub const SYS_GETSOCKNAME: Sysno = 204;
pub const SYS_GETPEERNAME: Sysno = 205;
pub const SYS_SENDTO: Sysno = 206;
pub const SYS_RECVFROM: Sysno = 207;
pub const SYS_SETSOCKOPT: Sysno = 208;
pub const SYS_GETSOCKOPT: Sysno = 209;
pub const SYS_SHUTDOWN: Sysno = 210;
pub const SYS_SENDMSG: Sysno = 211;
pub const SYS_RECVMSG: Sysno = 212;
pub const SYS_READAHEAD: Sysno = 213;
pub const SYS_BRK: Sysno = 214;
pub const SYS_MUNMAP: Sysno = 215;
pub const SYS_MREMAP: Sysno = 216;
pub const SYS_ADD_KEY: Sysno = 217;
pub const SYS_REQUEST_KEY: Sysno = 218;
pub const SYS_KEYCTL: Sysno = 219;
pub const SYS_CLONE: Sysno = 220;
pub const SYS_EXECVE: Sysno = 221;
pub const SYS_SWAPON: Sysno = 224;
pub const SYS_SWAPOFF: Sysno = 225;
pub const SYS_MPROTECT: Sysno = 226;
pub const SYS_MSYNC: Sysno = 227;
pub const SYS_MLOCK: Sysno = 228;
pub const SYS_MUNLOCK: Sysno = 229;
pub const SYS_MLOCKALL: Sysno = 230;
pub const SYS_MUNLOCKALL: Sysno = 231;
pub const SYS_MINCORE: Sysno = 232;
pub const SYS_MADVISE: Sysno = 233;
pub const SYS_REMAP_FILE_PAGES: Sysno = 234;
pub const SYS_MBIND: Sysno = 235;
pub const SYS_GET_MEMPOLICY: Sysno = 236;
pub const SYS_SET_MEMPOLICY: Sysno = 237;
pub const SYS_MIGRATE_PAGES: Sysno = 238;
pub const SYS_MOVE_PAGES: Sysno = 239;
pub const SYS_RT_TGSIGQUEUEINFO: Sysno = 240;
pub const SYS_PERF_EVENT_OPEN: Sysno = 241;
pub const SYS_ACCEPT4: Sysno = 242;
pub const SYS_RECVMMSG: Sysno = 243;
pub const SYS_ARCH_SPECIFIC_SYSCALL: Sysno = 244;
pub const SYS_WAIT4: Sysno = 260;
pub const SYS_PRLIMIT64: Sysno = 261;
pub const SYS_FANOTIFY_INIT: Sysno = 262;
pub const SYS_FANOTIFY_MARK: Sysno = 263;
pub const SYS_NAME_TO_HANDLE_AT: Sysno = 264;
pub const SYS_OPEN_BY_HANDLE_AT: Sysno = 265;
pub const SYS_CLOCK_ADJTIME: Sysno = 266;
pub const SYS_SYNCFS: Sysno = 267;
pub const SYS_SETNS: Sysno = 268;
pub const SYS_SENDMMSG: Sysno = 269;
pub const SYS_PROCESS_VM_READV: Sysno = 270;
pub const SYS_PROCESS_VM_WRITEV: Sysno = 271;
pub const SYS_KCMP: Sysno = 272;
pub const SYS_FINIT_MODULE: Sysno = 273;
pub const SYS_SCHED_SETATTR: Sysno = 274;
pub const SYS_SCHED_GETATTR: Sysno = 275;
pub const SYS_RENAMEAT2: Sysno = 276;
pub const SYS_SECCOMP: Sysno = 277;
pub const SYS_GETRANDOM: Sysno = 278;
pub const SYS_MEMFD_CREATE: Sysno = 279;
pub const SYS_BPF: Sysno = 280;
pub const SYS_EXECVEAT: Sysno = 281;
pub const SYS_USERFAULTFD: Sysno = 282;
pub const SYS_MEMBARRIER: Sysno = 283;
pub const SYS_MLOCK2: Sysno = 284;
pub const SYS_COPY_FILE_RANGE: Sysno = 285;
pub const SYS_PREADV2: Sysno = 286;
pub const SYS_PWRITEV2: Sysno = 287;
pub const SYS_PKEY_MPROTECT: Sysno = 288;
pub const SYS_PKEY_ALLOC: Sysno = 289;
pub const SYS_PKEY_FREE: Sysno = 290;
pub const SYS_STATX: Sysno = 291;
pub const SYS_IO_PGETEVENTS: Sysno = 292;
pub const SYS_RSEQ: Sysno = 293;
pub const SYS_KEXEC_FILE_LOAD: Sysno = 294;
pub const SYS_PIDFD_SEND_SIGNAL: Sysno = 424;
pub const SYS_IO_URING_SETUP: Sysno = 425;
pub const SYS_IO_URING_ENTER: Sysno = 426;
pub const SYS_IO_URING_REGISTER: Sysno = 427;
pub const SYS_OPEN_TREE: Sysno = 428;
pub const SYS_MOVE_MOUNT: Sysno = 429;
pub const SYS_FSOPEN: Sysno = 430;
pub const SYS_FSCONFIG: Sysno = 431;
pub const SYS_FSMOUNT: Sysno = 432;
pub const SYS_FSPICK: Sysno = 433;
pub const SYS_SYSCALLS: Sysno = 434;
