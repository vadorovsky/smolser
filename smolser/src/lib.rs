#![no_std]

use core::mem;

#[cfg(feature = "derive")]
pub use smolser_derive::Pod;

#[derive(Debug)]
pub enum SmolserError {
    SizeMismatch { expected: usize, len: usize },
    AlignmentMismatch { align: usize, ptr: usize },
}

#[inline]
fn validate_bytes<T: Sized>(s: &[u8]) -> Result<(), SmolserError> {
    let expected = mem::size_of::<T>();
    let len = s.len();
    if len != expected {
        return Err(SmolserError::SizeMismatch { expected, len });
    }

    #[cfg(not(target_os = "solana"))]
    {
        let align = mem::align_of::<T>();
        let ptr = s.as_ptr() as usize;
        if ptr % align != 0 {
            return Err(SmolserError::AlignmentMismatch { align, ptr });
        }
    }

    Ok(())
}

pub unsafe trait Pod: Sized {
    #[inline(always)]
    fn from_bytes(s: &[u8]) -> Result<&Self, SmolserError> {
        validate_bytes::<Self>(s)?;
        // SAFETY: We checked the size and alignment.
        Ok(unsafe { Self::from_bytes_unchecked(s) })
    }

    #[inline(always)]
    unsafe fn from_bytes_unchecked(s: &[u8]) -> &Self {
        unsafe { &*(s.as_ptr() as *const Self) }
    }

    #[inline(always)]
    fn from_bytes_mut(s: &mut [u8]) -> Result<&mut Self, SmolserError> {
        validate_bytes::<Self>(s)?;
        // SAFETY: We checked the size and alignment.
        Ok(unsafe { Self::from_bytes_mut_unchecked(s) })
    }

    #[inline(always)]
    unsafe fn from_bytes_mut_unchecked(s: &mut [u8]) -> &mut Self {
        unsafe { &mut *(s.as_mut_ptr() as *mut Self) }
    }
}

// SAFETY: Primitive types.
unsafe impl Pod for () {}
unsafe impl Pod for char {}
unsafe impl Pod for u8 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for usize {}
unsafe impl Pod for isize {}
unsafe impl Pod for u128 {}
unsafe impl Pod for i128 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

// SAFETY: These types have the same memory layout as T.
unsafe impl<T: Pod> Pod for core::cell::Cell<T> {}
unsafe impl<T: Pod> Pod for core::cell::UnsafeCell<T> {}

pub trait AccountExt {}
