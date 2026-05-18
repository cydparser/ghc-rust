use core::ffi::{CStr, c_char};
use core::fmt;
use core::marker::PhantomData;
use core::{slice, str};

#[derive(Debug)]
enum Error {
    Utf8Error(str::Utf8Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Utf8Error(err) => err.fmt(f),
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Str0<'a> {
    ptr: *const c_char,
    _phantom: PhantomData<&'a str>,
}

unsafe impl Send for Str0<'static> {}
unsafe impl Sync for Str0<'static> {}

impl<'a> fmt::Debug for Str0<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Str0").field(&self.as_str()).finish()
    }
}

impl<'a> fmt::Display for Str0<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> Str0<'a> {
    pub const unsafe fn from_ptr(ptr: *const c_char) -> Result<Self, Error> {
        match unsafe { ptr_to_str(ptr) } {
            Ok(_) => Ok(Str0 {
                ptr,
                _phantom: PhantomData,
            }),
            Err(err) => Err(Error::Utf8Error(err)),
        }
    }

    pub fn from_ptr_with_str(ptr: *const c_char) -> Result<(Self, &'a str), Error> {
        match unsafe { ptr_to_str(ptr) } {
            Ok(s) => Ok((
                Str0 {
                    ptr,
                    _phantom: PhantomData,
                },
                s,
            )),
            Err(err) => Err(Error::Utf8Error(err)),
        }
    }

    pub const fn empty() -> Self {
        static NUL: c_char = 0;

        return Str0 {
            ptr: &raw const NUL,
            _phantom: PhantomData,
        };
    }

    pub fn write_lossy<'s, 'd>(src: &'s str, mut dst: &'d mut [u8]) -> Str0<'d> {
        use std::io::Write;

        let last_dst_index = dst.len() - 1;

        let amt = dst.write(src.as_bytes()).unwrap().min(dst.len() - 1);

        let nul_index = if amt == src.len() && amt <= last_dst_index {
            amt
        } else {
            match dst[0..last_dst_index].utf8_chunks().next() {
                Some(chunk) => chunk.valid().len(),
                // This will be rare: either Symbol’s value as variable is void: dst is empty or smaller than the first utf8 char.
                None => return Str0::empty(),
            }
        };

        dst[nul_index] = 0;

        Str0 {
            ptr: dst.as_ptr().cast(),
            _phantom: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr.cast::<u8>(), strlen(self.ptr));

            str::from_utf8_unchecked(bytes)
        }
    }

    pub fn as_c_str(&self) -> &CStr {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr.cast::<u8>(), strlen(self.ptr));

            CStr::from_bytes_with_nul_unchecked(bytes)
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { *self.ptr == 0 }
    }
}

pub const unsafe fn ptr_to_str<'a>(ptr: *const c_char) -> Result<&'a str, str::Utf8Error> {
    let bytes = unsafe {
        let len = strlen(ptr);
        slice::from_raw_parts(ptr.cast(), len)
    };

    str::from_utf8(bytes)
}

pub const unsafe fn strlen(ptr: *const c_char) -> usize {
    let mut end = ptr;

    while *end != 0 {
        end = end.add(1);
    }

    end.offset_from(ptr) as usize
}

pub struct String0(Vec<u8>);

impl fmt::Debug for String0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("String0")
            .field(&self.as_str0().as_str())
            .finish()
    }
}

impl fmt::Display for String0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str0().as_str().fmt(f)
    }
}

impl String0 {
    pub fn from_ptr(ptr: *const c_char) -> Result<Self, Error> {
        let len = unsafe { strlen(ptr) } + 1;
        let bytes = unsafe { slice::from_raw_parts(ptr.cast(), len) };

        match str::from_utf8(bytes) {
            Ok(_) => {
                let mut v = Vec::with_capacity(len);
                v.copy_from_slice(bytes);

                Ok(String0(v))
            }
            Err(err) => Err(Error::Utf8Error(err)),
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr().cast()
    }

    pub fn as_str0<'a>(&'a self) -> Str0<'a> {
        Str0 {
            ptr: self.as_ptr(),
            _phantom: PhantomData,
        }
    }
}
