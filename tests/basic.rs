use nova::newtype;
use std::ops::Deref;

#[test]
fn ahhh() {
    #[newtype(copy)]
    pub type Hello = u8;

    let wow = Hello(32);
    assert_eq!(wow.into_inner(), 32);
}

#[test]
fn ahhh2() {
    #[newtype(borrow = "str")]
    pub type Hello = String;

    let wow = Hello("wew lad".into());
    assert_eq!(wow.deref(), "wew lad");
    assert_eq!(wow.into_inner(), "wew lad");
}