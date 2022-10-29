#[cfg(test)]
mod tests {
    #[test]
    fn raw_pointers() {
        let mut num = 5;

        // raw pointers can be created outside `unsafe` blocks. dereferencing raw pointers must happen inside `unsafe` blocks.
        // note: immutable and mutable raw pointers can create data races
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // since the address of these raw pointers is derived from valid references, the raw pointers are guaranteed to have
        // a valid reference.
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        // the `address` raw pointer is bounded directly to a memory address and thus is not guaranteed to be a valid pointer
        let address  = 0x012345usize;
        let _r = address as *const i32;
    }

    #[test]
    fn unsafe_functions() {
        unsafe fn dangerous() { }

        unsafe {
            dangerous();
        }

        use std::slice;

        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        // // building a slice from a random memory address is not guaranteed to have valid `i32` values
        // let address = 0x01234usize;
        // let s = address as *mut i32;
        // let values: &[i32] = unsafe { slice::from_raw_parts_mut(s, 10_000) };
        // println!("{:?}", values);
    }

    #[test]
    fn external_foreign_function_interfaces() {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    fn rust_foreign_function_interfaces() {
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }
    }

    static HELLO_WORLD: &str = "Hello, world!";
    static mut COUNTER: u32 = 0;

    #[test]
    fn static_variables() {
        println!("name is: {}", HELLO_WORLD);

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
