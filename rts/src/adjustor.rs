/*! A little bit of background...

An adjustor thunk is a dynamically allocated code snippet that allows
Haskell closures to be viewed as C function pointers.

Stable pointers provide a way for the outside world to get access to,
and evaluate, Haskell heap objects, with the RTS providing a small
range of ops for doing so. So, assuming we've got a stable pointer in
our hand in C, we can jump into the Haskell world and evaluate a callback
procedure, say. This works OK in some cases where callbacks are used, but
does require the external code to know about stable pointers and how to deal
with them. We'd like to hide the Haskell-nature of a callback and have it
be invoked just like any other C function pointer.

Enter adjustor thunks. An adjustor thunk is a little piece of code
that's generated on-the-fly (one per Haskell closure being exported)
that, when entered using some 'universal' calling convention (e.g., the
C calling convention on platform X), pushes an implicit stable pointer
(to the Haskell callback) before calling another (static) C function stub
which takes care of entering the Haskell code via its stable pointer.

An adjustor thunk is allocated on the C heap, and is called from within
Haskell just before handing out the function pointer to the Haskell (IO)
action. User code should never have to invoke it explicitly.

An adjustor thunk differs from a C function pointer in one respect: when
the code is through with it, it has to be freed in order to release Haskell
and C resources. Failure to do so will result in memory leaks on both the C and
Haskell side.
*/

use crate::prelude::*;
pub(crate) use internal::initAdjustors;
pub use internal::{createAdjustor, freeHaskellFunctionPtr};

// TODO(rust): Choose path based on architecture. E.g.
//   #[cfg_attr(target_arch = "x86", path  = "adjustor/native_i386.rs")]
//   #[cfg_attr(all(windows, target_env = "gnu"), path  = "adjustor/native_amd64_mingw.rs")]
//   #[cfg_attr(all(windows, not(target_env = "gnu")), path  = "adjustor/native_amd64.rs")]
#[cfg_attr(true, path = "adjustor/libffi_adjustor.rs")]
mod internal;

pub(crate) mod adjustor_pool;

unsafe fn totalArgumentSize(mut typeString: *const c_char) -> i32 {
    let mut sz = 0;

    while *typeString != 0 {
        let t = *typeString as u8;
        typeString = typeString.offset(1);

        // On 32-bit platforms, Double and Int64 occupy two words.
        if cfg!(target_pointer_width = "32") && matches!(t, b'd' | b'l' | b'L') {
            sz += 2;
        } else {
            // Everything else is one word.
            sz += 1;
        }
    }

    sz
}
