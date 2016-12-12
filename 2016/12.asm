; nasm -f elf64 12.asm
; ld 12.o -o 12
section	.text
    global _start
_start:
    ; start input code
    mov rax,1
    mov rbx,1
    mov rdx,26
    mov rcx,1
    cmp rcx,0
    jne l1
    jmp l2
l1: mov rcx,7
l3: inc rdx
    dec rcx
    cmp rcx,0
    jne l3
l2: mov rcx,rax
l4: inc rax
    dec rbx
    cmp rbx,0
    jne l4
    mov rbx,rcx
    dec rdx
    cmp rdx,0
    jne l2
    mov rcx,17
l6: mov rdx,18
l5: inc rax
    dec rdx
    cmp rdx,0
    jne l5
    dec rcx
    cmp rcx,0
    jne l6
    ; stop input code

    call printNumber

    mov	rax,1       ;system call number (sys_exit)
    int	0x80        ;call kernel

printNumber:
    push rax
    push rdx
    xor rdx,rdx          ;rdx:rax = number
    div dword [const10]  ;rax = quotient, rdx = remainder
    test rax,rax         ;Is quotient zero?
    je .l1               ; yes, don't display it
    call printNumber     ;Display the quotient
.l1:
    lea rax,[rdx+'0']
    mov [buf],rax
    mov	rdx,1     ;message length
    mov	rcx,buf     ;message to write
    mov	rbx,1       ;file descriptor (stdout)
    mov	rax,4       ;system call number (sys_write)
    int	0x80        ;call kernel
    pop rdx
    pop rax
    ret
section .data
    buf db 0
    const10 dd 10
