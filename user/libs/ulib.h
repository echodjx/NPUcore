#ifndef __USER_LIBS_ULIB_H__
#define __USER_LIBS_ULIB_H__

#include <defs.h>
#include <stat.h>

void __warn(const char *file, int line, const char *fmt, ...);
void __noreturn __panic(const char *file, int line, const char *fmt, ...);

#define warn(...)                                       \
    __warn(__FILE__, __LINE__, __VA_ARGS__)

#define panic(...)                                      \
    __panic(__FILE__, __LINE__, __VA_ARGS__)

#define assert(x)                                       \
    do {                                                \
        if (!(x)) {                                     \
            panic("assertion failed: %s", #x);          \
        }                                               \
    } while (0)

// static_assert(x) will generate a compile-time error if 'x' is false.
#define static_assert(x)                                \
    switch (x) { case 0: case (x): ; }







void __noreturn exit(int error_code);
int fork(void);
int wait(void);
int waitpid(int pid, int *store, int option);
void yield(void);
int kill(int pid);
int getpid(void);
void print_pgdir(void);
unsigned int gettime_msec(void);
void lab6_set_priority(uint32_t priority);
unsigned int sleep(unsigned int time);
int fprintf(int fd, const char *fmt, ...);
int __exec(const char *name, const char **argv);
#endif /* !__USER_LIBS_ULIB_H__ */
#define STDIN 0
#define STDOUT 1
#define STDERR 2

#define PROT_NONE 0
#define PROT_READ 1
#define PROT_WRITE 2
#define PROT_EXEC 4
#define PROT_GROWSDOWN 0X01000000
#define PROT_GROWSUP 0X02000000

#define MAP_FILE 0
#define MAP_SHARED 0x01
#define MAP_PRIVATE 0X02
#define MAP_FAILED ((void *) -1)

// chap1:echo, exec
int execve(const char *pathname, char * const argv[], char * const envp[]);

// chap2:write,exit
int write(int fd, void *base, size_t len);
void __noreturn exit(int error_code);

// chap3:gettime,yield,set_priority,brk
unsigned int get_time(void);
#define sched_yield yield
void yield(void);
void lab6_set_priority(uint32_t priority);
uintptr_t brk(size_t nu);
void *mmap(void *start, size_t length, uint64_t prot, uint64_t flags,int fd, off_t offset);
int munmap(uintptr_t addr, size_t len);

// chap4:fork,getpid,wait,waitpid,dup,dup2,clone
int fork(void);
int getpid(void);
int getppid(void);
//int dup(int64_t oldfd);
//int dup2(int64_t oldfd, int64_t newfd);

// chap5:mount,pipe,getdents

//int getdents(uint64_t fd, struct linux_dirent64 *dirp,uint64_t count);
int pipe(int *fd_store);
int times(struct _times *t);
int uname(struct _uname *un);

// rzh: we must use asm to realize sys_clone, aborted.
int clone(int (*fn)(void *arg), void *arg, void *stack, size_t stack_size, uint64_t flags);

#define pid_t int
#define SIGCHLD   17

// chap6:open, read, mkdir, chdir, close, unlink, openat
//int open(const char* pathname, uint64_t flags);
int read(int fd, void *base, size_t len);
int mkdir(const char* pathname, int64_t flags);
int chdir(const char* pathname);
int close(int fd);
int unlink(const char *pathname);
int openat(int fd_dir, const char* pathname, uint64_t flags);

// chap7:fstat
int fstat(int fd, struct stat *stat);
void print_stat(const char *name, int fd, struct stat *stat);
# define uint64 uint64_t
# define uint32 uint32_t
typedef unsigned int mode_t;
//char* getcwd(char *buffer, size_t len);


