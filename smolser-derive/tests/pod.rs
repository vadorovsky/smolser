use smolser::Pod as PodTrait;
use smolser_derive::Pod;

#[derive(Pod)]
#[repr(C)]
struct Inner {
    a: u8,
    _pad: [u8; 1],
    b: u16,
}

#[derive(Pod)]
#[repr(C)]
struct Outer {
    value: u32,
    inner: Inner,
}

fn assert_pod<T: PodTrait>() {}

#[test]
fn derive_pod_for_structs() {
    assert_pod::<Inner>();
    assert_pod::<Outer>();
}
