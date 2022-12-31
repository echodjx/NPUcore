#ifndef __USER_LIBS_SYSCALL_H__
#define __USER_LIBS_SYSCALL_H__
struct dirent;
struct stat;

uintptr_t sys_brk(size_t nu);
int sys_chdir(const char *path);

int sys_close(int64_t fd);
int sys_dup(int64_t oldfd);
int sys_dup2(int64_t fd1, int64_t fd2);
int sys_exec(const char *name, int64_t argc, const char **argv);
int sys_exit(int64_t error_code);
int sys_fork(void);
int sys_fstat(int64_t fd, struct stat *stat);
int sys_fsync(int64_t fd);
int sys_getcwd(char *buffer, size_t len);
int sys_getdirentry(int64_t fd, struct dirent *dirent);

int sys_getpid(void);
int sys_getppid(void);
int sys_gettime(void);

int sys_kill(int64_t pid);
int sys_mkdir(const char* pathname, int64_t flags);
void* sys_mmap(void *start, size_t length, uint64_t prot, uint64_t flags,int fd, off_t offset);

int sys_munmap(uintptr_t addr, size_t len);
int sys_open(const char *path, uint64_t open_flags);
int sys_openat(int fd_dir, const char* pathname, uint64_t flags);
int sys_pgdir(void);
int sys_pipe(int *fd_store);
int sys_putc(int64_t c);
int sys_read(int64_t fd, void *base, size_t len);
int sys_seek(int64_t fd, off_t pos, int64_t whence);
int sys_sleep(int64_t time);

int sys_unlink(const char *path);
int sys_wait(int64_t pid, int64_t *store);
int sys_write(int64_t fd, void *base, size_t len);
int sys_yield(void);

//int sys_exec(const char *name, int64_t argc, const char **argv);



void sys_lab6_set_priority(uint64_t priority); //only for lab6


#endif /* !__USER_LIBS_SYSCALL_H__ */

