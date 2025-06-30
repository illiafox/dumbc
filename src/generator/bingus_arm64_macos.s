    .text
    .global bingus
    .p2align    2

bingus:
    // — Prologue: save fp & lr, alloc 32-byte frame
    stp     x29, x30, [sp, #-32]!
    mov     x29, sp

    // save original w0 for sign test
    str     w0, [sp, #16]

    // sign-extend w0 → x1
    sxtw    x1, w0

    // x2 → end-of-buffer on stack
    mov     x2, sp
    add     x2, x2, #32

    // init counters & flags
    mov     w3, 0           // digit count
    mov     w7, 0           // initialize negative flag
    // if x1 == 0 → handle “0”
    cbz     x1, .zero_case

    // if negative, negate & mark flag
    cmp     x1, #0
    b.ge    .convert_loop
    neg     x1, x1
    mov     w7, 1           // mark negative
    b       .convert_start

.convert_loop:
    mov     x4, 10
    udiv    x5, x1, x4      // q = x1/10
    msub    x6, x5, x4, x1  // r = x1 – q*10
    add     w6, w6, #'0'    // ascii digit
    sub     x2, x2, 1
    strb    w6, [x2]
    mov     x1, x5
    add     w3, w3, 1
    cbnz    x1, .convert_loop
    b       .maybe_negative_sign

.convert_start:
    // (we already set w4 above)
    b       .convert_loop

.zero_case:
    sub     x2, x2, 1
    mov     w6, #'0'
    strb    w6, [x2]
    mov     w3, 1
    mov     w4, 0
    b       .print_number

.maybe_negative_sign:
    cmp     w7, 0
    beq     .print_number
    sub     x2, x2, 1
    mov     w6, #'-'
    strb    w6, [x2]
    add     w3, w3, 1

.print_number:
    // — write number (syscall write: x16=4, svc #0x80)
    mov     x0, 1           // fd = stdout
    mov     x1, x2          // buf ptr
    uxtw    x2, w3          // length
    mov     x16, #4         // macOS write
    svc     #0x80

    // — write newline (stack slot sp+31)
    mov     x0, 1
    mov     w6, #'\n'
    strb    w6, [sp, #31]
    add     x1, sp, #31
    mov     x2, 1
    mov     x16, #4
    svc     #0x80

    // — Epilogue
    ldp     x29, x30, [sp], #32
    ret
