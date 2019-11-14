; nasm -f bin -o test_func src/test_func.asm
; ndisasm -b 32 test_func
;
BITS 32
start:
    push 23
    push 12
    call add
    add esp, 8
    mov ebx, eax

    push 21; arg2
    push 52; arg1
    call sub
    add esp, 8
    ret
add:
    push ebp
    mov ebp, esp

    mov eax, [ebp+8]
    add eax, [ebp+12]

    mov esp, ebp
    pop ebp
    ret

sub:
    push ebp
    mov ebp, esp

    mov eax, [ebp+8]; arg1
    sub eax, [ebp+12]; arg2

    mov esp, ebp
    pop ebp
    ret