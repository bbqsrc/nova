use nova::newtype;
use std::{borrow::Cow, ops::Deref};

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

#[test]
fn ahhh3() {
    #[newtype(new, borrow = "str", display)]
    pub type Hello<'a> = Cow<'a, str>;

    let wow = Hello("wew lad".into());
    assert_eq!(wow.deref(), "wew lad");
    assert_eq!(wow.into_inner(), "wew lad");
}

#[test]
fn ahhh4() {
    struct DeriveNothing {}

    #[newtype(custom_derives())]
    type Hello = DeriveNothing;

    assert!(true);// if it builds it works
}

#[test]
fn ahhh5() {
    #[derive(Debug)]
    struct JustDebug(u32);

    #[newtype(custom_derives(Debug))]
    type Hello = JustDebug;

    assert_eq!("JustDebug(42)", &format!("{:?}", JustDebug(42)));
    assert_eq!("Hello(JustDebug(42))", &format!("{:?}", Hello(JustDebug(42))));
}
