/* exit-witness — records the call that ends a process, with its caller.
 *
 * `scripts/agent-run.sh boot` loads this library into the app process alone
 * (LD_PRELOAD on its one spawn line) when the harness names a built copy in
 * ANDROMEDA_PULSE_EXIT_WITNESS_LIB. It appends one JSON object per line to
 * the file named by ANDROMEDA_PULSE_EXIT_WITNESS_FILE:
 *
 *   loaded        at load: pid, process name
 *   end           at exit / _exit / _Exit / quick_exit / abort: pid, thread,
 *                 process name, the call, its code, errno, the caller frames
 *                 as module basename, exported symbol and offset
 *   runtime-exit  from an exit handler: the C runtime's own exit path ran
 *
 * So `exit` through the linker leaves loaded, end, runtime-exit; a direct
 * `_exit` leaves loaded, end; `main` returning leaves loaded, runtime-exit;
 * a direct system call leaves loaded alone. A line holds no environment
 * value, no argument, no path beyond a basename and nothing of the process's
 * memory. Read by `cargo xtask harness:settled` (xtask/src/harness_witness.rs).
 *
 * Build: cc -shared -fPIC -O2 -o exit-witness.so scripts/exit-witness.c
 */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <execinfo.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>

#define FILE_VAR "ANDROMEDA_PULSE_EXIT_WITNESS_FILE"
#define LINE_MAX_BYTES 4096
/* The longest frame entry: two bounded names, a 64-bit offset, punctuation. */
#define FRAME_MAX_BYTES 200
#define MAX_FRAMES 24

static int witness_fd = -1;
static char comm[32] = "unknown";

/* Keeps a line one valid JSON object of printable ASCII whatever a name holds. */
static void copy_safe(char *dst, size_t cap, const char *src) {
  size_t i = 0;
  for (; src && src[i] && i + 1 < cap; i++) {
    unsigned char c = (unsigned char)src[i];
    dst[i] = (c < 0x20 || c > 0x7e || c == '"' || c == '\\') ? '?' : (char)c;
  }
  dst[i] = 0;
}

static void write_line(const char *line, int len) {
  ssize_t written = write(witness_fd, line, (size_t)len);
  (void)written;
}

static void record_runtime_exit(void) {
  char line[96];
  int len = snprintf(line, sizeof(line), "{\"kind\":\"runtime-exit\",\"pid\":%d}\n", (int)getpid());
  write_line(line, len);
}

__attribute__((constructor)) static void witness_init(void) {
  const char *path = getenv(FILE_VAR);
  if (!path || !*path) return;
  int fd = open(path, O_WRONLY | O_APPEND | O_CREAT | O_CLOEXEC, 0644);
  /* No process this one starts inherits the preload or the file. */
  unsetenv("LD_PRELOAD");
  unsetenv(FILE_VAR);
  if (fd < 0) return;

  char raw[32] = "";
  int c = open("/proc/self/comm", O_RDONLY | O_CLOEXEC);
  if (c >= 0) {
    ssize_t n = read(c, raw, sizeof(raw) - 1);
    close(c);
    if (n > 0) {
      raw[n] = 0;
      raw[strcspn(raw, "\n")] = 0;
      copy_safe(comm, sizeof(comm), raw);
    }
  }
  void *prime[2];
  backtrace(prime, 2); /* loads the unwinder now, not inside the ending call */

  witness_fd = fd;
  char line[128];
  int len = snprintf(line, sizeof(line), "{\"kind\":\"loaded\",\"pid\":%d,\"comm\":\"%s\"}\n",
                     (int)getpid(), comm);
  write_line(line, len);
  atexit(record_runtime_exit);
}

__attribute__((noinline)) static void record_end(const char *call, int code) {
  int saved = errno;
  if (witness_fd < 0) return;
  void *frames[MAX_FRAMES];
  int n = backtrace(frames, MAX_FRAMES);
  char line[LINE_MAX_BYTES];
  int len = snprintf(line, sizeof(line),
                     "{\"kind\":\"end\",\"pid\":%d,\"tid\":%ld,\"comm\":\"%s\",\"call\":\"%s\","
                     "\"code\":%d,\"errno\":%d,\"frames\":[",
                     (int)getpid(), (long)syscall(SYS_gettid), comm, call, code, saved);
  /* Frame 0 is this function; the interposed call and its callers follow. */
  for (int i = 1; i < n && len < LINE_MAX_BYTES - FRAME_MAX_BYTES - 4; i++) {
    Dl_info info;
    memset(&info, 0, sizeof(info));
    dladdr(frames[i], &info);
    const char *slash = info.dli_fname ? strrchr(info.dli_fname, '/') : NULL;
    char module[61], symbol[81];
    copy_safe(module, sizeof(module), slash ? slash + 1 : (info.dli_fname ? info.dli_fname : "?"));
    copy_safe(symbol, sizeof(symbol), info.dli_sname ? info.dli_sname : "");
    unsigned long offset =
        info.dli_fbase ? (unsigned long)((char *)frames[i] - (char *)info.dli_fbase) : 0;
    len += snprintf(line + len, sizeof(line) - (size_t)len,
                    "%s{\"m\":\"%s\",\"s\":\"%s\",\"o\":\"0x%lx\"}", i > 1 ? "," : "", module,
                    symbol, offset);
  }
  len += snprintf(line + len, sizeof(line) - (size_t)len, "]}\n");
  write_line(line, len);
  errno = saved;
}

void exit(int code) {
  record_end("exit", code);
  void (*real)(int) = dlsym(RTLD_NEXT, "exit");
  if (real) real(code);
  syscall(SYS_exit_group, code);
  __builtin_unreachable();
}

void _exit(int code) {
  record_end("_exit", code);
  void (*real)(int) = dlsym(RTLD_NEXT, "_exit");
  if (real) real(code);
  syscall(SYS_exit_group, code);
  __builtin_unreachable();
}

void _Exit(int code) {
  record_end("_Exit", code);
  void (*real)(int) = dlsym(RTLD_NEXT, "_Exit");
  if (real) real(code);
  syscall(SYS_exit_group, code);
  __builtin_unreachable();
}

void quick_exit(int code) {
  record_end("quick_exit", code);
  void (*real)(int) = dlsym(RTLD_NEXT, "quick_exit");
  if (real) real(code);
  syscall(SYS_exit_group, code);
  __builtin_unreachable();
}

void abort(void) {
  record_end("abort", -1);
  void (*real)(void) = dlsym(RTLD_NEXT, "abort");
  if (real) real();
  syscall(SYS_exit_group, 134);
  __builtin_unreachable();
}
