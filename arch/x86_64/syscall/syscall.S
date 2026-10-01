.code64
.global pyxis_syscall_entry
.extern syscall_handler

pyxis_syscall_entry:
    /* Save user state */
    pushq %rcx /* User RIP in syscall instruction */
    pushq %r11 /* User RFLAGS */
    pushq %rbp
    pushq %rbx
    pushq %r12
    pushq %r13
    pushq %r14
    pushq %r15

    /* Dispatch to C handler:
     * %rdi = syscall number (from %rax)
     * %rsi = arg1 (%rdi)
     * %rdx = arg2 (%rsi)
     * %rcx = arg3 (%rdx)
     */
    movq %rax, %r8
    movq %rdi, %rcx
    movq %rsi, %rdx
    movq %rdx, %rsi
    movq %r8,  %rdi
    call syscall_handler

    /* Restore user state */
    popq %r15
    popq %r14
    popq %r13
    popq %r12
    popq %rbx
    popq %rbp
    popq %r11
    popq %rcx

    /* Return to user mode */
    sysretq
