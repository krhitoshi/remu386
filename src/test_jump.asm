; nasm -f bin -o test_jump src/test_jump.asm
; ndisasm -b 32 test_jump
;
BITS 32
start:
    jmp label1
label2:
    ret
label1:
    mov eax, 54
    jmp label2