# Syscall API for Infinity oS

## The basics
The Infinity OS syscall API is very similar to linux to provide cross-os compability. I recommend learning about Linux syscalls before these.
The syscall arguments will be passed in this order of registers:
* `rax`: The function ID
* `rdi`: Argument 0
* `rsi`: Argument 1
* `rdx`: Argument 2
* `r10`: Argument 3
* `r8`: Argument 4
* `r9`: Argument 5

## Syscalls:
| Implemented | Name  | Number (`rax`) | Description                 | Arg 0<br>(`rdi`)         | Arg 1<br>(`rsi`)     | Arg 2<br>(`rdx`)  | Arg 3<br>(`r10`) | Arg 4<br>(`r8`) | Arg 5<br>(`r9`)     |
|-------------|-------|----------------|-----------------------------|--------------------------|----------------------|-------------------|------------------|-----------------|---------------------|
| :x:         | Read  | 0              | Read from a file descriptor | unsigned int fd          | char *buf            | size_t count      |                  |                 |                     |
| :x:         | Write | 1              | Write to a file descriptor  | unsigned int fd          | const char *buf      | size_t count      |                  |                 |                     |
