# Deep Dive: Computer Architecture, Virtual Memory, and Hardware-Enforced Memory Safety

This document details how native binaries interact with physical CPU hardware, operating system virtual memory subsystems, and the mechanics behind segmentation faults (`SIGSEGV`), stack overflows, and compiler-level Undefined Behavior (UB).

---

## 1. Native Code Execution and Instruction Set Architecture (ISA)

### 1.1 What is Native Code?
Native code is machine language (binary 1s and 0s) compiled directly for a specific CPU Instruction Set Architecture (such as x86-64 or ARM64).
- Does not run inside a Virtual Machine (e.g. JVM, BEAM) or an interpreter (e.g. CPython, V8).
- Directly addresses hardware registers and memory via CPU instructions (`mov`, `add`, `push`, `pop`, `call`, `jmp`).

### 1.2 x86-64 Architecture: Key Registers & Execution State
On x86-64 systems, the CPU maintains 16 general-purpose 64-bit registers:

| Register | Name / Conventional Role in System V AMD64 ABI |
| :--- | :--- |
| `%rax` | Accumulator; holds first return value from function |
| `%rdi`, `%rsi`, `%rdx`, `%rcx`, `%r8`, `%r9` | Function arguments 1 through 6 passed into callee |
| `%rsp` | **Stack Pointer**: points to the topmost byte of current stack frame |
| `%rbp` | **Base / Frame Pointer**: points to the base of current stack frame |
| `%rip` | **Instruction Pointer**: points to next machine instruction to execute |
| `%rflags` | Condition codes (Zero Flag ZF, Sign Flag SF, Overflow Flag OF, Carry Flag CF) |

---

## 2. Virtual Memory Subsystem and the MMU

Modern operating systems (Linux, macOS, Windows) run user processes in isolated **Virtual Address Spaces**. A program never observes or manipulates physical RAM addresses directly.

### 2.1 Virtual Address Space Anatomy (x86-64 User-Space: 48-bit Canonical Addressing)

```
Virtual Address
0x0000_0000_0000_0000 +------------------------------------------+  [NO ACCESS: Traps NULL dereferences]
                      | 0 to 4KB (Null Page / Restricted Trap)   |
0x0000_0000_0040_0000 +------------------------------------------+
                      | .text: Compiled executable instructions  |  (Permissions: Read + Execute, No Write)
                      +------------------------------------------+
                      | .rodata: String literals, constants      |  (Permissions: Read Only)
                      +------------------------------------------+
                      | .data / .bss: Global and static variables|  (Permissions: Read + Write)
                      +------------------------------------------+
                      | Heap: Dynamic runtime allocations        |  (Permissions: Read + Write)
                      |       (brk / mmap, grows upwards)        |
                      |                 |                        |
                      |                 v                        |
                      |                                          |
                      |                 ^                        |
                      |                 |                        |
                      | Stack: Local frames, return addresses    |  (Permissions: Read + Write)
                      |        (Grows downwards toward lower addr)|
0x0000_7FFF_FFFF_FFFF +------------------------------------------+
                      | Kernel Space (Ring 0 / Supervisor only)  |
0xFFFF_FFFF_FFFF_FFFF +------------------------------------------+
```

### 2.2 The Memory Management Unit (MMU) & Page Tables
1. **Paging**: Physical RAM and virtual memory are divided into fixed-size chunks called **Pages** (typically 4 KB = 4,096 bytes).
2. **Page Tables**: Multi-level hierarchical trees (PML4 -> PDPT -> Page Directory -> Page Table) maintained in memory by the OS kernel.
3. **PTE (Page Table Entry)**: Contains the physical base address and bit flags:
   - `P` (Present Bit): 1 if page is resident in physical RAM, 0 if swapped to disk or unmapped.
   - `R/W` (Read/Write Bit): 0 for Read-Only, 1 for Read-Write.
   - `U/S` (User/Supervisor Bit): 0 for Kernel mode (Ring 0), 1 for User mode (Ring 3).
   - `NX` / `XD` (No-Execute / Execute-Disable Bit): Prevents execution of data segments (Stack/Heap) to block buffer overflow exploits (W^X security model).
4. **Translation Lookaside Buffer (TLB)**: An ultra-fast, on-chip associative cache inside the CPU storing recent Virtual-to-Physical translations.

```
Virtual Address ---> [ TLB Cache ]
                          |
             (Miss)       v
                     [ MMU Page Table Walk (CR3 register) ]
                          |
                          v
                     Physical Address in DRAM
```

---

## 3. Hardware Faults, Segmentation Faults, and Signals

### 3.1 What is a Segmentation Fault (`SIGSEGV`)?
A segmentation fault occurs when software attempts an invalid memory operation that violates MMU hardware protections.

### 3.2 The Hardware Exception Lifecycle (Step-by-Step)

```
[ Step 1: Illegal Memory Operation ]
User instruction: `mov (%rax), %rbx` where `%rax = 0x00000000` (NULL) or unmapped address.

[ Step 2: MMU Translation Failure ]
MMU traverses page tables:
- Present bit is 0 (Page not mapped), OR
- Write attempted on page where R/W bit is 0 (.text / .rodata), OR
- User mode (Ring 3) instruction accesses page with U/S = 0 (Kernel).

[ Step 3: Hardware Exception Triggered ]
CPU halts execution of current instruction and generates a hardware interrupt:
- Page Fault (#PF, Vector 14) or General Protection Fault (#GP, Vector 13).
- Saves faulting address in `%cr2` register and error code onto kernel stack.

[ Step 4: Ring 0 Kernel Mode Transition ]
CPU switches to Ring 0 and jumps to the kernel's Interrupt Descriptor Table (IDT).
Linux kernel handler `do_page_fault()` is invoked.

[ Step 5: Kernel Virtual Memory Area (VMA) Inspection ]
Kernel inspects the process's `mm_struct` and `vm_area_struct` list:
- Case A: Demand paging or copy-on-write (COW) -> Kernel allocates physical frame, updates PTE, and resumes instruction transparently.
- Case B: Address does not belong to any valid VMA or violates permissions -> Kernel generates `SIGSEGV` (Signal 11).

[ Step 6: Signal Dispatch & Process Termination ]
Kernel delivers `SIGSEGV` to the offending thread.
- If no custom `sigaction` handler is installed, default action executes:
  1. Abruptly terminates process.
  2. Dumps memory image to disk (`core.<pid>` core dump) for post-mortem debugging with GDB/LLDB.
```

---

## 4. Stack Overflows & Guard Page Protection

### 4.1 How Stack Memory Works
The stack is a contiguous region of virtual memory managed by the `%rsp` register.
- Allocating local variables: `sub %rsp, N` (subtracts bytes, moving `%rsp` toward lower memory).
- Deallocating: `add %rsp, N`.

### 4.2 The Guard Page Mechanism
To prevent a runaway stack (e.g. infinite recursion or massive stack arrays) from silently corrupting neighboring heap or mapped memory:
1. When creating a thread, the OS maps a stack space (e.g. 2 MB for thread, 8 MB for main process).
2. Directly below the bottom of the valid stack region, the OS maps an unallocated page marked with `PROT_NONE` (Present bit = 0, no read/write/execute permissions). This is the **Guard Page**.
3. When `%rsp` crosses below the valid stack limit into the guard page, the CPU attempts to write to the unmapped page, instantly raising a Page Fault (`#PF`).
4. The OS kernel recognizes that the fault occurred within the thread's guard page boundary and safely kills the process with `SIGSEGV` / stack overflow abort.

```
High Memory  +-------------------------------+
             | main() frame                  |
             +-------------------------------+
             | helper() frame                |
             +-------------------------------+
             | %rsp -> current stack frame   |
             |           |                   |
             |           v (Stack grows down)|
             +-------------------------------+
             | [ GUARD PAGE: PROT_NONE ]     |  <-- %rsp hits this -> Instant SIGSEGV
Low Memory   +-------------------------------+
```

### 4.3 Why `Box::new([0; 1_000_000])` Can Overflow the Stack
In Rust, expressions passed as function arguments are evaluated by the caller **before** the function is invoked:
```rust
let a = Box::new([0; 1_000_000]);
```
1. `[0; 1_000_000]` is evaluated first. It is an array of 1,000,000 `i32` elements (4 MB).
2. In debug mode (unoptimized), the compiler allocates space for this 4 MB array on the **caller's stack frame**.
3. If running on a secondary thread with a 2 MB stack limit, adjusting `%rsp` by 4 MB overshoots the stack boundary directly into the guard page, triggering an instant hardware crash before `Box::new` can allocate heap memory.

---

## 5. Undefined Behavior (UB) and Optimizing Compilers

### 5.1 What is Undefined Behavior?
In systems programming (C, C++, unsafe Rust), **Undefined Behavior (UB)** means the language specification places **no requirements** on what the implementation must do.

**The Compiler Contract**:
The optimizing backend (LLVM) assumes that valid programs **never** execute undefined behavior under any circumstances. When UB is possible, the optimizer makes deductive assumptions to eliminate code, reorder instructions, or delete entire branches.

### 5.2 Classic UB Optimization Traps

#### 1. Dead Check Elimination (Null Pointer Dereference)
```c
// C Example
void test(int *ptr) {
    int val = *ptr; // Dereferenced before check
    if (ptr == NULL) {
        printf("ptr is null!\n"); // OPTIMIZED AWAY COMPLETELY!
    }
}
```
**Optimizer reasoning**: "The programmer dereferenced `ptr` on line 2. Dereferencing NULL is Undefined Behavior. Because the programmer cannot write UB, `ptr` is guaranteed never to be NULL. Therefore, the `if (ptr == NULL)` condition is always false and the check is deleted."

#### 2. Infinite Loop Deletion
Compilers assume loops without side-effects terminate. Non-terminating loops with no I/O can be completely removed from the binary.

#### 3. Signed Integer Overflow
In C/C++, signed integer overflow is UB. The compiler assumes `x + 1 > x` is always true and removes overflow checks.

---

## 6. The Memory Safety Flaw Taxonomy & Rust's Static Elimination

| Vulnerability Class | C/C++ Root Cause | Exploit Impact | Rust Static Invariant |
| :--- | :--- | :--- | :--- |
| **Use-After-Free (UAF)** | Dangling pointer dereference after `free()` | Arbitrary code execution, Heap spraying | **Affine Types & Ownership**: value moved or dropped cannot be accessed. |
| **Double Free** | Calling `free()` twice on same address | Allocator metadata corruption, Fastbin duplication | **Single Ownership**: exactly one owner drops resource upon scope exit. |
| **Buffer Overflow** | Unchecked pointer arithmetic / array indexing | Return address (`%rip`) overwriting, Stack smashing | **Fat Pointers & Bounds Checks**: slices store pointer + length. |
| **Null Dereference** | Dereferencing uninitialized or NULL pointer | Crash (`SIGSEGV`), denial of service | **No Null Pointers**: missing values modeled explicitly with `Option<T>`. |
| **Uninitialized Read** | Reading uninitialized stack/heap memory | Information leakage (keys, tokens, stale pointers) | **Definite Assignment**: compiler proves all variables initialized before read. |
| **Data Race** | Concurrent read/write without atomic synchronization | Memory corruption, non-deterministic state | **`Send` / `Sync` traits + Aliasing XOR Mutability**: compile-time proof. |

---

## 7. Deep Copy vs. Shallow Copy vs. Destructive Move

```
Memory Source (Stack + Heap):
Stack: [ ptr = 0x5000, len = 4 ]  --->  Heap [0x5000]: [10, 20, 30, 40]
```

### 7.1 Shallow Copy (C / Python / JavaScript)
- Copies only the stack descriptor (`ptr = 0x5000, len = 4`).
- **Hazard**: Both variables point to the same heap address `0x5000`. Modifying or freeing through one corrupts or invalidates the other (Aliasing Bug / Double Free).

### 7.2 Deep Copy (`Clone` in Rust)
- Allocates a brand new heap buffer `0x9000`.
- Copies all data elements from `0x5000` to `0x9000`.
- Creates new stack descriptor pointing to `0x9000`.
- **Properties**: Safe, independent, but expensive (O(N) memory and time).

### 7.3 Destructive Move (Rust Default)
- Copies the stack descriptor (`ptr = 0x5000, len = 4`) into the target slot.
- Statically **invalidates** the source binding at compile time.
- **Properties**: Zero heap overhead (O(1)), instantaneous, and mathematically guarantees zero aliasing and zero double-free vulnerabilities.

### 7.4 Scope of Invalidation: Heap Allocated vs. Stack-Only Types

> [!NOTE]
> Destructive Move invalidation applies **only** to heap-allocated types, collections, and resource handles that do not implement `Copy`. Stack-only types implementing `Copy` are duplicated on assignment without invalidating the source variable.

```
+-----------------------------------------------------------------------------------+
| HEAP-ALLOCATED / RESOURCE TYPES (MOVE -> SOURCE INVALIDATED)                       |
+-----------------------------------------------------------------------------------+
| String, Vec<T>, Box<T>, HashMap<K, V>, File, TcpStream, MutexGuard, Non-Copy Structs|
+-----------------------------------------------------------------------------------+
| STACK-ONLY PRIMITIVE TYPES (COPY -> SOURCE REMAINS FULLY VALID)                    |
+-----------------------------------------------------------------------------------+
| Integers (i8..i128, u8..u128, usize), Floats (f32, f64), bool, char, [T; N] (T:Copy) |
+-----------------------------------------------------------------------------------+
```
