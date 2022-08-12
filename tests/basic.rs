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

    #[newtype(derive())]
    type Hello = DeriveNothing;

    assert!(true);// if it builds it works
}

#[test]
fn ahhh5() {
    #[derive(Debug,Copy,Clone)]
    struct DebugCopyClone(u32);

    #[newtype(derive(Debug,Copy,Clone))]
    type Hello = DebugCopyClone;

    assert_eq!("DebugCopyClone(42)", &format!("{:?}", DebugCopyClone(42)));

    let hello = Hello(DebugCopyClone(42));
    assert_eq!("Hello(DebugCopyClone(42))", &format!("{:?}", hello));

    let goodbye = hello;
    assert_eq!("Hello(DebugCopyClone(42))", &format!("{:?}", hello));
    assert_eq!("Hello(DebugCopyClone(42))", &format!("{:?}", goodbye));



}
