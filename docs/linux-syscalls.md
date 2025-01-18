# Linux Syscalls: Detailed Explanation

Below is a categorized explanation of 20 commonly used Linux system calls, including their arguments and registers.

---

## **File Operations**

### **1. open**
- **Purpose**: Opens a file and returns a file descriptor.
- **Arguments**:
  - `filename` (const char*): Path to the file.
  - `flags` (int): File access mode (e.g., `O_RDONLY`, `O_WRONLY`, `O_RDWR`).
  - `mode` (mode_t): Permissions for a new file (if `O_CREAT` is used).
- **Registers**:
  - `rax`: Syscall number (2 for `open`).
  - `rdi`: Pointer to the filename.
  - `rsi`: Flags.
  - `rdx`: Mode (optional, only used with `O_CREAT`).

---

### **2. read**
- **Purpose**: Reads data from a file descriptor.
- **Arguments**:
  - `fd` (int): File descriptor.
  - `buf` (void*): Buffer to store data.
  - `count` (size_t): Number of bytes to read.
- **Registers**:
  - `rax`: Syscall number (0 for `read`).
  - `rdi`: File descriptor.
  - `rsi`: Buffer pointer.
  - `rdx`: Byte count.

---

### **3. write**
- **Purpose**: Writes data to a file descriptor.
- **Arguments**:
  - `fd` (int): File descriptor.
  - `buf` (const void*): Buffer containing data.
  - `count` (size_t): Number of bytes to write.
- **Registers**:
  - `rax`: Syscall number (1 for `write`).
  - `rdi`: File descriptor.
  - `rsi`: Buffer pointer.
  - `rdx`: Byte count.

---

### **4. close**
- **Purpose**: Closes a file descriptor.
- **Arguments**:
  - `fd` (int): File descriptor to close.
- **Registers**:
  - `rax`: Syscall number (3 for `close`).
  - `rdi`: File descriptor.

---

### **5. lseek**
- **Purpose**: Adjusts the file offset of an open file descriptor.
- **Arguments**:
  - `fd` (int): File descriptor.
  - `offset` (off_t): New offset.
  - `whence` (int): Positioning mode (`SEEK_SET`, `SEEK_CUR`, `SEEK_END`).
- **Registers**:
  - `rax`: Syscall number (8 for `lseek`).
  - `rdi`: File descriptor.
  - `rsi`: Offset.
  - `rdx`: Positioning mode.

---

## **Process Management**

### **6. fork**
- **Purpose**: Creates a new process (child inherits parent's memory space).
- **Arguments**: None.
- **Registers**:
  - `rax`: Syscall number (57 for `fork`).
- **Return Value**:
  - Parent process gets child PID.
  - Child process gets `0`.

---

### **7. execve**
- **Purpose**: Executes a new program, replacing the current process image.
- **Arguments**:
  - `filename` (const char*): Path to the executable.
  - `argv` (char* const[]): Argument list.
  - `envp` (char* const[]): Environment variables.
- **Registers**:
  - `rax`: Syscall number (59 for `execve`).
  - `rdi`: Filename pointer.
  - `rsi`: Argument list pointer.
  - `rdx`: Environment pointer.

---

### **8. exit**
- **Purpose**: Terminates the calling process.
- **Arguments**:
  - `status` (int): Exit status.
- **Registers**:
  - `rax`: Syscall number (60 for `exit`).
  - `rdi`: Exit status.

---

### **9. wait** / **waitpid**
- **Purpose**: Waits for a child process to change state.
- **Arguments**:
  - `pid` (pid_t): PID of child to wait for (`-1` to wait for any child).
  - `status` (int*): Pointer to store child's exit status.
  - `options` (int): Options (e.g., `WNOHANG`).
- **Registers**:
  - `rax`: Syscall number (61 for `wait4` or related calls).
  - `rdi`: PID.
  - `rsi`: Status pointer.
  - `rdx`: Options.

---

### **10. getpid**
- **Purpose**: Retrieves the PID of the calling process.
- **Arguments**: None.
- **Registers**:
  - `rax`: Syscall number (39 for `getpid`).

---

## **Memory Management**

### **11. mmap**
- **Purpose**: Maps files or devices into memory.
- **Arguments**:
  - `addr` (void*): Desired address (or `NULL` for auto-selection).
  - `length` (size_t): Size of mapping.
  - `prot` (int): Memory protection flags (e.g., `PROT_READ`, `PROT_WRITE`).
  - `flags` (int): Mapping flags (e.g., `MAP_PRIVATE`, `MAP_SHARED`).
  - `fd` (int): File descriptor.
  - `offset` (off_t): Offset in the file.
- **Registers**:
  - `rax`: Syscall number (9 for `mmap`).
  - `rdi`: Address.
  - `rsi`: Length.
  - `rdx`: Protection flags.
  - `r10`: Mapping flags.
  - `r8`: File descriptor.
  - `r9`: Offset.

---

### **12. munmap**
- **Purpose**: Unmaps memory regions.
- **Arguments**:
  - `addr` (void*): Address of memory region.
  - `length` (size_t): Size of region.
- **Registers**:
  - `rax`: Syscall number (11 for `munmap`).
  - `rdi`: Address.
  - `rsi`: Length.

---

### **13. brk**
- **Purpose**: Adjusts the program's data segment size.
- **Arguments**:
  - `addr` (void*): New program break (or `NULL` to query current break).
- **Registers**:
  - `rax`: Syscall number (12 for `brk`).
  - `rdi`: Address.

---

## **Inter-Process Communication (IPC)**

### **14. pipe**
- **Purpose**: Creates a pipe for inter-process communication.
- **Arguments**:
  - `pipefd` (int[2]): Array to store file descriptors for read and write ends.
- **Registers**:
  - `rax`: Syscall number (22 for `pipe`).
  - `rdi`: Array pointer.

---

### **15. socket**
- **Purpose**: Creates a network socket.
- **Arguments**:
  - `domain` (int): Protocol family (e.g., `AF_INET`).
  - `type` (int): Socket type (e.g., `SOCK_STREAM`).
  - `protocol` (int): Protocol to use (usually `0` for default).
- **Registers**:
  - `rax`: Syscall number (41 for `socket`).
  - `rdi`: Domain.
  - `rsi`: Type.
  - `rdx`: Protocol.

---

This is a partial explanation due to space constraints.
