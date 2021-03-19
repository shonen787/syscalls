//! Syscalls for the arm architecture.

// This file is automatically generated. Do not edit!

syscall_enum! {
    pub enum Sysno {
        restart_syscall = 0,
        exit = 1,
        fork = 2,
        read = 3,
        write = 4,
        open = 5,
        close = 6,
        creat = 8,
        link = 9,
        unlink = 10,
        execve = 11,
        chdir = 12,
        mknod = 14,
        chmod = 15,
        lchown = 16,
        lseek = 19,
        getpid = 20,
        mount = 21,
        setuid = 23,
        getuid = 24,
        ptrace = 26,
        pause = 29,
        access = 33,
        nice = 34,
        sync = 36,
        kill = 37,
        rename = 38,
        mkdir = 39,
        rmdir = 40,
        dup = 41,
        pipe = 42,
        times = 43,
        brk = 45,
        setgid = 46,
        getgid = 47,
        geteuid = 49,
        getegid = 50,
        acct = 51,
        umount2 = 52,
        ioctl = 54,
        fcntl = 55,
        setpgid = 57,
        umask = 60,
        chroot = 61,
        ustat = 62,
        dup2 = 63,
        getppid = 64,
        getpgrp = 65,
        setsid = 66,
        sigaction = 67,
        setreuid = 70,
        setregid = 71,
        sigsuspend = 72,
        sigpending = 73,
        sethostname = 74,
        setrlimit = 75,
        getrusage = 77,
        gettimeofday = 78,
        settimeofday = 79,
        getgroups = 80,
        setgroups = 81,
        symlink = 83,
        readlink = 85,
        uselib = 86,
        swapon = 87,
        reboot = 88,
        munmap = 91,
        truncate = 92,
        ftruncate = 93,
        fchmod = 94,
        fchown = 95,
        getpriority = 96,
        setpriority = 97,
        statfs = 99,
        fstatfs = 100,
        syslog = 103,
        setitimer = 104,
        getitimer = 105,
        stat = 106,
        lstat = 107,
        fstat = 108,
        vhangup = 111,
        wait4 = 114,
        swapoff = 115,
        sysinfo = 116,
        fsync = 118,
        sigreturn = 119,
        clone = 120,
        setdomainname = 121,
        uname = 122,
        adjtimex = 124,
        mprotect = 125,
        sigprocmask = 126,
        init_module = 128,
        delete_module = 129,
        quotactl = 131,
        getpgid = 132,
        fchdir = 133,
        bdflush = 134,
        sysfs = 135,
        personality = 136,
        setfsuid = 138,
        setfsgid = 139,
        _llseek = 140,
        getdents = 141,
        _newselect = 142,
        flock = 143,
        msync = 144,
        readv = 145,
        writev = 146,
        getsid = 147,
        fdatasync = 148,
        _sysctl = 149,
        mlock = 150,
        munlock = 151,
        mlockall = 152,
        munlockall = 153,
        sched_setparam = 154,
        sched_getparam = 155,
        sched_setscheduler = 156,
        sched_getscheduler = 157,
        sched_yield = 158,
        sched_get_priority_max = 159,
        sched_get_priority_min = 160,
        sched_rr_get_interval = 161,
        nanosleep = 162,
        mremap = 163,
        setresuid = 164,
        getresuid = 165,
        poll = 168,
        /// NOTE: `nfsservctl` is not implemented in the kernel.
        nfsservctl = 169,
        setresgid = 170,
        getresgid = 171,
        prctl = 172,
        rt_sigreturn = 173,
        rt_sigaction = 174,
        rt_sigprocmask = 175,
        rt_sigpending = 176,
        rt_sigtimedwait = 177,
        rt_sigqueueinfo = 178,
        rt_sigsuspend = 179,
        pread64 = 180,
        pwrite64 = 181,
        chown = 182,
        getcwd = 183,
        capget = 184,
        capset = 185,
        sigaltstack = 186,
        sendfile = 187,
        vfork = 190,
        ugetrlimit = 191,
        mmap2 = 192,
        truncate64 = 193,
        ftruncate64 = 194,
        stat64 = 195,
        lstat64 = 196,
        fstat64 = 197,
        lchown32 = 198,
        getuid32 = 199,
        getgid32 = 200,
        geteuid32 = 201,
        getegid32 = 202,
        setreuid32 = 203,
        setregid32 = 204,
        getgroups32 = 205,
        setgroups32 = 206,
        fchown32 = 207,
        setresuid32 = 208,
        getresuid32 = 209,
        setresgid32 = 210,
        getresgid32 = 211,
        chown32 = 212,
        setuid32 = 213,
        setgid32 = 214,
        setfsuid32 = 215,
        setfsgid32 = 216,
        getdents64 = 217,
        pivot_root = 218,
        mincore = 219,
        madvise = 220,
        fcntl64 = 221,
        gettid = 224,
        readahead = 225,
        setxattr = 226,
        lsetxattr = 227,
        fsetxattr = 228,
        getxattr = 229,
        lgetxattr = 230,
        fgetxattr = 231,
        listxattr = 232,
        llistxattr = 233,
        flistxattr = 234,
        removexattr = 235,
        lremovexattr = 236,
        fremovexattr = 237,
        tkill = 238,
        sendfile64 = 239,
        futex = 240,
        sched_setaffinity = 241,
        sched_getaffinity = 242,
        io_setup = 243,
        io_destroy = 244,
        io_getevents = 245,
        io_submit = 246,
        io_cancel = 247,
        exit_group = 248,
        lookup_dcookie = 249,
        epoll_create = 250,
        epoll_ctl = 251,
        epoll_wait = 252,
        remap_file_pages = 253,
        set_tid_address = 256,
        timer_create = 257,
        timer_settime = 258,
        timer_gettime = 259,
        timer_getoverrun = 260,
        timer_delete = 261,
        clock_settime = 262,
        clock_gettime = 263,
        clock_getres = 264,
        clock_nanosleep = 265,
        statfs64 = 266,
        fstatfs64 = 267,
        tgkill = 268,
        utimes = 269,
        arm_fadvise64_64 = 270,
        pciconfig_iobase = 271,
        pciconfig_read = 272,
        pciconfig_write = 273,
        mq_open = 274,
        mq_unlink = 275,
        mq_timedsend = 276,
        mq_timedreceive = 277,
        mq_notify = 278,
        mq_getsetattr = 279,
        waitid = 280,
        socket = 281,
        bind = 282,
        connect = 283,
        listen = 284,
        accept = 285,
        getsockname = 286,
        getpeername = 287,
        socketpair = 288,
        send = 289,
        sendto = 290,
        recv = 291,
        recvfrom = 292,
        shutdown = 293,
        setsockopt = 294,
        getsockopt = 295,
        sendmsg = 296,
        recvmsg = 297,
        semop = 298,
        semget = 299,
        semctl = 300,
        msgsnd = 301,
        msgrcv = 302,
        msgget = 303,
        msgctl = 304,
        shmat = 305,
        shmdt = 306,
        shmget = 307,
        shmctl = 308,
        add_key = 309,
        request_key = 310,
        keyctl = 311,
        semtimedop = 312,
        /// NOTE: `vserver` is not implemented in the kernel.
        vserver = 313,
        ioprio_set = 314,
        ioprio_get = 315,
        inotify_init = 316,
        inotify_add_watch = 317,
        inotify_rm_watch = 318,
        mbind = 319,
        get_mempolicy = 320,
        set_mempolicy = 321,
        openat = 322,
        mkdirat = 323,
        mknodat = 324,
        fchownat = 325,
        futimesat = 326,
        fstatat64 = 327,
        unlinkat = 328,
        renameat = 329,
        linkat = 330,
        symlinkat = 331,
        readlinkat = 332,
        fchmodat = 333,
        faccessat = 334,
        pselect6 = 335,
        ppoll = 336,
        unshare = 337,
        set_robust_list = 338,
        get_robust_list = 339,
        splice = 340,
        arm_sync_file_range = 341,
        tee = 342,
        vmsplice = 343,
        move_pages = 344,
        getcpu = 345,
        epoll_pwait = 346,
        kexec_load = 347,
        utimensat = 348,
        signalfd = 349,
        timerfd_create = 350,
        eventfd = 351,
        fallocate = 352,
        timerfd_settime = 353,
        timerfd_gettime = 354,
        signalfd4 = 355,
        eventfd2 = 356,
        epoll_create1 = 357,
        dup3 = 358,
        pipe2 = 359,
        inotify_init1 = 360,
        preadv = 361,
        pwritev = 362,
        rt_tgsigqueueinfo = 363,
        perf_event_open = 364,
        recvmmsg = 365,
        accept4 = 366,
        fanotify_init = 367,
        fanotify_mark = 368,
        prlimit64 = 369,
        name_to_handle_at = 370,
        open_by_handle_at = 371,
        clock_adjtime = 372,
        syncfs = 373,
        sendmmsg = 374,
        setns = 375,
        process_vm_readv = 376,
        process_vm_writev = 377,
        kcmp = 378,
        finit_module = 379,
        sched_setattr = 380,
        sched_getattr = 381,
        renameat2 = 382,
        seccomp = 383,
        getrandom = 384,
        memfd_create = 385,
        bpf = 386,
        execveat = 387,
        userfaultfd = 388,
        membarrier = 389,
        mlock2 = 390,
        copy_file_range = 391,
        preadv2 = 392,
        pwritev2 = 393,
        pkey_mprotect = 394,
        pkey_alloc = 395,
        pkey_free = 396,
        statx = 397,
        rseq = 398,
        io_pgetevents = 399,
        migrate_pages = 400,
        kexec_file_load = 401,
        clock_gettime64 = 403,
        clock_settime64 = 404,
        clock_adjtime64 = 405,
        clock_getres_time64 = 406,
        clock_nanosleep_time64 = 407,
        timer_gettime64 = 408,
        timer_settime64 = 409,
        timerfd_gettime64 = 410,
        timerfd_settime64 = 411,
        utimensat_time64 = 412,
        pselect6_time64 = 413,
        ppoll_time64 = 414,
        io_pgetevents_time64 = 416,
        recvmmsg_time64 = 417,
        mq_timedsend_time64 = 418,
        mq_timedreceive_time64 = 419,
        semtimedop_time64 = 420,
        rt_sigtimedwait_time64 = 421,
        futex_time64 = 422,
        sched_rr_get_interval_time64 = 423,
        pidfd_send_signal = 424,
        io_uring_setup = 425,
        io_uring_enter = 426,
        io_uring_register = 427,
        open_tree = 428,
        move_mount = 429,
        fsopen = 430,
        fsconfig = 431,
        fsmount = 432,
        fspick = 433,
        pidfd_open = 434,
        clone3 = 435,
        close_range = 436,
        openat2 = 437,
        pidfd_getfd = 438,
        faccessat2 = 439,
        process_madvise = 440,
        epoll_pwait2 = 441,
        mount_setattr = 442,
        quotactl_path = 443,
        landlock_create_ruleset = 444,
        landlock_add_rule = 445,
        landlock_restrict_self = 446,
    }
    LAST: landlock_restrict_self;
}
