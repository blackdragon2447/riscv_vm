use crate::hart::csr_address::{CsrAddress, CsrType};

#[test]
fn cycle_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC00).get_type(),
        CsrType::StandardRO
    )
}

#[test]
fn time_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC01).get_type(),
        CsrType::StandardRO
    )
}

#[test]
fn instret_csr_type() {
    assert_eq!(
        <u16 as Into<CsrAddress>>::into(0xC02).get_type(),
        CsrType::StandardRO
    )
}
