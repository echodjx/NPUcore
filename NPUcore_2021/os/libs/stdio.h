#ifndef __LIBS_STDIO_H__
#define __LIBS_STDIO_H__

#include <defs.h>
#include <stdarg.h>

/* kern/libs/stdio.c */
int cprintf(const char *fmt, ...);
int dprintf(const char *fmt, ...);
int vcprintf(const char *fmt, va_list ap);
void cputchar(int c);
int cputs(const char *str);
int getchar(void);
void input_wakeup();

/* kern/libs/readline.c */
char *readline(const char *prompt);

/* libs/printfmt.c */
void printfmt(void (*putch)(int, void *, int), int fd, void *putdat, const char *fmt, ...);
void vprintfmt(void (*putch)(int, void *, int), int fd, void *putdat, const char *fmt, va_list ap);    
int snprintf(char *str, size_t size, const char *fmt, ...);
int vsnprintf(char *str, size_t size, const char *fmt, va_list ap);

#define TEST_START(x) cputs("========== START ");cputs(x);cputs(" ==========\n");
#define TEST_END(x) cputs("========== END ");cputs(x);cputs(" ==========\n");
#define printf(...) cprintf(__VA_ARGS__)

#endif /* !__LIBS_STDIO_H__ */

