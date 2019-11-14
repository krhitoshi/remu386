; nasm -f bin -o test_call_pop src/test_call_ret.asm
; ndisasm -b 32 test_call_ret
;
BITS 32
start:
    call sub
    ret
sub:
    mov eax, 0x43252
    ret
