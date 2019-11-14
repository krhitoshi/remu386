; nasm -f bin -o test_add_sub src/test_add_sub.asm
; ndisasm -b 32 test_add_sub
;
BITS 32
start:
    mov eax, 156
    add eax, 234
    sub eax, 200
    mov ebx, 642
    sub ebx, -12
    add ebx, 22
    add ebx, 2222
    sub ebx, 1111
    ret