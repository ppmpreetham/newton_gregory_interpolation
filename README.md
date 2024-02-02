# Newton-Gregory Interpolation

This is an implementation of Newton-Gregory interpolation method in Rust. Input your cool random numbers and then get a function for that.

## Usage

Here, we are inserting the random list of numbers [1, 2, 4, 8, 16, 31] from the famous Dividing a circle into areas question in math.

```rust
fn main() {
    let result = newton_gregory_interpolation(vec![1, 2, 4, 8, 16, 31]);
    println!("{}", result);
}
```

Note: The nC notation in the formula represents the binomial coefficient, which is used in the Newton-Gregory interpolation method