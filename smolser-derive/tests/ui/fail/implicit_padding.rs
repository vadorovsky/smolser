use smolser_derive::Pod;

#[derive(Pod)]
#[repr(C)]
struct ImplicitPadding {
    a: u8,
    b: u16,
}

fn main() {}
