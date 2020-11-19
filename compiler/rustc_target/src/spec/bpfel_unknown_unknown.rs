use crate::spec::abi::Abi;
use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, Target, TargetOptions};

// All the calling conventions trigger an assertion(Unsupported calling
// convention) in llvm on BPF
pub fn unsupported_abis() -> Vec<Abi> {
    vec![
        Abi::Cdecl,
        Abi::Stdcall,
        Abi::Fastcall,
        Abi::Vectorcall,
        Abi::Thiscall,
        Abi::Aapcs,
        Abi::Win64,
        Abi::SysV64,
        Abi::PtxKernel,
        Abi::Msp430Interrupt,
        Abi::X86Interrupt,
        Abi::AmdGpuKernel,
    ]
}

pub fn target() -> Target {
    Target {
        llvm_target: "bpf".to_string(),
        data_layout: "e-m:e-p:64:64-i64:64-n32:64-S128".to_string(),
        arch: "bpf".to_string(),
        pointer_width: 64,

        options: TargetOptions {
            endian: "little".to_string(),
            is_builtin: true,
            c_int_width: "64".to_string(),
            os: "unknown".to_string(),
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            dynamic_linking: true,
            executables: true,
            dll_prefix: "".to_string(),
            no_default_libraries: true,
            position_independent_executables: true,
            max_atomic_width: Some(64),
            panic_strategy: PanicStrategy::Abort,
            unsupported_abis: unsupported_abis(),
            singlethread: true,
            no_builtins: true,

            ..Default::default()
        },
    }
}
