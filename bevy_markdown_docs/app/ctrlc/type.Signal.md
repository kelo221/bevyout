[bevy](../../index.html)::[app](../index.html)::[ctrlc](index.html)

# Type Alias Signal 

[Source](https://docs.rs/ctrlc/3.5.2/x86_64-unknown-linux-gnu/src/ctrlc/platform/unix/mod.rs.html#58)

```rust
pub type Signal = Signal;
```

Platform specific signal type

## Aliased Type

```rust
#[repr(i32)]pub enum Signal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGBUS = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGSTKFLT = 16,
    SIGCHLD = 17,
    SIGCONT = 18,
    SIGSTOP = 19,
    SIGTSTP = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGURG = 23,
    SIGXCPU = 24,
    SIGXFSZ = 25,
    SIGVTALRM = 26,
    SIGPROF = 27,
    SIGWINCH = 28,
    SIGIO = 29,
    SIGPWR = 30,
    SIGSYS = 31,
}
```

## Variants

### SIGHUP = 1

Hangup

### SIGINT = 2

Interrupt

### SIGQUIT = 3

Quit

### SIGILL = 4

Illegal instruction (not reset when caught)

### SIGTRAP = 5

Trace trap (not reset when caught)

### SIGABRT = 6

Abort

### SIGBUS = 7

Bus error

### SIGFPE = 8

Floating point exception

### SIGKILL = 9

Kill (cannot be caught or ignored)

### SIGUSR1 = 10

User defined signal 1

### SIGSEGV = 11

Segmentation violation

### SIGUSR2 = 12

User defined signal 2

### SIGPIPE = 13

Write on a pipe with no one to read it

### SIGALRM = 14

Alarm clock

### SIGTERM = 15

Software termination signal from kill

### SIGSTKFLT = 16

Available on **(`linux_android` or Emscripten or Fuchsia) and neither MIPS nor MIPS release 6 nor MIPS-64 nor MIPS-64 release 6 nor SPARC nor SPARC-64** only.

Stack fault (obsolete)

### SIGCHLD = 17

To parent on child stop or exit

### SIGCONT = 18

Continue a stopped process

### SIGSTOP = 19

Sendable stop signal not from tty

### SIGTSTP = 20

Stop signal from tty

### SIGTTIN = 21

To readers pgrp upon background tty read

### SIGTTOU = 22

Like TTIN if (tp->t\_local&LTOSTOP)

### SIGURG = 23

Urgent condition on IO channel

### SIGXCPU = 24

Exceeded CPU time limit

### SIGXFSZ = 25

Exceeded file size limit

### SIGVTALRM = 26

Virtual time alarm

### SIGPROF = 27

Profiling time alarm

### SIGWINCH = 28

Window size changes

### SIGIO = 29

Available on **non-Haiku** only.

Input/output possible signal

### SIGPWR = 30

Available on **`linux_android` or AIX or Emscripten or Fuchsia** only.

Power failure imminent.

### SIGSYS = 31

Bad system call