; Some wrappers for calling the rust based interrupt sevice routine. It allows
; the usage of wrappers to avoid messing up the stack during interrupt handling.
; it also pushes the context, and passes the information to a main interrupt
; service routine which will then call the registered high level handlers.
; More details can be found at: https://wiki.osdev.org/IDT#Structure_AMD64
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

section .text

; The main rust handler which will be called (it then calls all other handlers).
%define RUST_MAIN_HANDLER main_handler

; Make sure the default rust handlers are accessible here.
extern RUST_MAIN_HANDLER

;  The main isr handling  :;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; A macro for saving all the general purpose registers (the current context).
%macro save_context 0
    push rax
    push rbx
    push rcx
    push rdx
    push rbp
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
%endmacro

; A macro for loading all the general purpose registers (the current context).
%macro load_context 0
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rbp
    pop rdx
    pop rcx
    pop rbx
    pop rax
%endmacro

; A macro which makes a basic wrapper for the interrupts which don't push an
; error code to the stack. It saves the current context, calls the rust handler
; (to avoid stack issues), and returns from the interrupt. The first argument
; is the index within the IDT. It will always call the default handler and
; passes the interrupt number, and a pointer to the context structure (defined
; in arch::interrupts::handlers::context). A good source for reading about what
; is pushed on the stack by the cpu is https://os.phil-opp.com/cpu-exceptions/
%macro make_handler 1
    global int_%1_handler           ; Make the isr accessible to rust.
    align 4

    int_%1_handler:                 ; Create a label.
        push 0                      ; Just add a 0 error number.
        save_context                ; Save the general purpose registers.
        mov rdi, %1                 ; Push the interrupt number as first arg.
        mov rsi, rsp                ; Context struct pointer as second arg.
        cld                         ; Clear the direction flag (sysv64 ABI).
        call RUST_MAIN_HANDLER      ; Call the high level handler.
        load_context                ; Load back the gp registers.
        pop rax                     ; Pop the error number.
        iretq                       ; Return from the interrupt.
%endmacro

; A macro which makes a basic wrapper for the interrupts which do push an
; error code to the stack. Based on the architecture, the interrupts 8, 10,
; 11, 12, 13, 14, and 17 have error pushed onto the stack. This macro saves
; the current context, calls the rust handler (to avoid stack issues), and
; returns from the interrupt. The first argument is the index within the IDT.
; It will always call the default handler and passes the interrupt number, and
; a pointer to the context structure (defined in arch::interrupts::handlers
; ::context). A good source for reading about what is pushed on the
;  stack by the cpu is https://os.phil-opp.com/cpu-exceptions/
%macro make_handler_err 1
    global int_%1_handler           ; Make the isr accessible to rust.
    align 4

    int_%1_handler:                 ; Create a label.
        save_context                ; Save the general purpose registers.
        mov rdi, %1                 ; Interrupt number as first arg.
        mov rsi, rsp                ; Context struct pointer as second arg.
        cld                         ; Clear the direction flag (sysv64 ABI).
        call RUST_MAIN_HANDLER      ; Call the high level handler.
        load_context                ; Load back the gp registers.
        pop rax                     ; Pop the error number.
        iretq                       ; Return from the interrupt.
%endmacro

; Handlers are defined here ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; A python script was used to generate these definitions.
make_handler 0
make_handler 1
make_handler 2
make_handler 3
make_handler 4
make_handler 5
make_handler 6
make_handler 7
make_handler_err 8
make_handler 9
make_handler_err 10
make_handler_err 11
make_handler_err 12
make_handler_err 13
make_handler_err 14
make_handler 15
make_handler 16
make_handler_err 17
make_handler 18
make_handler 19
make_handler 20
make_handler 21
make_handler 22
make_handler 23
make_handler 24
make_handler 25
make_handler 26
make_handler 27
make_handler 28
make_handler 29
make_handler 30
make_handler 31
make_handler 32
make_handler 33
make_handler 34
make_handler 35
make_handler 36
make_handler 37
make_handler 38
make_handler 39
make_handler 40
make_handler 41
make_handler 42
make_handler 43
make_handler 44
make_handler 45
make_handler 46
make_handler 47
make_handler 48
make_handler 49
make_handler 50
make_handler 51
make_handler 52
make_handler 53
make_handler 54
make_handler 55
make_handler 56
make_handler 57
make_handler 58
make_handler 59
make_handler 60
make_handler 61
make_handler 62
make_handler 63
make_handler 64
make_handler 65
make_handler 66
make_handler 67
make_handler 68
make_handler 69
make_handler 70
make_handler 71
make_handler 72
make_handler 73
make_handler 74
make_handler 75
make_handler 76
make_handler 77
make_handler 78
make_handler 79
make_handler 80
make_handler 81
make_handler 82
make_handler 83
make_handler 84
make_handler 85
make_handler 86
make_handler 87
make_handler 88
make_handler 89
make_handler 90
make_handler 91
make_handler 92
make_handler 93
make_handler 94
make_handler 95
make_handler 96
make_handler 97
make_handler 98
make_handler 99
make_handler 100
make_handler 101
make_handler 102
make_handler 103
make_handler 104
make_handler 105
make_handler 106
make_handler 107
make_handler 108
make_handler 109
make_handler 110
make_handler 111
make_handler 112
make_handler 113
make_handler 114
make_handler 115
make_handler 116
make_handler 117
make_handler 118
make_handler 119
make_handler 120
make_handler 121
make_handler 122
make_handler 123
make_handler 124
make_handler 125
make_handler 126
make_handler 127
make_handler 128
make_handler 129
make_handler 130
make_handler 131
make_handler 132
make_handler 133
make_handler 134
make_handler 135
make_handler 136
make_handler 137
make_handler 138
make_handler 139
make_handler 140
make_handler 141
make_handler 142
make_handler 143
make_handler 144
make_handler 145
make_handler 146
make_handler 147
make_handler 148
make_handler 149
make_handler 150
make_handler 151
make_handler 152
make_handler 153
make_handler 154
make_handler 155
make_handler 156
make_handler 157
make_handler 158
make_handler 159
make_handler 160
make_handler 161
make_handler 162
make_handler 163
make_handler 164
make_handler 165
make_handler 166
make_handler 167
make_handler 168
make_handler 169
make_handler 170
make_handler 171
make_handler 172
make_handler 173
make_handler 174
make_handler 175
make_handler 176
make_handler 177
make_handler 178
make_handler 179
make_handler 180
make_handler 181
make_handler 182
make_handler 183
make_handler 184
make_handler 185
make_handler 186
make_handler 187
make_handler 188
make_handler 189
make_handler 190
make_handler 191
make_handler 192
make_handler 193
make_handler 194
make_handler 195
make_handler 196
make_handler 197
make_handler 198
make_handler 199
make_handler 200
make_handler 201
make_handler 202
make_handler 203
make_handler 204
make_handler 205
make_handler 206
make_handler 207
make_handler 208
make_handler 209
make_handler 210
make_handler 211
make_handler 212
make_handler 213
make_handler 214
make_handler 215
make_handler 216
make_handler 217
make_handler 218
make_handler 219
make_handler 220
make_handler 221
make_handler 222
make_handler 223
make_handler 224
make_handler 225
make_handler 226
make_handler 227
make_handler 228
make_handler 229
make_handler 230
make_handler 231
make_handler 232
make_handler 233
make_handler 234
make_handler 235
make_handler 236
make_handler 237
make_handler 238
make_handler 239
make_handler 240
make_handler 241
make_handler 242
make_handler 243
make_handler 244
make_handler 245
make_handler 246
make_handler 247
make_handler 248
make_handler 249
make_handler 250
make_handler 251
make_handler 252
make_handler 253
make_handler 254
make_handler 255
