.code64
.global pyxis_context_switch

/*
 * void pyxis_context_switch(uint64_t *old_rsp_ptr, uint64_t new_rsp);
 *
 * System V AMD64 ABI:
 *   %rdi = old_rsp_ptr (pointer to store current stack pointer)
 *   %rsi = new_rsp     (new stack pointer to switch to)
 */
pyxis_context_switch:
    /* Save callee-preserved registers */
    pushq %r15
    pushq %r14
    pushq %r13
    pushq %r12
    pushq %rbx
    pushq %rbp

    /* Save current stack pointer */
    movq %rsp, (%rdi)

    /* Load new stack pointer */
    movq %rsi, %rsp

    /* Restore callee-preserved registers */
    popq %rbp
    popq %rbx
    popq %r12
    popq %r13
    popq %r14
    popq %r15

    ret
