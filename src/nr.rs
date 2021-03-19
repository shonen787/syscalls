// AUTOMATICALLY GENERATED. DO NOT EDIT.

pub use self::SyscallNo::*;
use core::fmt;
#[cfg(feature = "serde_repr")]
use serde_repr::{Deserialize_repr, Serialize_repr};
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde_repr", derive(Serialize_repr, Deserialize_repr))]
#[repr(i32)]
pub enum SyscallNo {
    SYS_read = 0,
    SYS_write = 1,
    SYS_open = 2,
    SYS_close = 3,
    SYS_stat = 4,
    SYS_fstat = 5,
    SYS_lstat = 6,
    SYS_poll = 7,
    SYS_lseek = 8,
    SYS_mmap = 9,
    SYS_mprotect = 10,
    SYS_munmap = 11,
    SYS_brk = 12,
    SYS_rt_sigaction = 13,
    SYS_rt_sigprocmask = 14,
    SYS_rt_sigreturn = 15,
    SYS_ioctl = 16,
    SYS_pread64 = 17,
    SYS_pwrite64 = 18,
    SYS_readv = 19,
    SYS_writev = 20,
    SYS_access = 21,
    SYS_pipe = 22,
    SYS_select = 23,
    SYS_sched_yield = 24,
    SYS_mremap = 25,
    SYS_msync = 26,
    SYS_mincore = 27,
    SYS_madvise = 28,
    SYS_shmget = 29,
    SYS_shmat = 30,
    SYS_shmctl = 31,
    SYS_dup = 32,
    SYS_dup2 = 33,
    SYS_pause = 34,
    SYS_nanosleep = 35,
    SYS_getitimer = 36,
    SYS_alarm = 37,
    SYS_setitimer = 38,
    SYS_getpid = 39,
    SYS_sendfile = 40,
    SYS_socket = 41,
    SYS_connect = 42,
    SYS_accept = 43,
    SYS_sendto = 44,
    SYS_recvfrom = 45,
    SYS_sendmsg = 46,
    SYS_recvmsg = 47,
    SYS_shutdown = 48,
    SYS_bind = 49,
    SYS_listen = 50,
    SYS_getsockname = 51,
    SYS_getpeername = 52,
    SYS_socketpair = 53,
    SYS_setsockopt = 54,
    SYS_getsockopt = 55,
    SYS_clone = 56,
    SYS_fork = 57,
    SYS_vfork = 58,
    SYS_execve = 59,
    SYS_exit = 60,
    SYS_wait4 = 61,
    SYS_kill = 62,
    SYS_uname = 63,
    SYS_semget = 64,
    SYS_semop = 65,
    SYS_semctl = 66,
    SYS_shmdt = 67,
    SYS_msgget = 68,
    SYS_msgsnd = 69,
    SYS_msgrcv = 70,
    SYS_msgctl = 71,
    SYS_fcntl = 72,
    SYS_flock = 73,
    SYS_fsync = 74,
    SYS_fdatasync = 75,
    SYS_truncate = 76,
    SYS_ftruncate = 77,
    SYS_getdents = 78,
    SYS_getcwd = 79,
    SYS_chdir = 80,
    SYS_fchdir = 81,
    SYS_rename = 82,
    SYS_mkdir = 83,
    SYS_rmdir = 84,
    SYS_creat = 85,
    SYS_link = 86,
    SYS_unlink = 87,
    SYS_symlink = 88,
    SYS_readlink = 89,
    SYS_chmod = 90,
    SYS_fchmod = 91,
    SYS_chown = 92,
    SYS_fchown = 93,
    SYS_lchown = 94,
    SYS_umask = 95,
    SYS_gettimeofday = 96,
    SYS_getrlimit = 97,
    SYS_getrusage = 98,
    SYS_sysinfo = 99,
    SYS_times = 100,
    SYS_ptrace = 101,
    SYS_getuid = 102,
    SYS_syslog = 103,
    SYS_getgid = 104,
    SYS_setuid = 105,
    SYS_setgid = 106,
    SYS_geteuid = 107,
    SYS_getegid = 108,
    SYS_setpgid = 109,
    SYS_getppid = 110,
    SYS_getpgrp = 111,
    SYS_setsid = 112,
    SYS_setreuid = 113,
    SYS_setregid = 114,
    SYS_getgroups = 115,
    SYS_setgroups = 116,
    SYS_setresuid = 117,
    SYS_getresuid = 118,
    SYS_setresgid = 119,
    SYS_getresgid = 120,
    SYS_getpgid = 121,
    SYS_setfsuid = 122,
    SYS_setfsgid = 123,
    SYS_getsid = 124,
    SYS_capget = 125,
    SYS_capset = 126,
    SYS_rt_sigpending = 127,
    SYS_rt_sigtimedwait = 128,
    SYS_rt_sigqueueinfo = 129,
    SYS_rt_sigsuspend = 130,
    SYS_sigaltstack = 131,
    SYS_utime = 132,
    SYS_mknod = 133,
    SYS_uselib = 134,
    SYS_personality = 135,
    SYS_ustat = 136,
    SYS_statfs = 137,
    SYS_fstatfs = 138,
    SYS_sysfs = 139,
    SYS_getpriority = 140,
    SYS_setpriority = 141,
    SYS_sched_setparam = 142,
    SYS_sched_getparam = 143,
    SYS_sched_setscheduler = 144,
    SYS_sched_getscheduler = 145,
    SYS_sched_get_priority_max = 146,
    SYS_sched_get_priority_min = 147,
    SYS_sched_rr_get_interval = 148,
    SYS_mlock = 149,
    SYS_munlock = 150,
    SYS_mlockall = 151,
    SYS_munlockall = 152,
    SYS_vhangup = 153,
    SYS_modify_ldt = 154,
    SYS_pivot_root = 155,
    SYS__sysctl = 156,
    SYS_prctl = 157,
    SYS_arch_prctl = 158,
    SYS_adjtimex = 159,
    SYS_setrlimit = 160,
    SYS_chroot = 161,
    SYS_sync = 162,
    SYS_acct = 163,
    SYS_settimeofday = 164,
    SYS_mount = 165,
    SYS_umount2 = 166,
    SYS_swapon = 167,
    SYS_swapoff = 168,
    SYS_reboot = 169,
    SYS_sethostname = 170,
    SYS_setdomainname = 171,
    SYS_iopl = 172,
    SYS_ioperm = 173,
    SYS_create_module = 174,
    SYS_init_module = 175,
    SYS_delete_module = 176,
    SYS_get_kernel_syms = 177,
    SYS_query_module = 178,
    SYS_quotactl = 179,
    SYS_nfsservctl = 180,
    SYS_getpmsg = 181,
    SYS_putpmsg = 182,
    SYS_afs_syscall = 183,
    SYS_tuxcall = 184,
    SYS_security = 185,
    SYS_gettid = 186,
    SYS_readahead = 187,
    SYS_setxattr = 188,
    SYS_lsetxattr = 189,
    SYS_fsetxattr = 190,
    SYS_getxattr = 191,
    SYS_lgetxattr = 192,
    SYS_fgetxattr = 193,
    SYS_listxattr = 194,
    SYS_llistxattr = 195,
    SYS_flistxattr = 196,
    SYS_removexattr = 197,
    SYS_lremovexattr = 198,
    SYS_fremovexattr = 199,
    SYS_tkill = 200,
    SYS_time = 201,
    SYS_futex = 202,
    SYS_sched_setaffinity = 203,
    SYS_sched_getaffinity = 204,
    SYS_set_thread_area = 205,
    SYS_io_setup = 206,
    SYS_io_destroy = 207,
    SYS_io_getevents = 208,
    SYS_io_submit = 209,
    SYS_io_cancel = 210,
    SYS_get_thread_area = 211,
    SYS_lookup_dcookie = 212,
    SYS_epoll_create = 213,
    SYS_epoll_ctl_old = 214,
    SYS_epoll_wait_old = 215,
    SYS_remap_file_pages = 216,
    SYS_getdents64 = 217,
    SYS_set_tid_address = 218,
    SYS_restart_syscall = 219,
    SYS_semtimedop = 220,
    SYS_fadvise64 = 221,
    SYS_timer_create = 222,
    SYS_timer_settime = 223,
    SYS_timer_gettime = 224,
    SYS_timer_getoverrun = 225,
    SYS_timer_delete = 226,
    SYS_clock_settime = 227,
    SYS_clock_gettime = 228,
    SYS_clock_getres = 229,
    SYS_clock_nanosleep = 230,
    SYS_exit_group = 231,
    SYS_epoll_wait = 232,
    SYS_epoll_ctl = 233,
    SYS_tgkill = 234,
    SYS_utimes = 235,
    SYS_vserver = 236,
    SYS_mbind = 237,
    SYS_set_mempolicy = 238,
    SYS_get_mempolicy = 239,
    SYS_mq_open = 240,
    SYS_mq_unlink = 241,
    SYS_mq_timedsend = 242,
    SYS_mq_timedreceive = 243,
    SYS_mq_notify = 244,
    SYS_mq_getsetattr = 245,
    SYS_kexec_load = 246,
    SYS_waitid = 247,
    SYS_add_key = 248,
    SYS_request_key = 249,
    SYS_keyctl = 250,
    SYS_ioprio_set = 251,
    SYS_ioprio_get = 252,
    SYS_inotify_init = 253,
    SYS_inotify_add_watch = 254,
    SYS_inotify_rm_watch = 255,
    SYS_migrate_pages = 256,
    SYS_openat = 257,
    SYS_mkdirat = 258,
    SYS_mknodat = 259,
    SYS_fchownat = 260,
    SYS_futimesat = 261,
    SYS_newfstatat = 262,
    SYS_unlinkat = 263,
    SYS_renameat = 264,
    SYS_linkat = 265,
    SYS_symlinkat = 266,
    SYS_readlinkat = 267,
    SYS_fchmodat = 268,
    SYS_faccessat = 269,
    SYS_pselect6 = 270,
    SYS_ppoll = 271,
    SYS_unshare = 272,
    SYS_set_robust_list = 273,
    SYS_get_robust_list = 274,
    SYS_splice = 275,
    SYS_tee = 276,
    SYS_sync_file_range = 277,
    SYS_vmsplice = 278,
    SYS_move_pages = 279,
    SYS_utimensat = 280,
    SYS_epoll_pwait = 281,
    SYS_signalfd = 282,
    SYS_timerfd_create = 283,
    SYS_eventfd = 284,
    SYS_fallocate = 285,
    SYS_timerfd_settime = 286,
    SYS_timerfd_gettime = 287,
    SYS_accept4 = 288,
    SYS_signalfd4 = 289,
    SYS_eventfd2 = 290,
    SYS_epoll_create1 = 291,
    SYS_dup3 = 292,
    SYS_pipe2 = 293,
    SYS_inotify_init1 = 294,
    SYS_preadv = 295,
    SYS_pwritev = 296,
    SYS_rt_tgsigqueueinfo = 297,
    SYS_perf_event_open = 298,
    SYS_recvmmsg = 299,
    SYS_fanotify_init = 300,
    SYS_fanotify_mark = 301,
    SYS_prlimit64 = 302,
    SYS_name_to_handle_at = 303,
    SYS_open_by_handle_at = 304,
    SYS_clock_adjtime = 305,
    SYS_syncfs = 306,
    SYS_sendmmsg = 307,
    SYS_setns = 308,
    SYS_getcpu = 309,
    SYS_process_vm_readv = 310,
    SYS_process_vm_writev = 311,
    SYS_kcmp = 312,
    SYS_finit_module = 313,
    SYS_sched_setattr = 314,
    SYS_sched_getattr = 315,
    SYS_renameat2 = 316,
    SYS_seccomp = 317,
    SYS_getrandom = 318,
    SYS_memfd_create = 319,
    SYS_kexec_file_load = 320,
    SYS_bpf = 321,
    SYS_execveat = 322,
    SYS_userfaultfd = 323,
    SYS_membarrier = 324,
    SYS_mlock2 = 325,
    SYS_copy_file_range = 326,
    SYS_preadv2 = 327,
    SYS_pwritev2 = 328,
    SYS_pkey_mprotect = 329,
    SYS_pkey_alloc = 330,
    SYS_pkey_free = 331,
    SYS_statx = 332,
    SYS_io_pgetevents = 333,
    SYS_rseq = 334,
    SYS_pidfd_send_signal = 424,
    SYS_io_uring_setup = 425,
    SYS_io_uring_enter = 426,
    SYS_io_uring_register = 427,
    SYS_open_tree = 428,
    SYS_move_mount = 429,
    SYS_fsopen = 430,
    SYS_fsconfig = 431,
    SYS_fsmount = 432,
    SYS_fspick = 433,
    SYS_pidfd_open = 434,
    SYS_clone3 = 435,
}
static SYSCALL_NAMES: [Option<&str>; 436] = [
    Some("read"),
    Some("write"),
    Some("open"),
    Some("close"),
    Some("stat"),
    Some("fstat"),
    Some("lstat"),
    Some("poll"),
    Some("lseek"),
    Some("mmap"),
    Some("mprotect"),
    Some("munmap"),
    Some("brk"),
    Some("rt_sigaction"),
    Some("rt_sigprocmask"),
    Some("rt_sigreturn"),
    Some("ioctl"),
    Some("pread64"),
    Some("pwrite64"),
    Some("readv"),
    Some("writev"),
    Some("access"),
    Some("pipe"),
    Some("select"),
    Some("sched_yield"),
    Some("mremap"),
    Some("msync"),
    Some("mincore"),
    Some("madvise"),
    Some("shmget"),
    Some("shmat"),
    Some("shmctl"),
    Some("dup"),
    Some("dup2"),
    Some("pause"),
    Some("nanosleep"),
    Some("getitimer"),
    Some("alarm"),
    Some("setitimer"),
    Some("getpid"),
    Some("sendfile"),
    Some("socket"),
    Some("connect"),
    Some("accept"),
    Some("sendto"),
    Some("recvfrom"),
    Some("sendmsg"),
    Some("recvmsg"),
    Some("shutdown"),
    Some("bind"),
    Some("listen"),
    Some("getsockname"),
    Some("getpeername"),
    Some("socketpair"),
    Some("setsockopt"),
    Some("getsockopt"),
    Some("clone"),
    Some("fork"),
    Some("vfork"),
    Some("execve"),
    Some("exit"),
    Some("wait4"),
    Some("kill"),
    Some("uname"),
    Some("semget"),
    Some("semop"),
    Some("semctl"),
    Some("shmdt"),
    Some("msgget"),
    Some("msgsnd"),
    Some("msgrcv"),
    Some("msgctl"),
    Some("fcntl"),
    Some("flock"),
    Some("fsync"),
    Some("fdatasync"),
    Some("truncate"),
    Some("ftruncate"),
    Some("getdents"),
    Some("getcwd"),
    Some("chdir"),
    Some("fchdir"),
    Some("rename"),
    Some("mkdir"),
    Some("rmdir"),
    Some("creat"),
    Some("link"),
    Some("unlink"),
    Some("symlink"),
    Some("readlink"),
    Some("chmod"),
    Some("fchmod"),
    Some("chown"),
    Some("fchown"),
    Some("lchown"),
    Some("umask"),
    Some("gettimeofday"),
    Some("getrlimit"),
    Some("getrusage"),
    Some("sysinfo"),
    Some("times"),
    Some("ptrace"),
    Some("getuid"),
    Some("syslog"),
    Some("getgid"),
    Some("setuid"),
    Some("setgid"),
    Some("geteuid"),
    Some("getegid"),
    Some("setpgid"),
    Some("getppid"),
    Some("getpgrp"),
    Some("setsid"),
    Some("setreuid"),
    Some("setregid"),
    Some("getgroups"),
    Some("setgroups"),
    Some("setresuid"),
    Some("getresuid"),
    Some("setresgid"),
    Some("getresgid"),
    Some("getpgid"),
    Some("setfsuid"),
    Some("setfsgid"),
    Some("getsid"),
    Some("capget"),
    Some("capset"),
    Some("rt_sigpending"),
    Some("rt_sigtimedwait"),
    Some("rt_sigqueueinfo"),
    Some("rt_sigsuspend"),
    Some("sigaltstack"),
    Some("utime"),
    Some("mknod"),
    Some("uselib"),
    Some("personality"),
    Some("ustat"),
    Some("statfs"),
    Some("fstatfs"),
    Some("sysfs"),
    Some("getpriority"),
    Some("setpriority"),
    Some("sched_setparam"),
    Some("sched_getparam"),
    Some("sched_setscheduler"),
    Some("sched_getscheduler"),
    Some("sched_get_priority_max"),
    Some("sched_get_priority_min"),
    Some("sched_rr_get_interval"),
    Some("mlock"),
    Some("munlock"),
    Some("mlockall"),
    Some("munlockall"),
    Some("vhangup"),
    Some("modify_ldt"),
    Some("pivot_root"),
    Some("_sysctl"),
    Some("prctl"),
    Some("arch_prctl"),
    Some("adjtimex"),
    Some("setrlimit"),
    Some("chroot"),
    Some("sync"),
    Some("acct"),
    Some("settimeofday"),
    Some("mount"),
    Some("umount2"),
    Some("swapon"),
    Some("swapoff"),
    Some("reboot"),
    Some("sethostname"),
    Some("setdomainname"),
    Some("iopl"),
    Some("ioperm"),
    Some("create_module"),
    Some("init_module"),
    Some("delete_module"),
    Some("get_kernel_syms"),
    Some("query_module"),
    Some("quotactl"),
    Some("nfsservctl"),
    Some("getpmsg"),
    Some("putpmsg"),
    Some("afs_syscall"),
    Some("tuxcall"),
    Some("security"),
    Some("gettid"),
    Some("readahead"),
    Some("setxattr"),
    Some("lsetxattr"),
    Some("fsetxattr"),
    Some("getxattr"),
    Some("lgetxattr"),
    Some("fgetxattr"),
    Some("listxattr"),
    Some("llistxattr"),
    Some("flistxattr"),
    Some("removexattr"),
    Some("lremovexattr"),
    Some("fremovexattr"),
    Some("tkill"),
    Some("time"),
    Some("futex"),
    Some("sched_setaffinity"),
    Some("sched_getaffinity"),
    Some("set_thread_area"),
    Some("io_setup"),
    Some("io_destroy"),
    Some("io_getevents"),
    Some("io_submit"),
    Some("io_cancel"),
    Some("get_thread_area"),
    Some("lookup_dcookie"),
    Some("epoll_create"),
    Some("epoll_ctl_old"),
    Some("epoll_wait_old"),
    Some("remap_file_pages"),
    Some("getdents64"),
    Some("set_tid_address"),
    Some("restart_syscall"),
    Some("semtimedop"),
    Some("fadvise64"),
    Some("timer_create"),
    Some("timer_settime"),
    Some("timer_gettime"),
    Some("timer_getoverrun"),
    Some("timer_delete"),
    Some("clock_settime"),
    Some("clock_gettime"),
    Some("clock_getres"),
    Some("clock_nanosleep"),
    Some("exit_group"),
    Some("epoll_wait"),
    Some("epoll_ctl"),
    Some("tgkill"),
    Some("utimes"),
    Some("vserver"),
    Some("mbind"),
    Some("set_mempolicy"),
    Some("get_mempolicy"),
    Some("mq_open"),
    Some("mq_unlink"),
    Some("mq_timedsend"),
    Some("mq_timedreceive"),
    Some("mq_notify"),
    Some("mq_getsetattr"),
    Some("kexec_load"),
    Some("waitid"),
    Some("add_key"),
    Some("request_key"),
    Some("keyctl"),
    Some("ioprio_set"),
    Some("ioprio_get"),
    Some("inotify_init"),
    Some("inotify_add_watch"),
    Some("inotify_rm_watch"),
    Some("migrate_pages"),
    Some("openat"),
    Some("mkdirat"),
    Some("mknodat"),
    Some("fchownat"),
    Some("futimesat"),
    Some("newfstatat"),
    Some("unlinkat"),
    Some("renameat"),
    Some("linkat"),
    Some("symlinkat"),
    Some("readlinkat"),
    Some("fchmodat"),
    Some("faccessat"),
    Some("pselect6"),
    Some("ppoll"),
    Some("unshare"),
    Some("set_robust_list"),
    Some("get_robust_list"),
    Some("splice"),
    Some("tee"),
    Some("sync_file_range"),
    Some("vmsplice"),
    Some("move_pages"),
    Some("utimensat"),
    Some("epoll_pwait"),
    Some("signalfd"),
    Some("timerfd_create"),
    Some("eventfd"),
    Some("fallocate"),
    Some("timerfd_settime"),
    Some("timerfd_gettime"),
    Some("accept4"),
    Some("signalfd4"),
    Some("eventfd2"),
    Some("epoll_create1"),
    Some("dup3"),
    Some("pipe2"),
    Some("inotify_init1"),
    Some("preadv"),
    Some("pwritev"),
    Some("rt_tgsigqueueinfo"),
    Some("perf_event_open"),
    Some("recvmmsg"),
    Some("fanotify_init"),
    Some("fanotify_mark"),
    Some("prlimit64"),
    Some("name_to_handle_at"),
    Some("open_by_handle_at"),
    Some("clock_adjtime"),
    Some("syncfs"),
    Some("sendmmsg"),
    Some("setns"),
    Some("getcpu"),
    Some("process_vm_readv"),
    Some("process_vm_writev"),
    Some("kcmp"),
    Some("finit_module"),
    Some("sched_setattr"),
    Some("sched_getattr"),
    Some("renameat2"),
    Some("seccomp"),
    Some("getrandom"),
    Some("memfd_create"),
    Some("kexec_file_load"),
    Some("bpf"),
    Some("execveat"),
    Some("userfaultfd"),
    Some("membarrier"),
    Some("mlock2"),
    Some("copy_file_range"),
    Some("preadv2"),
    Some("pwritev2"),
    Some("pkey_mprotect"),
    Some("pkey_alloc"),
    Some("pkey_free"),
    Some("statx"),
    Some("io_pgetevents"),
    Some("rseq"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("pidfd_send_signal"),
    Some("io_uring_setup"),
    Some("io_uring_enter"),
    Some("io_uring_register"),
    Some("open_tree"),
    Some("move_mount"),
    Some("fsopen"),
    Some("fsconfig"),
    Some("fsmount"),
    Some("fspick"),
    Some("pidfd_open"),
    Some("clone3"),
];

impl SyscallNo {
    /// Returns the name of the syscall.
    #[inline]
    pub fn name(&self) -> &'static str {
        SYSCALL_NAMES[*self as usize].unwrap()
    }

    /// Constructs a `SyscallNo` from an ID. Returns `None` if the number falls
    /// outside the bounds of possible enum values.
    pub fn new(id: usize) -> Option<Self> {
        SYSCALL_IDS.get(id).and_then(|x| *x)
    }
}

impl fmt::Display for SyscallNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Debug for SyscallNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

static SYSCALL_IDS: [Option<SyscallNo>; 436] = [
    Some(SYS_read),
    Some(SYS_write),
    Some(SYS_open),
    Some(SYS_close),
    Some(SYS_stat),
    Some(SYS_fstat),
    Some(SYS_lstat),
    Some(SYS_poll),
    Some(SYS_lseek),
    Some(SYS_mmap),
    Some(SYS_mprotect),
    Some(SYS_munmap),
    Some(SYS_brk),
    Some(SYS_rt_sigaction),
    Some(SYS_rt_sigprocmask),
    Some(SYS_rt_sigreturn),
    Some(SYS_ioctl),
    Some(SYS_pread64),
    Some(SYS_pwrite64),
    Some(SYS_readv),
    Some(SYS_writev),
    Some(SYS_access),
    Some(SYS_pipe),
    Some(SYS_select),
    Some(SYS_sched_yield),
    Some(SYS_mremap),
    Some(SYS_msync),
    Some(SYS_mincore),
    Some(SYS_madvise),
    Some(SYS_shmget),
    Some(SYS_shmat),
    Some(SYS_shmctl),
    Some(SYS_dup),
    Some(SYS_dup2),
    Some(SYS_pause),
    Some(SYS_nanosleep),
    Some(SYS_getitimer),
    Some(SYS_alarm),
    Some(SYS_setitimer),
    Some(SYS_getpid),
    Some(SYS_sendfile),
    Some(SYS_socket),
    Some(SYS_connect),
    Some(SYS_accept),
    Some(SYS_sendto),
    Some(SYS_recvfrom),
    Some(SYS_sendmsg),
    Some(SYS_recvmsg),
    Some(SYS_shutdown),
    Some(SYS_bind),
    Some(SYS_listen),
    Some(SYS_getsockname),
    Some(SYS_getpeername),
    Some(SYS_socketpair),
    Some(SYS_setsockopt),
    Some(SYS_getsockopt),
    Some(SYS_clone),
    Some(SYS_fork),
    Some(SYS_vfork),
    Some(SYS_execve),
    Some(SYS_exit),
    Some(SYS_wait4),
    Some(SYS_kill),
    Some(SYS_uname),
    Some(SYS_semget),
    Some(SYS_semop),
    Some(SYS_semctl),
    Some(SYS_shmdt),
    Some(SYS_msgget),
    Some(SYS_msgsnd),
    Some(SYS_msgrcv),
    Some(SYS_msgctl),
    Some(SYS_fcntl),
    Some(SYS_flock),
    Some(SYS_fsync),
    Some(SYS_fdatasync),
    Some(SYS_truncate),
    Some(SYS_ftruncate),
    Some(SYS_getdents),
    Some(SYS_getcwd),
    Some(SYS_chdir),
    Some(SYS_fchdir),
    Some(SYS_rename),
    Some(SYS_mkdir),
    Some(SYS_rmdir),
    Some(SYS_creat),
    Some(SYS_link),
    Some(SYS_unlink),
    Some(SYS_symlink),
    Some(SYS_readlink),
    Some(SYS_chmod),
    Some(SYS_fchmod),
    Some(SYS_chown),
    Some(SYS_fchown),
    Some(SYS_lchown),
    Some(SYS_umask),
    Some(SYS_gettimeofday),
    Some(SYS_getrlimit),
    Some(SYS_getrusage),
    Some(SYS_sysinfo),
    Some(SYS_times),
    Some(SYS_ptrace),
    Some(SYS_getuid),
    Some(SYS_syslog),
    Some(SYS_getgid),
    Some(SYS_setuid),
    Some(SYS_setgid),
    Some(SYS_geteuid),
    Some(SYS_getegid),
    Some(SYS_setpgid),
    Some(SYS_getppid),
    Some(SYS_getpgrp),
    Some(SYS_setsid),
    Some(SYS_setreuid),
    Some(SYS_setregid),
    Some(SYS_getgroups),
    Some(SYS_setgroups),
    Some(SYS_setresuid),
    Some(SYS_getresuid),
    Some(SYS_setresgid),
    Some(SYS_getresgid),
    Some(SYS_getpgid),
    Some(SYS_setfsuid),
    Some(SYS_setfsgid),
    Some(SYS_getsid),
    Some(SYS_capget),
    Some(SYS_capset),
    Some(SYS_rt_sigpending),
    Some(SYS_rt_sigtimedwait),
    Some(SYS_rt_sigqueueinfo),
    Some(SYS_rt_sigsuspend),
    Some(SYS_sigaltstack),
    Some(SYS_utime),
    Some(SYS_mknod),
    Some(SYS_uselib),
    Some(SYS_personality),
    Some(SYS_ustat),
    Some(SYS_statfs),
    Some(SYS_fstatfs),
    Some(SYS_sysfs),
    Some(SYS_getpriority),
    Some(SYS_setpriority),
    Some(SYS_sched_setparam),
    Some(SYS_sched_getparam),
    Some(SYS_sched_setscheduler),
    Some(SYS_sched_getscheduler),
    Some(SYS_sched_get_priority_max),
    Some(SYS_sched_get_priority_min),
    Some(SYS_sched_rr_get_interval),
    Some(SYS_mlock),
    Some(SYS_munlock),
    Some(SYS_mlockall),
    Some(SYS_munlockall),
    Some(SYS_vhangup),
    Some(SYS_modify_ldt),
    Some(SYS_pivot_root),
    Some(SYS__sysctl),
    Some(SYS_prctl),
    Some(SYS_arch_prctl),
    Some(SYS_adjtimex),
    Some(SYS_setrlimit),
    Some(SYS_chroot),
    Some(SYS_sync),
    Some(SYS_acct),
    Some(SYS_settimeofday),
    Some(SYS_mount),
    Some(SYS_umount2),
    Some(SYS_swapon),
    Some(SYS_swapoff),
    Some(SYS_reboot),
    Some(SYS_sethostname),
    Some(SYS_setdomainname),
    Some(SYS_iopl),
    Some(SYS_ioperm),
    Some(SYS_create_module),
    Some(SYS_init_module),
    Some(SYS_delete_module),
    Some(SYS_get_kernel_syms),
    Some(SYS_query_module),
    Some(SYS_quotactl),
    Some(SYS_nfsservctl),
    Some(SYS_getpmsg),
    Some(SYS_putpmsg),
    Some(SYS_afs_syscall),
    Some(SYS_tuxcall),
    Some(SYS_security),
    Some(SYS_gettid),
    Some(SYS_readahead),
    Some(SYS_setxattr),
    Some(SYS_lsetxattr),
    Some(SYS_fsetxattr),
    Some(SYS_getxattr),
    Some(SYS_lgetxattr),
    Some(SYS_fgetxattr),
    Some(SYS_listxattr),
    Some(SYS_llistxattr),
    Some(SYS_flistxattr),
    Some(SYS_removexattr),
    Some(SYS_lremovexattr),
    Some(SYS_fremovexattr),
    Some(SYS_tkill),
    Some(SYS_time),
    Some(SYS_futex),
    Some(SYS_sched_setaffinity),
    Some(SYS_sched_getaffinity),
    Some(SYS_set_thread_area),
    Some(SYS_io_setup),
    Some(SYS_io_destroy),
    Some(SYS_io_getevents),
    Some(SYS_io_submit),
    Some(SYS_io_cancel),
    Some(SYS_get_thread_area),
    Some(SYS_lookup_dcookie),
    Some(SYS_epoll_create),
    Some(SYS_epoll_ctl_old),
    Some(SYS_epoll_wait_old),
    Some(SYS_remap_file_pages),
    Some(SYS_getdents64),
    Some(SYS_set_tid_address),
    Some(SYS_restart_syscall),
    Some(SYS_semtimedop),
    Some(SYS_fadvise64),
    Some(SYS_timer_create),
    Some(SYS_timer_settime),
    Some(SYS_timer_gettime),
    Some(SYS_timer_getoverrun),
    Some(SYS_timer_delete),
    Some(SYS_clock_settime),
    Some(SYS_clock_gettime),
    Some(SYS_clock_getres),
    Some(SYS_clock_nanosleep),
    Some(SYS_exit_group),
    Some(SYS_epoll_wait),
    Some(SYS_epoll_ctl),
    Some(SYS_tgkill),
    Some(SYS_utimes),
    Some(SYS_vserver),
    Some(SYS_mbind),
    Some(SYS_set_mempolicy),
    Some(SYS_get_mempolicy),
    Some(SYS_mq_open),
    Some(SYS_mq_unlink),
    Some(SYS_mq_timedsend),
    Some(SYS_mq_timedreceive),
    Some(SYS_mq_notify),
    Some(SYS_mq_getsetattr),
    Some(SYS_kexec_load),
    Some(SYS_waitid),
    Some(SYS_add_key),
    Some(SYS_request_key),
    Some(SYS_keyctl),
    Some(SYS_ioprio_set),
    Some(SYS_ioprio_get),
    Some(SYS_inotify_init),
    Some(SYS_inotify_add_watch),
    Some(SYS_inotify_rm_watch),
    Some(SYS_migrate_pages),
    Some(SYS_openat),
    Some(SYS_mkdirat),
    Some(SYS_mknodat),
    Some(SYS_fchownat),
    Some(SYS_futimesat),
    Some(SYS_newfstatat),
    Some(SYS_unlinkat),
    Some(SYS_renameat),
    Some(SYS_linkat),
    Some(SYS_symlinkat),
    Some(SYS_readlinkat),
    Some(SYS_fchmodat),
    Some(SYS_faccessat),
    Some(SYS_pselect6),
    Some(SYS_ppoll),
    Some(SYS_unshare),
    Some(SYS_set_robust_list),
    Some(SYS_get_robust_list),
    Some(SYS_splice),
    Some(SYS_tee),
    Some(SYS_sync_file_range),
    Some(SYS_vmsplice),
    Some(SYS_move_pages),
    Some(SYS_utimensat),
    Some(SYS_epoll_pwait),
    Some(SYS_signalfd),
    Some(SYS_timerfd_create),
    Some(SYS_eventfd),
    Some(SYS_fallocate),
    Some(SYS_timerfd_settime),
    Some(SYS_timerfd_gettime),
    Some(SYS_accept4),
    Some(SYS_signalfd4),
    Some(SYS_eventfd2),
    Some(SYS_epoll_create1),
    Some(SYS_dup3),
    Some(SYS_pipe2),
    Some(SYS_inotify_init1),
    Some(SYS_preadv),
    Some(SYS_pwritev),
    Some(SYS_rt_tgsigqueueinfo),
    Some(SYS_perf_event_open),
    Some(SYS_recvmmsg),
    Some(SYS_fanotify_init),
    Some(SYS_fanotify_mark),
    Some(SYS_prlimit64),
    Some(SYS_name_to_handle_at),
    Some(SYS_open_by_handle_at),
    Some(SYS_clock_adjtime),
    Some(SYS_syncfs),
    Some(SYS_sendmmsg),
    Some(SYS_setns),
    Some(SYS_getcpu),
    Some(SYS_process_vm_readv),
    Some(SYS_process_vm_writev),
    Some(SYS_kcmp),
    Some(SYS_finit_module),
    Some(SYS_sched_setattr),
    Some(SYS_sched_getattr),
    Some(SYS_renameat2),
    Some(SYS_seccomp),
    Some(SYS_getrandom),
    Some(SYS_memfd_create),
    Some(SYS_kexec_file_load),
    Some(SYS_bpf),
    Some(SYS_execveat),
    Some(SYS_userfaultfd),
    Some(SYS_membarrier),
    Some(SYS_mlock2),
    Some(SYS_copy_file_range),
    Some(SYS_preadv2),
    Some(SYS_pwritev2),
    Some(SYS_pkey_mprotect),
    Some(SYS_pkey_alloc),
    Some(SYS_pkey_free),
    Some(SYS_statx),
    Some(SYS_io_pgetevents),
    Some(SYS_rseq),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SYS_pidfd_send_signal),
    Some(SYS_io_uring_setup),
    Some(SYS_io_uring_enter),
    Some(SYS_io_uring_register),
    Some(SYS_open_tree),
    Some(SYS_move_mount),
    Some(SYS_fsopen),
    Some(SYS_fsconfig),
    Some(SYS_fsmount),
    Some(SYS_fspick),
    Some(SYS_pidfd_open),
    Some(SYS_clone3),
];
impl From<i32> for SyscallNo {
    fn from(id: i32) -> Self {
        Self::new(id as usize)
            .unwrap_or_else(|| panic!("invalid syscall: {}", id))
    }
}
