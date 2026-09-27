use std::ffi::c_void;
use std::mem::{self, MaybeUninit};
use std::ops::{BitAnd, BitOr, BitXor, Not};

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::GetCurrentProcess;

extern "C" {
    /// Defined in `guarded_read.c`. Copies `len` bytes from `src` to `dst`,
    /// returning 0 instead of crashing if `src` is not readable.
    fn libds3_guarded_read(src: *const c_void, dst: *mut c_void, len: usize) -> i32;
}

/// Lowest address that can ever be mapped on Windows: the first 64 KiB are
/// always reserved. Catches null pointers plus an offset without faulting.
const MIN_USER_ADDRESS: usize = 0x1_0000;
/// Highest user-mode address on x64 Windows. Anything above is kernel space
/// or non-canonical, and can never be read from user mode.
const MAX_USER_ADDRESS: usize = 0x7FFF_FFFE_FFFF;

/// Reads a `T` from `addr` without crashing if the memory isn't readable.
///
/// Addresses that can never be mapped are rejected upfront, as handling an
/// access violation is much more expensive than a successful read.
///
/// `T` must be valid for any bit pattern.
fn guarded_read<T>(addr: usize) -> Option<T> {
    let len = mem::size_of::<T>();
    let last = addr.checked_add(len.max(1) - 1)?;
    if addr < MIN_USER_ADDRESS || last > MAX_USER_ADDRESS {
        return None;
    }

    let mut value = MaybeUninit::<T>::uninit();
    // SAFETY: `value` has room for `len` bytes, and the C side catches access
    // violations on `addr` instead of letting them unwind.
    let ok = unsafe { libds3_guarded_read(addr as _, value.as_mut_ptr() as _, len) };

    // SAFETY: all `len` bytes were written, and `T` is valid for any bit pattern.
    (ok != 0).then(|| unsafe { value.assume_init() })
}

/// Wraps CheatEngine's concept of pointer with nested offsets. Evaluates,
/// if the evaluation does not fail, to a mutable pointer of type `T`.
///
/// At runtime, it evaluates the final address of the chain by reading the
/// base pointer, then recursively reading the next memory address in the
/// chain at an offset from there. For example,
///
/// ```
/// PointerChain::<T>::new(&[a, b, c, d, e])
/// ```
///
/// evaluates to
///
/// ```
/// *(*(*(*(*a + b) + c) + d) + e)
/// ```
///
/// This is useful for managing reverse engineered structures which are not
/// fully known.
#[derive(Clone, Debug)]
pub struct PointerChain<T> {
    proc: HANDLE,
    base: *mut T,
    offsets: Vec<usize>,
}
unsafe impl<T> Send for PointerChain<T> {}
unsafe impl<T> Sync for PointerChain<T> {}

impl<T> PointerChain<T> {
    /// Creates a new pointer chain given an array of addresses.
    pub fn new(chain: &[usize]) -> PointerChain<T> {
        let mut it = chain.iter();
        let base = *it.next().unwrap() as *mut T;
        PointerChain {
            proc: unsafe { GetCurrentProcess() },
            base,
            offsets: it.copied().collect(), // it.map(|x| *x).collect(),
        }
    }

    /// Safely evaluates the pointer chain.
    /// Relies on guarded reads instead of plain pointer dereferencing for
    /// crash safety. Returns `None` if the evaluation failed.
    pub fn eval(&self) -> Option<*mut T> {
        self.offsets
            .iter()
            .try_fold(self.base as usize, |addr, &offs| {
                guarded_read::<usize>(addr).map(|value| value.wrapping_add(offs))
            })
            .map(|addr| addr as *mut T)
    }

    /// Evaluates the pointer chain and attempts to read the datum.
    /// Returns `None` if either the evaluation or the read failed.
    pub fn read(&self) -> Option<T> {
        guarded_read(self.eval()? as usize)
    }

    /// Evaluates the pointer chain and attempts to write the datum.
    /// Returns `None` if either the evaluation or the write failed.
    ///
    /// Uses `WriteProcessMemory`, which also succeeds on read-only pages (e.g.
    /// code patches) by temporarily changing their protection.
    pub fn write(&self, mut value: T) -> Option<()> {
        let ptr = self.eval()?;
        unsafe {
            WriteProcessMemory(
                self.proc,
                ptr as _,
                &mut value as *mut _ as _,
                std::mem::size_of::<T>(),
                None,
            )
            .ok()
            .map(|_| ())
        }
    }

    pub fn cast<S>(&self) -> PointerChain<S> {
        PointerChain { proc: self.proc, base: self.base as *mut S, offsets: self.offsets.clone() }
    }
}

#[derive(Clone, Debug)]
pub struct Bitflag<T>(PointerChain<T>, T);

impl<T> Bitflag<T>
where
    T: BitXor<Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + Not<Output = T>
        + PartialEq
        + Copy,
{
    pub fn new(c: PointerChain<T>, mask: T) -> Self {
        Bitflag(c, mask)
    }

    pub fn toggle(&self) {
        if let Some(x) = self.0.read() {
            self.0.write(x ^ self.1);
        }
    }

    pub fn get(&self) -> Option<bool> {
        self.0.read().map(|x| (x & self.1) == self.1)
    }

    pub fn set(&self, flag: bool) {
        if let Some(x) = self.0.read() {
            self.0.write(if flag { x | self.1 } else { x & !self.1 });
        }
    }
}

#[macro_export]
macro_rules! pointer_chain {
    ($($e:expr),+) => { PointerChain::new(&[$($e,)*]) }
}

#[macro_export]
macro_rules! bitflag {
    ($b:expr; $($e:expr),+) => { Bitflag::new(PointerChain::new(&[$($e,)*]), $b) }
}

pub use bitflag;
pub use pointer_chain;

#[cfg(test)]
mod tests {
    use windows::Win32::System::Memory::{
        VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_NOACCESS,
    };

    use super::*;

    #[test]
    fn test_guarded_read_valid() {
        let value: u64 = 0x0123_4567_89AB_CDEF;
        assert_eq!(guarded_read::<u64>(&value as *const u64 as usize), Some(value));
    }

    #[test]
    fn test_guarded_read_unmappable() {
        assert_eq!(guarded_read::<u64>(0), None);
        assert_eq!(guarded_read::<u64>(0x80), None);
        assert_eq!(guarded_read::<u64>(MIN_USER_ADDRESS - 1), None);
        assert_eq!(guarded_read::<u64>(MAX_USER_ADDRESS - 4), None);
        assert_eq!(guarded_read::<u64>(0xFFFF_F800_0000_0000), None);
        assert_eq!(guarded_read::<u64>(usize::MAX - 2), None);
    }

    #[test]
    fn test_guarded_read_access_violation() {
        unsafe {
            // Not rejected upfront: these reads fault and must be caught.
            let reserved = VirtualAlloc(None, 0x1000, MEM_RESERVE, PAGE_NOACCESS);
            assert!(!reserved.is_null());
            assert_eq!(guarded_read::<u64>(reserved as usize), None);
            VirtualFree(reserved, 0, MEM_RELEASE).unwrap();

            let no_access = VirtualAlloc(None, 0x1000, MEM_RESERVE | MEM_COMMIT, PAGE_NOACCESS);
            assert!(!no_access.is_null());
            assert_eq!(guarded_read::<[u8; 16]>(no_access as usize), None);
            VirtualFree(no_access, 0, MEM_RELEASE).unwrap();
        }
    }

    #[test]
    fn test_pointer_chain() {
        let value: u32 = 0xDEAD_BEEF;
        let inner: [usize; 2] = [0, &value as *const u32 as usize - 8];
        let outer: usize = inner.as_ptr() as usize;

        // *(*(&outer) + 8) + 8
        let chain = PointerChain::<u32>::new(&[&outer as *const usize as usize, 8, 8]);
        assert_eq!(chain.read(), Some(value));

        // Second deref reads inner[0], a null pointer, and the third reads from it.
        let broken = PointerChain::<u32>::new(&[&outer as *const usize as usize, 0, 0, 8]);
        assert_eq!(broken.eval(), None);
        assert_eq!(broken.read(), None);
    }
}
