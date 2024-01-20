use inst::inst_internal;
use proc_macro::TokenStream;

mod executor;
mod inst;

#[proc_macro]
/// Syntax:
/// `inst!($name($type) for [$lengths]: {$impl})`
///
/// $name:
///     The name of the instruction, gets expanded
///     with $length for the length it is generated for
///
/// $type:
///     The type of the instruction maps more or less onto
///     the riscv inst types, B and J are folded into S and U
///     respectively. For instructions that need access to
///     memory there are `_mem` types.
///     - r
///     - i
///     - i_mem
///     - s
///     - s_mem
///     - u
///
/// $lengths:
///     The lengths for which this instruction must be
///     implemented, may contain 32, 64, 128. 128 will panic
///     with a not implemented error.
///
/// $impl:
///     the implementation of the instruction, it has
///     access to any registers the instruction has
///     access to (rd as mut), the pc, and if it is
///     of type i_mem or s_mem to the memory,
///     it also has access to the types ixlen and uxlen
///     which are signed and unsigned integers of length xlen,
///     and iexlen and uexlen which are integers of double
///     xlen length and the xlen variable.
pub fn inst(input: TokenStream) -> TokenStream {
    inst_internal(input)
}
