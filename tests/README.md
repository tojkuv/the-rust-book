```rust
$ cargo new tests --lib
     Created library `tests` project
$ cd tests
```

```rust
#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }    
}
```

```rust
cargo test
 Compiling tests v0.1.0 (file:/home/machine/Documents/GitHub/
    the-rust-book/tests)
  Finished test [unoptimized + debuginfo] target(s) in 0.57s
   Running unittests (target/debug/deps/tests-4b45657c46fb4d25)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 
    0 filtered out; finished in 0.00s

    Doc-tests tests

running 0 tests

test results: ok. 0 passed; 0 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s
```

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        // the `assert_eq!` macro will fail if the parameter values are
        // not equal, calling the `panic!` macro and printing the
        // parameters to stdout
        assert_eq!(2 + 2, 4);
    }    
}
```

```rust
$ cargo test
   Compiling tests v0.1.0 (
       /home/machine/Documents/GitHub/the-rust-book/tests)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (
         target/debug/deps/tests-4b45657c46fb4d25)

running 1 test
test tests::expliraation ... ok

test result: ok 1 passed; 0 faiiled 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s

    Doc-tests tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s
```

```rust
...

running 2 tests
test tests::expliration ... ok
test tests::another ... FAILLED

failures:

---- tests::another stdout ----
thread 'tests::another' paniched at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display
    a backtrace

failurers:
    tests:another

test result: FAILED, 1 passed; 1 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

```rust
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s

    Doc-tests tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s
```

```rust
...
running 3 tests
test tests::larger_can_hold_smaller ... ok
test tests::it_add_two ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::it_add_two stdout ----
thread 'tests::it_add_two' paniched at ' assertion failed:
    `(left == right)`
    left: `4`,
    right: `5`', src/lib.rs:23:9
note: run  with `RUST_BACKTRACE=1` environment variable to display a
    backtrace

failures:
    tests::it_add_two

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured;
    0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```


