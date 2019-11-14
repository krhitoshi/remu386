; nasm -f bin -o test3 src/test3.asm
; ndisasm -b 32 test3
;
BITS 32
start:
    mov ebp, esp
    push esp
    mov ebx, 0x71234564
    push ebx
    pop eax
    mov ecx, 0x2553
    push ecx
    pop edx
    ret