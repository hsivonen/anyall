# anyall

A demo for inspecting the generated code for boolean reductions using both the `simd` and `stdsimd` crates.

Have a clone of `stdsimd` in an adjacent directory.

```bash
cargo rustc --release -- --emit asm
less target/release/deps/anyall-*.s
```

## Licensing

CC0

## Result

Using rustc 1.26.0-nightly (2789b067d 2018-03-06) on x86_64:

```asm
    .text
    .file   "anyall0-66776763bbd7db1b14e447f414e2cf9a.rs"
    .section    .text.stdsimd_any_8x16,"ax",@progbits
    .globl  stdsimd_any_8x16
    .p2align    4, 0x90
    .type   stdsimd_any_8x16,@function
stdsimd_any_8x16:
    .cfi_startproc
    movdqa  (%rdi), %xmm0
    pshufd  $78, %xmm0, %xmm1
    por %xmm0, %xmm1
    pshufd  $229, %xmm1, %xmm0
    por %xmm1, %xmm0
    movdqa  %xmm0, %xmm1
    psrld   $16, %xmm1
    por %xmm0, %xmm1
    movdqa  %xmm1, %xmm0
    psrlw   $8, %xmm0
    por %xmm1, %xmm0
    movd    %xmm0, %eax
    testb   %al, %al
    setne   %al
    retq
.Lfunc_end0:
    .size   stdsimd_any_8x16, .Lfunc_end0-stdsimd_any_8x16
    .cfi_endproc

    .section    .text.stdsimd_all_8x16,"ax",@progbits
    .globl  stdsimd_all_8x16
    .p2align    4, 0x90
    .type   stdsimd_all_8x16,@function
stdsimd_all_8x16:
    .cfi_startproc
    movdqa  (%rdi), %xmm0
    pshufd  $78, %xmm0, %xmm1
    pand    %xmm0, %xmm1
    pshufd  $229, %xmm1, %xmm0
    pand    %xmm1, %xmm0
    movdqa  %xmm0, %xmm1
    psrld   $16, %xmm1
    pand    %xmm0, %xmm1
    movdqa  %xmm1, %xmm0
    psrlw   $8, %xmm0
    pand    %xmm1, %xmm0
    movd    %xmm0, %eax
    testb   %al, %al
    setne   %al
    retq
.Lfunc_end1:
    .size   stdsimd_all_8x16, .Lfunc_end1-stdsimd_all_8x16
    .cfi_endproc

    .section    .text.simd_any_8x16,"ax",@progbits
    .globl  simd_any_8x16
    .p2align    4, 0x90
    .type   simd_any_8x16,@function
simd_any_8x16:
    .cfi_startproc
    movdqa  (%rdi), %xmm0
    pmovmskb    %xmm0, %eax
    testl   %eax, %eax
    setne   %al
    retq
.Lfunc_end2:
    .size   simd_any_8x16, .Lfunc_end2-simd_any_8x16
    .cfi_endproc

    .section    .text.simd_all_8x16,"ax",@progbits
    .globl  simd_all_8x16
    .p2align    4, 0x90
    .type   simd_all_8x16,@function
simd_all_8x16:
    .cfi_startproc
    movdqa  (%rdi), %xmm0
    pmovmskb    %xmm0, %eax
    cmpl    $65535, %eax
    sete    %al
    retq
.Lfunc_end3:
    .size   simd_all_8x16, .Lfunc_end3-simd_all_8x16
    .cfi_endproc


    .section    ".note.GNU-stack","",@progbits
```
