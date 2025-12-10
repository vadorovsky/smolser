use smolser_derive::Pod;

#[derive(Pod)]
#[repr(C)]
struct ExplicitPadding {
    head: u8,
    _pad: [u8; 1],
    tail: u16,
}

fn main() {}
