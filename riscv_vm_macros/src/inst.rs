use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self},
    Block, Ident, Result, Token,
};

enum InstType {
    R,
    // FR,
    R4,
    RMem,
    I,
    IMem,
    // FIMem,
    S,
    SMem,
    // FSMem,
    U,
}

enum RegType {
    Int,
    Float,
}

#[derive(PartialEq, Eq)]
enum RegisterArg {
    Rd,
    Rs1,
    Rs2,
    Rs3,
    Rs4,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum XLen {
    B32,
    B64,
    B128,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FLen {
    F32,
    F64,
    F128,
}

struct InstSyntax {
    name: Ident,
    _paren_token: token::Paren,
    inst_type: Ident,
    _for_token: Token![for],
    _bracket_token1: token::Bracket,
    lengths: Vec<Ident>,
    _where_token: Token![where],
    _bracket_token2: token::Bracket,
    arguments: Vec<ArgumentType>,
    _colon_token: Token![:],
    code: Block,
}

struct ArgumentType {
    register: RegisterArg,
    r#type: RegType,
}

impl Parse for ArgumentType {
    fn parse(input: ParseStream) -> Result<Self> {
        let register: Ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let r#type: Ident = input.parse()?;

        let register = match register.to_string().as_str() {
            "rd" => RegisterArg::Rd,
            "rs1" => RegisterArg::Rs1,
            "rs2" => RegisterArg::Rs2,
            "rs3" => RegisterArg::Rs3,
            "rs4" => RegisterArg::Rs4,
            _ => panic!("Invalid register argument name"),
        };

        let r#type = match r#type.to_string().as_str() {
            "int" => RegType::Int,
            "float" => RegType::Float,
            _ => panic!("Invalid register argument type"),
        };

        Ok(Self { register, r#type })
    }
}

struct Inst {
    name: Ident,
    inst_type: InstType,
    code: Block,
    int_lengths: Vec<XLen>,
    float_length: Option<FLen>,
    registers: Vec<ArgumentType>,
}

impl Parse for Inst {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            panic!("Invalid Instruction Signature")
        }
        let inst_type;
        let lengths;
        let arguments;
        let syntax = InstSyntax {
            name: input.parse()?,
            _paren_token: parenthesized!(inst_type in input),
            inst_type: inst_type.parse()?,
            _for_token: input.parse()?,
            _bracket_token1: bracketed!(lengths in input),
            lengths: Punctuated::<Ident, Token![,]>::parse_terminated(&lengths)?
                .into_iter()
                .collect(),
            _where_token: input.parse()?,
            _bracket_token2: bracketed!(arguments in input),
            arguments: Punctuated::<ArgumentType, Token![,]>::parse_terminated(&arguments)?
                .into_iter()
                .collect(),
            _colon_token: input.parse()?,
            code: input.parse()?,
        };

        let inst_type = match syntax.inst_type.to_string().as_str() {
            "r" => InstType::R,
            // "fr" => InstType::FR,
            "r4" => InstType::R4,
            "r_mem" => InstType::RMem,
            "i" => InstType::I,
            "i_mem" => InstType::IMem,
            // "fi_mem" => InstType::FIMem,
            "s" => InstType::S,
            "s_mem" => InstType::SMem,
            // "fs_mem" => InstType::FSMem,
            "u" => InstType::U,
            t => panic!("Invalid Instuction Type: {}", t),
        };

        let mut int_lengths = Vec::new();
        for l in &syntax.lengths {
            match l.to_string().as_str() {
                "b32" => int_lengths.push(XLen::B32),
                "b64" => int_lengths.push(XLen::B64),
                "b128" => int_lengths.push(XLen::B128),
                "f32" | "f64" | "f128" => {}
                _ => panic!("Invalid XLen, expected 32, 64, or 128"),
            }
        }
        let mut float_length = None;
        for l in &syntax.lengths {
            match l.to_string().as_str() {
                "f32" => {
                    if float_length.is_none() {
                        float_length = Some(FLen::F32)
                    } else {
                        panic!("Instructions ccan be generated only for one float size, not more")
                    }
                }
                "f64" => {
                    if float_length.is_none() {
                        float_length = Some(FLen::F64)
                    } else {
                        panic!("Instructions ccan be generated only for one float size, not more")
                    }
                }
                "f128" => {
                    if float_length.is_none() {
                        float_length = Some(FLen::F128)
                    } else {
                        panic!("Instructions ccan be generated only for one float size, not more")
                    }
                }
                "b32" | "b64" | "b128" => {}
                _ => panic!("Invalid FLen, expected 32, 64, or 128"),
            }
        }

        int_lengths.dedup();

        Ok(Self {
            name: syntax.name,
            inst_type,
            code: syntax.code,
            int_lengths,
            float_length,
            registers: syntax.arguments,
        })
    }
}

pub(super) fn inst_internal(input: TokenStream) -> TokenStream {
    let Inst {
        name,
        inst_type,
        code,
        int_lengths,
        float_length,
        registers,
    } = parse_macro_input!(input as Inst);

    let mut impls = Vec::new();
    let xlen32 = quote!(i32);
    let b32len = quote!(
        type ixlen = i32;
        type uxlen = u32;
        type iexlen = i64;
        type uexlen = u64;
        const xlen: usize = 32;
    );
    let xlen64 = quote!(i64);
    let b64len = quote!(
        type ixlen = i64;
        type uxlen = u64;
        type iexlen = i128;
        type uexlen = u128;
        const xlen: usize = 64;
    );

    for l in int_lengths {
        let (name, len_types, xlen) = match l {
            XLen::B32 => (format_ident!("{}_32", name), &b32len, &xlen32),
            XLen::B64 => (format_ident!("{}_64", name), &b64len, &xlen64),
            XLen::B128 => panic!("128 Bit instruction have yet to be implemented"),
        };

        let rd_type = registers
            .iter()
            .find(|a| a.register == RegisterArg::Rd)
            .map(|r| match r.r#type {
                RegType::Int => xlen.clone(),
                RegType::Float => {
                    let Some(flen) = float_length else {
                        panic!("Float arguments require a specified float length");
                    };
                    match flen {
                        FLen::F32 => quote!(F32),
                        FLen::F64 => quote!(F64),
                        FLen::F128 => quote!(F128),
                    }
                }
            });
        let rs1_type = registers
            .iter()
            .find(|a| a.register == RegisterArg::Rs1)
            .map(|r| match r.r#type {
                RegType::Int => xlen.clone(),
                RegType::Float => {
                    let Some(flen) = float_length else {
                        panic!("Float arguments require a specified float length");
                    };
                    match flen {
                        FLen::F32 => quote!(F32),
                        FLen::F64 => quote!(F64),
                        FLen::F128 => quote!(F128),
                    }
                }
            });
        let rs2_type = registers
            .iter()
            .find(|a| a.register == RegisterArg::Rs2)
            .map(|r| match r.r#type {
                RegType::Int => xlen.clone(),
                RegType::Float => {
                    let Some(flen) = float_length else {
                        panic!("Float arguments require a specified float length");
                    };
                    match flen {
                        FLen::F32 => quote!(F32),
                        FLen::F64 => quote!(F64),
                        FLen::F128 => quote!(F128),
                    }
                }
            });
        let rs3_type = registers
            .iter()
            .find(|a| a.register == RegisterArg::Rs3)
            .map(|r| match r.r#type {
                RegType::Int => xlen.clone(),
                RegType::Float => {
                    let Some(flen) = float_length else {
                        panic!("Float arguments require a specified float length");
                    };
                    match flen {
                        FLen::F32 => quote!(F32),
                        FLen::F64 => quote!(F64),
                        FLen::F128 => quote!(F128),
                    }
                }
            });
        let _rs4_type = registers
            .iter()
            .find(|a| a.register == RegisterArg::Rs4)
            .map(|r| match r.r#type {
                RegType::Int => xlen.clone(),
                RegType::Float => {
                    let Some(flen) = float_length else {
                        panic!("Float arguments require a specified float length");
                    };
                    match flen {
                        FLen::F32 => quote!(F32),
                        FLen::F64 => quote!(F64),
                        FLen::F128 => quote!(F128),
                    }
                }
            });

        let rm_arg = if float_length.is_some() {
            quote!(rm: RoundingMode,)
        } else {
            quote!()
        };
        match inst_type {
            InstType::R => {
                assert!(
                    rd_type.is_some() && rs1_type.is_some() && rs2_type.is_some(),
                    "Please ensure types for rd, rs1, and rs2 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #rd_type,
                        rs1: &#rs1_type,
                        rs2: &#rs2_type,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::R4 => {
                assert!(
                    rd_type.is_some()
                        && rs1_type.is_some()
                        && rs2_type.is_some()
                        && rs3_type.is_some(),
                    "Please ensure types for rd, rs1, rs2 and rs3 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #rd_type,
                        rs1: &#rs1_type,
                        rs2: &#rs2_type,
                        rs3: &#rs3_type,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::RMem => {
                assert!(
                    rd_type.is_some() && rs1_type.is_some() && rs2_type.is_some(),
                    "Please ensure types for rd, rs1 and rs2 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        mut mem: MemoryWindow,
                        rd: &mut #rd_type,
                        rs1: &#rs1_type,
                        rs2: &#rs2_type,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::I => {
                assert!(
                    rd_type.is_some() && rs1_type.is_some(),
                    "Please ensure types for rd and rs1 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #rd_type,
                        rs1: &#rs1_type,
                        imm: i32,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::IMem => {
                assert!(
                    rd_type.is_some() && rs1_type.is_some(),
                    "Please ensure types for rd and rs1 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        mut mem: MemoryWindow,
                        rd: &mut #rd_type,
                        rs1: &#rs1_type,
                        imm: i32,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::S => {
                assert!(
                    rs1_type.is_some() && rs2_type.is_some(),
                    "Please ensure types for rs1 and rs2 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rs1: &#rs1_type,
                        rs2: &#rs2_type,
                        imm: i32,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::SMem => {
                assert!(
                    rs1_type.is_some() && rs2_type.is_some(),
                    "Please ensure types for rs1 and rs2 are specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        mut mem: MemoryWindow,
                        rs1: &#rs1_type,
                        rs2: &#rs2_type,
                        imm: i32,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::U => {
                assert!(
                    rd_type.is_some(),
                    "Please ensure a type for rd is specified"
                );
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #rd_type,
                        imm: i32,
                        #rm_arg
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
        }
    }

    quote!(#(#impls)*).into()
}
