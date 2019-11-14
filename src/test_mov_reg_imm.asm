; nasm -f bin -o test1 src/test1.asm
; ndisasm -b 32 test1
;
BITS 32
start:
    mov eax, 555959652
    mov ecx, 0x33
    mov edi, 0x1256
    ret
;
; 00000000  B821000000        mov eax,0x21
; 00000005  C3                ret