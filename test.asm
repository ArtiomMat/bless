section .text
global _gcd

_gcd:
    push    ebp ; Looks like cdecl
    mov     ebp, esp
    mov     eax, [ebp+8] ; Skip return address, get 4 bytes into eax. call it X.
    mov     ecx, [ebp+12] ; Now another 4 bytes. call it Y
.L1:
    test    ecx, ecx ; ecx&ecx
    je      .L2 ; if ecx(which is ecx&ecx) is 0, to .L2
    mov     edx, eax ; Save eax(X)
    mov     eax, ecx ; ecx to eax
    xor     edx, edx ; edx=0
    div     ecx ; eax=X/Y
    mov     ecx, edx ; ecx=X%Y
    jmp     .L1
.L2:
    mov     esp, ebp
    pop     ebp
    ret
