use smolser::Pod as PodTrait;
use smolser_derive::Pod;

#[derive(Pod)]
#[repr(C)]
struct Generic<T: PodTrait> {
    head: u8,
    value: T,
}

fn main() {}
