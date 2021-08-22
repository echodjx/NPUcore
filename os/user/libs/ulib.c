#include <defs.h>
#include <syscall.h>
#include <stdio.h>
#include <ulib.h>
#include <stat.h>
#include <lock.h>

void
exit(int error_code) {
    sys_exit(error_code);
    cprintf("BUG: exit failed.\n");
    while (1);
}

int
fork(void) {
    return sys_fork();
}

// int
// wait(void* arg) {
//     return sys_wait(0, arg);
// }

int
waitpid(int pid, int *store, int option) {
    return sys_wait((int64_t)pid, (int64_t)store);
}

void
yield(void) {
    sys_yield();
}

int
kill(int pid) {
    return sys_kill(pid);
}

int
getpid(void) {
    return sys_getpid();
}

//new 0510
int
getppid(void){
    return sys_getppid();
}

//print_pgdir - print the PDT&PT
void
print_pgdir(void) {
    sys_pgdir();
}

unsigned int
get_time(void) {
    return (unsigned int)sys_gettime();
}

void
lab6_set_priority(uint32_t priority)
{
    sys_lab6_set_priority(priority);
}

unsigned int
sleep(unsigned int time) {
    return sys_sleep(time);
}
int
__exec(const char *name, const char **argv) {
    int argc = 0;
    while (argv[argc] != NULL) {
        argc ++;
    }
    //cprintf("in __exec, name:%s, argc:%d\n", name, argc);
    return sys_exec(name, argc, argv);
}

int
execve(const char *pathname, char * const argv[], char * const envp[]) {
    int argc = 0;
    while (argv[argc] != NULL) {
        argc ++;
    }
    return sys_exec(pathname, argc, argv);
}

int
mkdir(const char* pathname, int64_t flags){
    return sys_mkdir(pathname, flags);
}

int
chdir(const char* pathname){
    return sys_chdir(pathname);
}

// int
// dup(int64_t oldfd){
//     return sys_dup(oldfd);
// }

int
unlink(const char* pathname){
    return sys_unlink(pathname);
}

int
openat(int fd_dir, const char* pathname, uint64_t flags){
    return sys_openat(fd_dir, pathname, flags);
}

int __clone(int (*fn)(void *), uintptr_t stack, uint64_t clone_flags, void *arg,  uint64_t pid, uint64_t cid);

int
clone(int (*fn)(void *), void *arg, void *stack, size_t stack_size, uint64_t flags){
    if (stack){
	    stack += stack_size;
    }
    return __clone(fn, stack, flags, NULL, NULL, NULL);
}

uintptr_t brk(size_t nu){
    return sys_brk(nu);
}

void *mmap(void *start, size_t length, uint64_t prot, uint64_t flags,int fd, off_t offset){
    return sys_mmap(start, length, prot, flags, fd, offset);
}

int munmap(uintptr_t addr, size_t len) {
    return sys_munmap(addr, len);
}

int getdents(uint64_t fd, struct linux_dirent64 *dirp,uint64_t count){
    int ret = sys_getdirentry(fd, dirp);
    return dirp->d_reclen;
}

int times(struct _times *t){
    return sys_times(t);
}

int uname(struct _uname *un){
    return sys_uname(un);
}