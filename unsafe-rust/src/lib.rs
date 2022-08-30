#[cfg(test)]
mod tests {
    #[test]
    fn raw_pointers() {
        let mut num = 5;

        // the following lines create raw pointers from references using the `as` keyword
        let r1 = &num as *const i32;
        let r2 = &num as *mut i32;
    }
}

