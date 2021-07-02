#![cfg_attr(mirai, allow(incomplete_features), feature(const_generics))]

#[macro_use]
extern crate mirai_annotations;
#[cfg(mirai)]
use mirai_annotations::{TagPropagationSet, TAG_PROPAGATION_ALL};

#[cfg(mirai)]
struct SecretTaintKind<const MASK: TagPropagationSet> {}
#[cfg(mirai)]
type SecretTaint = SecretTaintKind<TAG_PROPAGATION_ALL>;
#[cfg(not(mirai))]
type SecretTaint = (); // Ensures code compiles in non-MIRAI builds

fn get_abs_vec_val() -> Result<Vec<u8>, ()> {
    Ok(abstract_value!(vec![42, 100]))
}

pub fn test1() -> Result<(), ()> {
    let x : Result<Vec<u8>, ()> = get_abs_vec_val();
    add_tag!(&x, SecretTaint);
    let y = x?;
    verify!(does_not_have_tag!(&y, SecretTaint));
    Ok(())
}

pub fn test2() {
    let x : Result<i32, ()> = Ok(42);
    add_tag!(&x, SecretTaint);
    let y = x.or(Err(()));
    verify!(does_not_have_tag!(&y, SecretTaint));
}
