.code64
.global idt_flush

idt_flush:
    lidt (%rdi)
    ret
