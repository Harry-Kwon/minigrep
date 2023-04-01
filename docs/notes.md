# Collecting cli arguments
- `std::env::args()` return an iterator of over `String`
- `Iterator.colect()` transforms iterator to a collection (vector)

# References and slices
```rs
let args: Vec<String> = vec!["foo", "bar"];
let query = &args[1];  // immutable reference to element of the vector
```