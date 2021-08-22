#include <defs.h>
#include <unistd.h>
#include <stdarg.h>
#include <syscall.h>
#include <stat.h>
#include <dirent.h>


#define MAX_ARGS            5

static inline int
syscall(uint64_t num, ...) {
    va_list ap;
    va_start(ap, num);
    uint64_t a[MAX_ARGS];
    int i, ret;
    for (i = 0; i < MAX_ARGS; i ++) {
        a[i] = va_arg(ap, uint64_t);
    }
    va_end(ap);

    asm volatile (
        "ld a7, %1\n"
        "ld a0, %2\n"
        "ld a1, %3\n"
        "ld a2, %4\n"
        "ld a3, %5\n"
    	"ld a4, %6\n"
        "ecall\n"
        "sw a0, %0"
        : "=m" (ret)
        : "m" (num),
          "m" (a[0]),
          "m" (a[1]),
          "m" (a[2]),
          "m" (a[3]),
          "m" (a[4])
        : "memory"
      );
    return ret;
}

uintptr_t 
sys_brk(size_t nu){
    static uintptr_t brk_store = 0;
    syscall(SYS_brk, nu, &brk_store);
    return brk_store;
}

int sys_chdir(const char *path){
    return syscall(SYS_chdir, path);
}

int
sys_close(int64_t fd) {
    return syscall(SYS_close, fd);
}

int
sys_dup(int64_t oldfd){
    return syscall(SYS_dup, oldfd, NO_FD);
}

int
sys_dup2(int64_t fd1, int64_t fd2) {
    return syscall(SYS_dup, fd1, fd2);
}

int
sys_exec(const char *name, int64_t argc, const char **argv) {
    return syscall(SYS_exec, name, argc, argv);
}

int
sys_exit(int64_t error_code) {
    return syscall(SYS_exit, error_code);
}

int
sys_fork(void) {
    return syscall(SYS_fork);
}

int
sys_fstat(int64_t fd, struct stat *stat) {
    return syscall(SYS_fstat, fd, stat);
}

int
sys_fsync(int64_t fd) {
    return syscall(SYS_fsync, fd);
}

int
sys_getcwd(char *buffer, size_t len) {
    return syscall(SYS_getcwd, buffer, len);
}

int
sys_getdirentry(int64_t fd, struct dirent *dirent) {
    return syscall(SYS_getdirentry, fd, dirent);
}

int
sys_getpid(void) {
    return syscall(SYS_getpid);
}

int
sys_getppid(void) {
    return syscall(SYS_getppid);
}

int
sys_gettime(void) {
    return syscall(SYS_gettime);
}

int
sys_kill(int64_t pid) {
    return syscall(SYS_kill, pid);
}

int
sys_mkdir(const char* pathname, int64_t flags){
    return syscall(SYS_mkdir, pathname, flags);
}

void* sys_mmap(void *start, size_t length, uint64_t prot, uint64_t flags,int fd, off_t offset) {
    if(!start){
        static uintptr_t addr_store = 0;
        syscall(SYS_mmap, &addr_store, length, prot, flags, fd, offset);
        start = (void*)addr_store;
    }
    if(start == NULL){
        return ((void *) -1);
    }
    return start;
}

int
sys_munmap(uintptr_t addr, size_t len) {
    return syscall(SYS_munmap, addr, len);
}

int
sys_open(const char *path, uint64_t open_flags) {
    return syscall(SYS_openat, path, open_flags);
}

int
sys_openat(int fd_dir, const char* pathname, uint64_t flags){
    return syscall(SYS_openat, fd_dir, pathname, flags);
}

int
sys_pgdir(void) {
    return syscall(SYS_pgdir);
}

int
sys_pipe(int *fd_store) {
    return syscall(SYS_pipe, fd_store);
}

int
sys_putc(int64_t c) {
    return syscall(SYS_putc, c);
}

int
sys_read(int64_t fd, void *base, size_t len) {
    return syscall(SYS_read, fd, base, len);
}

int
sys_seek(int64_t fd, off_t pos, int64_t whence) {
    return syscall(SYS_seek, fd, pos, whence);
}

int
sys_sleep(int64_t time) {
    return syscall(SYS_sleep, time);
}

int sys_unlink(const char *path){
    return syscall(SYS_unlink, path);
}

int
sys_wait(int64_t pid, int64_t *store) {
    return syscall(SYS_wait, pid, store);
}

int
sys_write(int64_t fd, void *base, size_t len) {
    return syscall(SYS_write, fd, base, len);
}

int
sys_yield(void) {
    return syscall(SYS_yield);
}








void
sys_lab6_set_priority(uint64_t priority)
{
    syscall(SYS_lab6_set_priority, priority);
}

