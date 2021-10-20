#[doc(inline)]
pub use nova_macro::newtype;

pub trait NewType {
    type Inner;
}
