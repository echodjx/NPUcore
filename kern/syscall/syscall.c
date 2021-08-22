#include <unistd.h>
#include <proc.h>
#include <syscall.h>
#include <trap.h>
#include <stdio.h>
#include <pmm.h>
#include <assert.h>
#include <clock.h>
#include <sysfile.h>
#include<stdio.h>

static int
sys_brk(uint64_t arg[]){
    size_t nu = (size_t)arg[0];
    uintptr_t * brk_store = (uintptr_t*)arg[1];
    return do_brk(nu, brk_store);
}

static int
sys_chdir(uint64_t arg[]) {
    const char *path = (const char *)arg[0];
    return sysfile_chdir(path);
}

static int
sys_clone(uint64_t arg[]){
    struct trapframe *tf = current->tf;
    uint64_t clone_flags = (uint64_t)arg[1];
    uintptr_t stack = (uintptr_t)arg[0];
    //cprintf("[sys_clone] stack:%d\n", stack);
    if (stack == 0) {
        stack = tf->gpr.sp;
    }
    return do_fork(clone_flags, stack, tf);
}

static int
sys_close(uint64_t arg[]) {
    int fd = (int)arg[0];
    return sysfile_close(fd);
}


static int
sys_dup(uint64_t arg[]) {
    int fd1 = (int)arg[0];
    return sysfile_dup(fd1, NO_FD);
}

static int
sys_dup2(uint64_t arg[]) {
    int fd1 = (int)arg[0];
    int fd2 = (int)arg[1];
    return sysfile_dup(fd1, fd2);
}

static int
sys_exec(uint64_t arg[]) {
    //cprintf("ok3\n");
    const char *name = (const char *)(arg[0]);
    int argc = (int)arg[1];
    const char **argv = (const char **)arg[2];
    //cprintf("do_execve %s %d %d\n", name, argc, argv[2]);
    return do_execve(name, argc, argv);

}

static int
sys_exit(uint64_t arg[]) {
    int error_code = (int)arg[0];
    return do_exit(error_code);
}

static int
sys_fork(uint64_t arg[]) {
    struct trapframe *tf = current->tf;
    uintptr_t stack = tf->gpr.sp;
    return do_fork(0, stack, tf);
}

static int
sys_fstat(uint64_t arg[]) {
    int fd = (int)arg[0];
    struct stat *stat = (struct stat *)arg[1];
    return sysfile_fstat(fd, stat);
}

static int
sys_fsync(uint64_t arg[]) {
    int fd = (int)arg[0];
    return sysfile_fsync(fd);
}

static int
sys_getcwd(uint64_t arg[]) {
    char *buf = (char *)arg[0];
    size_t len = (size_t)arg[1];
    return sysfile_getcwd(buf, len);
}

static int
sys_getdirentry(uint64_t arg[]) {
    int fd = (int)arg[0];
    struct dirent *direntp = (struct dirent *)arg[1];
    return sysfile_getdirentry(fd, direntp);
}

static int
sys_getpid(uint64_t arg[]) {
    return current->pid;
}

static int
sys_getppid(uint64_t arg[]){
    return current->parent->pid;
}

static int 
sys_gettime(uint64_t arg[]){
    return (int)ticks*10;
}

static int
sys_kill(uint64_t arg[]) {
    int pid = (int)arg[0];
    return do_kill(pid);
}

static int
sys_mkdir(uint64_t arg[]) {
    const char *path = (const char *)arg[0];
    return sysfile_mkdir(path);
}

static int
sys_mkdirat(uint64_t arg[]) {
    const char *path = (const char *)arg[1];
    return sysfile_mkdir(path);
}


static int
sys_mmap(uint64_t arg[]) {
    uintptr_t *addr_store = (uintptr_t *)arg[0];
    //cprintf("[sys_mmap] addr_store:%d\n", *addr_store);
    size_t len = (size_t)arg[1];
    uint64_t mmap_flags = (uint64_t)arg[2];
    int fd = (int)arg[4];
    off_t offset = (off_t)arg[5];
    int ret = 0;
    if((ret = do_mmap(addr_store, len, mmap_flags)) != 0){
        return ret;
    }
    ret = sysfile_read(fd, (void*)(*addr_store), len);
    //cprintf("%s\n", *addr_store);
    return 0;
}

static int
sys_munmap(uint64_t arg[]) {
    uintptr_t addr = (uintptr_t)arg[0];
    size_t len = (size_t)arg[1];
    return do_munmap(addr, len);
}

static int
sys_open(uint64_t arg[]) {
    const char *path = (const char *)arg[0];
    uint32_t open_flags = (uint32_t)arg[1];
    return sysfile_open(path, open_flags);
}

static int
sys_openat(uint64_t arg[]) {
    const char *path = (const char *)arg[1];
    uint32_t open_flags = (uint32_t)arg[2]+1;
    return sysfile_open(path, open_flags)+3;
}


static int
sys_pipe(uint64_t arg[]) {
    int *fd_store = (int *)arg[0];
    return sysfile_pipe(fd_store);
}

static int
sys_putc(uint64_t arg[]) {
    int c = (int)arg[0];
    cputchar(c);
    return 0;
}

static int
sys_pgdir(uint64_t arg[]) {
    //print_pgdir();
    return 0;
}


static int
sys_read(uint64_t arg[]) {
    int fd = (int)arg[0];
    void *base = (void *)arg[1];
    size_t len = (size_t)arg[2];
        if(fd==0)
    {
        cprintf("ssss");
        int cnt = 0; 
        while(1){getchar();}
        return 1;
    }else
    {
        fd=fd-3;
        return sysfile_read(fd, base, len);
    }
}

static int
sys_seek(uint64_t arg[]) {
    int fd = (int)arg[0];
    off_t pos = (off_t)arg[1];
    int whence = (int)arg[2];
    return sysfile_seek(fd, pos, whence);
}

static int
sys_sleep(uint64_t arg[]) {
    unsigned int time = (unsigned int)arg[0];
    return do_sleep(time);
}

static int
sys_times(uint64_t arg[]){
    struct _times* buf = (struct _times*)arg[0];
    buf->tms_cstime = 0;
    buf->tms_stime = 0;
    buf->tms_utime = 0;
    buf->tms_cutime = 0;
    return 0;
}

static int 
sys_uname(uint64_t arg[]){
    struct _uname* buf = (struct _uname*)arg[0];
    snprintf(buf->sysname,20, "%s", "NWPUCore");
    snprintf(buf->version,20, "%s", "1.2");
    snprintf(buf->release,20, "%s", "No");
    snprintf(buf->machine,20, "%s", "k210");
    snprintf(buf->domainname,20, "%s", "nwpu.edu.cn");
    snprintf(buf->nodename,20, "%s", "nwpu");
    return 0;
}

static int
sys_unlink(uint64_t arg[]) {
    const char *path = (const char *)arg[0];
    return sysfile_unlink(path);
}

static int
sys_unlinkat(uint64_t arg[]) {
    const char *path = (const char *)arg[1];
    return sysfile_unlink(path);
}


static int
sys_wait(uint64_t arg[]) {
    int pid = (int)arg[0];
    int *store = (int *)arg[1];
    return do_wait(pid, store);
}

static int
sys_wait4(uint64_t arg[]) {
    int pid = (int)arg[0];
    int *store = (int *)arg[1];
    return do_wait(pid, store);
}

static int
sys_write(uint64_t arg[]) {
    int fd = (int)arg[0];
    void *base = (void *)arg[1];
    size_t len = (size_t)arg[2];
    if(fd==1)
    {
        int cnt = 0;
        char c;
        while(cnt!=len)
        {
            c=((char*)base)[cnt];
            cputchar((int)c);
            cnt++;
        }
        return cnt;
    }else
    {
        fd=fd-3;
        return sysfile_write(fd, base, len);
    }
}


static int
sys_yield(uint64_t arg[]) {
    return do_yield();
}

// static int
// sys_execs(uint64_t arg[]) {
//     const char *name = (const char *)(arg[0]);
//     int argc = (int)arg[1];
//     const char **argv = (const char **)arg[2];
//     return do_execves(name, argc, argv);

// }
static int
sys_execve(uint64_t arg[]){
    //cprintf("ok3\n");
    const char *path = (const char *)arg[0];
    int argc = (int)arg[1];
    char** argv = (char **)arg[2];
    argv[0] = path;
    
    int res = do_execve(path, argc, argv);
    return res;
}




static int sys_lab6_set_priority(uint64_t arg[]){
    uint64_t priority = (uint64_t)arg[0];
    lab6_set_priority(priority);
    return 0;
}


static int 
sys_io_setup(uint64_t arg[]) {
    //do nothing
    return 0;
}


static int (*syscalls[])(uint64_t arg[]) = {
    // [SYS_execs]              sys_execs,
    //[SYS_io_setup]          sys_io_setup,
    [SYS_brk]               sys_brk,
    [SYS_chdir]             sys_chdir,
    [SYS_clone]             sys_clone,
    [SYS_close]             sys_close,
    [SYS_dup3]               sys_dup,
    [SYS_dup]              sys_dup2,
    [SYS_exec]              sys_exec,
    [SYS_exit]              sys_exit,
    [SYS_fork]              sys_fork,
    [SYS_fstat]             sys_fstat,
    [SYS_fsync]             sys_fsync,
    [SYS_getcwd]            sys_getcwd,
    [SYS_getdents64]       sys_getdirentry,
    [SYS_getpid]            sys_getpid,
    [SYS_getppid]           sys_getppid,
    [SYS_gettime]           sys_gettime,
    [SYS_kill]              sys_kill,
    [SYS_mkdir]             sys_mkdir,
    [SYS_mkdirat]           sys_mkdirat,
    [SYS_mmap]              sys_mmap,
    [SYS_munmap]            sys_munmap,
    [SYS_open]              sys_open,
    [SYS_openat]            sys_openat,
    [SYS_pipe2]              sys_pipe,
    [SYS_putc]              sys_putc,
    [SYS_pgdir]             sys_pgdir,
    [SYS_read]              sys_read,
    [SYS_seek]              sys_seek,
    [SYS_sleep]             sys_sleep,
    [SYS_times]             sys_times,
    [SYS_uname]             sys_uname,
    [SYS_unlink]            sys_unlink,
    [SYS_unlinkat]          sys_unlinkat,
    [SYS_wait]              sys_wait,
    [SYS_wait4]             sys_wait4,
    [SYS_write]             sys_write,
    [SYS_sched_yield]             sys_yield,
    [SYS_lab6_set_priority] sys_lab6_set_priority,
};

#define NUM_SYSCALLS        ((sizeof(syscalls)) / (sizeof(syscalls[0])))

void
syscall(void) {
    struct trapframe *tf = current->tf;
    //cprintf("call syscall = %d\n", tf->gpr.a7);
    uint64_t arg[6];
    int num = tf->gpr.a7;
    if (num >= 0 && num < NUM_SYSCALLS) {
        if (syscalls[num] != NULL) {
            //cprintf("ok1\n");
            arg[0] = tf->gpr.a0;
            arg[1] = tf->gpr.a1;
            arg[2] = tf->gpr.a2;
            arg[3] = tf->gpr.a3;
            arg[4] = tf->gpr.a4;
            arg[5] = tf->gpr.a5;
            //cprintf("ok2\n");
            tf->gpr.a0 = syscalls[num](arg);
            //cprintf("ret val = %d\n", tf->gpr.a0);
            return ;
        }
        //cprintf("\nssss%d",num);
    }
    cprintf("%d",num);
    //print_trapframe(tf);
    panic("undefined syscall %d, pid = %d, name = %s.\n",
            num, current->pid, current->name);
}

