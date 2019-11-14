; nasm -f bin -o test2 src/test2.asm
; ndisasm -b 32 test2
;
BITS 32
start:
    mov esp, 0x71234564
    mov ebp, esp
    mov ebx, ebp
    ret