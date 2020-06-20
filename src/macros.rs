



/**
 SEE https://blog.datalust.co/rust-at-datalust-how-we-organize-a-complex-rust-codebase/
 
Allow a block of `unsafe` code with a reason.

The macro will expand to an `unsafe` block.

We #![deny(unsafe_code)] in the crate root and #[allow(unsafe_code)] in the 
unsafe_block! so that forgetting to use the macro and just writing unsafe { ... } 
results in a compile error. This is really just to enforce a convention to make our 
unsafe code more auditable. In usage, unsafe blocks look something like this:

let mut read_into = unsafe_block!(
    "Our writers don't read from the uninitialized buffer" => {
        buf.bytes_mut()
    }
);

*/
macro_rules! unsafe_block {
    ($reason:tt => $body:expr) => {{
        #[allow(unsafe_code)]
        let r = unsafe { $body };
        r
    }};
}