# ttimer
A simple and tiny timing crate.

## Example Usage
```rust
// time a function's execution
let result = timer!(some_function);

println!("Took {} ns", result.time.as_nanos());

// time execution and use output
fn some_function(a: i32, b: &str) -> Option<usize> {
    /* code */
}

let result = timer!(some_function, 12, "Some input string");

if let Some(value) = result.result {
    println!("Took {} ns to find {}", result.time.as_nanos(), value);
} else {
    println!("Took {} ns", result.time.as_nanos());
}
```