; A series of initializations which are executed after the bootloader is done.
; they get the system ready for calling rust code and starting the kernel.
; It additionally gets the system in long mode.
;
; Author: Ardalan Ahanchi
; Date: Spring 2020

;# Constants Used in the initialization process ################################

VGA_BUFFER_START equ 0xb8000    ; Start of the VGA buffer in x86

PAGING_PAGE_SIZE equ 0x1000     ; Default to 4kb pages during boot.
PAGING_NUM_ENTRIES equ 512      ; Number of entries in each page table.
PAGING_ENTRY_CONFIG equ (1 << 0) | (1 << 1)
                                ; Default page table entries configuration.
                                ; It is Present and writable.

EFER_MSR equ 0xC0000080			; MSR number for the Extended Feature
								; ... Enable Register (EFER). Used for
								; ... enabling long mode.

; Fields used in the 64-bit GDT entries. These are based on the fields provided
; in the AMD64 programming manual in chapter 4.8. The segment entries are also
; specified there (they are a fixed value).
GDT_ENT_LONGMODE equ (1 << 53)            ; Running in 64-bit mode (not compat).
GDT_ENT_PRESENT equ (1 << 47)             ; Making this entry present.
GDT_ENT_CODE_SEG equ (1 << 43) | (1 << 44); In code segment, bits 43-44 are set.
GDT_ENT_DATA_SEG equ (1 << 44)            ; In data segment, only bit 44 is set.
GDT_ENT_USER equ (1 << 45) | (1 << 46)    ; User mode flags (ring 3).

GDT_MAX_TSS_ENTRIES equ 1                 ; Max number of empty TSS entries.

NULL equ 0						          ; For reseting (more readable syntax).

;# 32-bit data and code section ################################################

global _start_32                                    ; Label the start of boot

bits 32                                             ; 32-bit Instructions

; BSS and Stack setup lines were borrowed from:
; https://wiki.osdev.org/Bare_Bones_with_NASM
; http://ringzeroandlower.com/2017/08/08/x86-64-kernel-boot.html

section .bss                                        ; Define the BSS section

align 4096                                          ; Page alligned

; Initial page tables used in the boot enviornment (4 level paging).
; It can page a total of 1GB of memory (in fact, it identity maps it).
PML4:                                               ; Page map level 4
    resb 4096
PDP:                                                ; Page directory pointer
    resb 4096
PD:                                                 ; Page directory
    resb 4096
PT:                                                 ; Page tables (512 of them)
    resb 4096 * 512

stack_end:                                          ; Define the stack
    resb 16384                                      ; Reseve 16 KiloBytes for it
stack_top:

section .data

; The long mode GDT which is set-up so we can actually execute 64-bit code
; and exit the 32-bit compatibility. This will be replaced when the kernel
; starts to allow a more complete configuration. Explained in great detail in:
; https://os.phil-opp.com/entering-longmode/
; https://wiki.osdev.org/Setting_Up_Long_Mode
; https://en.wikipedia.org/wiki/X86_memory_segmentation
; https://intermezzos.github.io/book/first-edition/setting-up-a-gdt.html
; http://www.jamesmolloy.co.uk/tutorial_html/4.-The%20GDT%20and%20IDT.html
global gdt
global gdt.tss

align 8
gdt:                       ; Define a x86_64 GDT. See AMD programming ch. 4.8.
.null: equ $ - gdt         ; GDT's First entry (zero)
    dq 0
.code: equ $ - gdt         ; Code descriptor is present, 64-bit and flagged.
    dq GDT_ENT_PRESENT | GDT_ENT_LONGMODE | GDT_ENT_CODE_SEG
.data: equ $ - gdt         ; Data descriptor is present and flagged.
    dq GDT_ENT_PRESENT | GDT_ENT_DATA_SEG
.user_code: equ $ - gdt    ; Code descriptor for users, present, 64-bit.
    dq GDT_ENT_PRESENT | GDT_ENT_LONGMODE | GDT_ENT_CODE_SEG | GDT_ENT_USER
.user_data: equ $ - gdt    ; Data descriptor for users. present and flagged.
    dq GDT_ENT_PRESENT | GDT_ENT_DATA_SEG | GDT_ENT_USER
.tss: equ $ - gdt          ; The entries for the tss (each take 16 bytes).
    times GDT_MAX_TSS_ENTRIES dq 0
    times GDT_MAX_TSS_ENTRIES dq 0
.ptr:                      ; Make a Descriptor table ptr as speicified by arch.
    dw $ - gdt - 1
    dq gdt

halt_msg: db "Oxid Halted. Error Number: "
.len: equ $ - halt_msg
.color: equ 0x04

section .text

; The starting point of the kernel which does the checks, enables long mode,
; and calls the rust code. It should be linked with a multiboot compatible
; header. It only supports x86-64.
_start_32:
    ; Grub passes the multiboot_info address inside ebx, store it at edi
    ; (since in SystemV ABI, the RDI will hold the first parameter).
    mov edi, ebx

    ; Setup the stack (move the address to stack_top to the esp)
    mov esp , stack_top

    ; Setup paging and enable long mode
    call _init_long_mode

    ; Make sure interrupts are disabled.
    cli

    ; Load GDT (required for running 64-bit code).
    lgdt [gdt.ptr]

    ; Do a far Jump to the 64-bit start routine (get the segment from gdt)
    jmp gdt.code:_start_64

    ; If we get here, something went terribly wrong. Halt the system (error 0).
    mov al , "0"
    jmp _halt_32

; A routine which identity maps the first GB of the kernel using 4KB pages and
; intel's 4-level paging. Please keep in mind that this code should be called
; prior to enabling long mode or paging.
_identity_map_first_gb:
    ; Store the current value of registers just in case.
    push eax
    push ebx
    push ecx
    push edx
    push edi

    ; Setup a simple paging structure used just for bootstrapping.
    ; Map pages from the different tables to allow identity paging.
    mov eax , PDP                           ; Map the PDP into PML
    or eax ,  PAGING_ENTRY_CONFIG
    mov [PML4] , eax

    mov eax , PD                            ; Map the PD into PDP
    or eax , PAGING_ENTRY_CONFIG
    mov [PDP] , eax

    mov ecx , 0                             ; Current PT number (within PD)
    mov ebx , 0                             ; Current address we're mapping.

.map_pd:                                    ; Map all the indexes within PD.
    mov eax , PAGING_PAGE_SIZE              ; Calculate the current PT address
    mul ecx                                 ; and store it in eax.
    add eax , PT

    mov edi , eax                           ; Create an entry, and set it at
    or edi , PAGING_ENTRY_CONFIG            ; the correct index within PD.
    mov [PD + ecx * 8] , edi

    mov edi , 0                             ; The index within PT.

.map_pt:
    mov edx , ebx                           ; Create an entry for the PT
    or edx , PAGING_ENTRY_CONFIG            ; using the current address
    mov [eax + edi * 8] , edx               ; and set it at the correct index.

    add ebx , PAGING_PAGE_SIZE              ; Increase the current address.
    inc edi                                 ; Increase the second loop counter.
    cmp edi , PAGING_NUM_ENTRIES
    jne _identity_map_first_gb.map_pt      	; If not done, countinue loop 2.

    inc ecx							        ; Increase the first loop counter.
    cmp ecx , PAGING_NUM_ENTRIES
    jne _identity_map_first_gb.map_pd      	; If not done, countinue loop 1.

    ; Restore the current value of registers from the stack.
    pop edi
    pop edx
    pop ecx
    pop ebx
    pop eax

    ret                                     ; Return to the callee.

; A routine which sets up the page tables by initializing the entries and
; pointing each table to the next. Finally, it loads the
; location of PML4 into the cr4 register, enables PAE, and long mode.
_init_long_mode:
    ; Identity map the first GB of the memory.
    call _identity_map_first_gb

    ; Enable Physical Address Extensions (PAE)
    mov eax , cr4						; Store the previous value of CR4
    or eax , (1 << 5)					; Set the PAE bit (5th Bit)
    mov cr4 , eax						; Save the new value of CR4

    ; Put the address of PML4 into the cr3 register (used by MMU for paging)
    mov eax , PML4
    mov cr3 , eax

    ; Enable long mode by setting the LME bit in the Extended Feature
    ; enable Register (EFER). Explained here in more detail:
    ; https://en.wikipedia.org/wiki/Control_register#EFER
    mov ecx , EFER_MSR				; The MSR number is put into ecx
    rdmsr							; Value of MSR is read into eax
    or eax , (1 << 8)				; Set the LME (Long mode enable) bit
    wrmsr							; Write back the MSR into register

    ; Finally, enable paging by setting the PG bit of the CR0 register
    ; explained at: https://en.wikipedia.org/wiki/Control_register#CR0
    mov eax , cr0								    ; Read CR0 into eax
    or eax , (1 << 31)								; Set the PG bit
    mov cr0 , eax								    ; Write back to CR0

    ret										        ; Return back

; A routine which halts the system, and prints a string (without using any
; dependencies, prints an error code, and halts the system.
; The error code should be stored in AL (this is specific to 32-bit mode)
_halt_32:
    ; A loop for printing the halt message.
    mov ecx , 0									    		   ; Loop's index

.print_next_char_32:
	mov byte bl , [halt_msg + ecx]							   ; Store the char
	mov byte [VGA_BUFFER_START + 2 * ecx] , bl				   ; Put it in VGA
	mov byte [VGA_BUFFER_START + 2 * ecx + 1] , halt_msg.color ; Set color

	; Loop semantics
	inc ecx
    cmp ecx , halt_msg.len									   ; Check the index
    jne _halt_32.print_next_char_32

    ; Print the error number from the AL register
    mov byte [VGA_BUFFER_START + 2 * ecx] , al        	  	   ; Print error num
    mov byte [VGA_BUFFER_START + 2 * ecx + 1] , halt_msg.color ; Set color

    hlt

;# 64-bit code section #########################################################

bits 64

; Define the external function which is the kernel's main function
extern kernel_main

; The starting point of long mode. The kernel would be called from here.
_start_64:
    ; Reset all the segment selectors (to override the old GDT offsets)
    call _reset_seg_selectors

    ; Clear the direction flag as specified by the sysV ABI.
    cld

    ; Call the rust code from here
    call kernel_main

    ; If we get here, there was an issue, print an error and halt.
    mov al , "1"
    jmp _halt_64

; A routine which fully resets the segment selectors (ds, es, fs, gs, and ss)
; More information at: http://ringzeroandlower.com/2017/08/08/x86-64-kernel-boot.html
_reset_seg_selectors:
	mov ax , NULL			; First store 0 into ax
	mov ds , ax			    ; Apply it to all segment selectors
	mov es , ax
	mov fs , ax
	mov gs , ax
	mov ss , ax

	ret

; A routine which halts the system, and prints a string (without using any
; dependencies, prints an error code, and halts the system.
; The error code should be stored in AL (this is specific to 64-bit mode)
_halt_64:
    ; A loop for printing the halt message.
    mov rcx , 0													; Loop's index

.print_next_char_64:
	mov byte bl , [halt_msg + rcx]								; Store the char
	mov byte [VGA_BUFFER_START + 2 * rcx] , bl					; Put it in VGA
	mov byte [VGA_BUFFER_START + 2 * rcx + 1] , halt_msg.color 	; Set color

	; Loop semantics
	inc rcx
    cmp rcx , halt_msg.len										; Check the index
    jne _halt_64.print_next_char_64

    ; Print the error number from the AL register
    mov byte [VGA_BUFFER_START + 2 * rcx] , al        	  	   	; Print error num
    mov byte [VGA_BUFFER_START + 2 * rcx + 1] , halt_msg.color 	; Set color

    hlt
