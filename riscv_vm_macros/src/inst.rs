use proc_macro::TokenStream;
use proc_macro2::Literal;
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
    I,
    IMem,
    S,
    SMem,
    U,
}

#[derive(PartialEq, Eq)]
enum XLen {
    B32,
    B64,
    B128,
}

struct InstSyntax {
    name: Ident,
    _paren_token: token::Paren,
    inst_type: Ident,
    _for_token: Token![for],
    _bracket_token: token::Bracket,
    lengths: Vec<Literal>,
    _colon_token: Token![:],
    code: Block,
}

struct Inst {
    name: Ident,
    inst_type: InstType,
    code: Block,
    lengths: Vec<XLen>,
}

impl Parse for Inst {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            panic!("Invalid Instruction Signature")
        }
        let inst_type;
        let lengths;
        let syntax = InstSyntax {
            name: input.parse()?,
            _paren_token: parenthesized!(inst_type in input),
            inst_type: inst_type.parse()?,
            _for_token: input.parse()?,
            _bracket_token: bracketed!(lengths in input),
            lengths: Punctuated::<Literal, Token![,]>::parse_terminated(&lengths)?
                .into_iter()
                .collect(),
            _colon_token: input.parse()?,
            code: input.parse()?,
        };

        let inst_type = match syntax.inst_type.to_string().as_str() {
            "r" => InstType::R,
            "i" => InstType::I,
            "i_mem" => InstType::IMem,
            "s" => InstType::S,
            "s_mem" => InstType::SMem,
            "u" => InstType::U,
            _ => panic!("Invalid Instuction Type"),
        };

        let mut lengths = Vec::new();
        for l in syntax.lengths {
            match l.to_string().as_str() {
                "32" => lengths.push(XLen::B32),
                "64" => lengths.push(XLen::B64),
                "128" => lengths.push(XLen::B128),
                _ => panic!("Invalid XLen, expected 32, 64, or 128"),
            }
        }

        lengths.dedup();

        Ok(Self {
            name: syntax.name,
            inst_type,
            code: syntax.code,
            lengths,
        })
    }
}

pub(super) fn inst_internal(input: TokenStream) -> TokenStream {
    let Inst {
        name,
        inst_type,
        code,
        lengths,
    } = parse_macro_input!(input as Inst);

    let mut impls = Vec::new();
    let xlen32 = quote!(i32);
    let b32len = quote!(
        type ixlen = i32;
        type uxlen = u32;
        type iexlen = i64;
        type uexlen = u64;
        let xlen: usize = 32;
    );
    let xlen64 = quote!(i64);
    let b64len = quote!(
        type ixlen = i64;
        type uxlen = u64;
        type iexlen = i128;
        type uexlen = u128;
        let xlen: usize = 64;
    );

    for l in lengths {
        let (name, len_types, xlen) = match l {
            XLen::B32 => (format_ident!("{}_32", name), &b32len, &xlen32),
            XLen::B64 => (format_ident!("{}_64", name), &b64len, &xlen64),
            XLen::B128 => panic!("128 Bit instruction have yet to be implemented"),
        };
        match inst_type {
            InstType::R => {
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #xlen,
                        rs1: &#xlen,
                        rs2: &#xlen
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::I => {
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #xlen,
                        rs1: &#xlen,
                        imm: i32,
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::IMem => {
                impls.push(quote!(
                    pub(super) fn #name<const SIZE: usize>(
                        pc: Address,
                        mem: &mut Memory<SIZE>,
                        rd: &mut #xlen,
                        rs1: &#xlen,
                        imm: i32
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::S => {
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rs1: &#xlen,
                        rs2: &#xlen,
                        imm: i32
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::SMem => {
                impls.push(quote!(
                    pub(super) fn #name<const SIZE: usize>(
                        pc: Address,
                        mem: &mut Memory<SIZE>,
                        rs1: &#xlen,
                        rs2: &#xlen,
                        imm: i32
                    ) -> Result<ExecuteResult, ExecuteError> {
                        #len_types

                        #code
                    }
                ));
            }
            InstType::U => {
                impls.push(quote!(
                    pub(super) fn #name(
                        pc: Address,
                        rd: &mut #xlen,
                        imm: i32
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
