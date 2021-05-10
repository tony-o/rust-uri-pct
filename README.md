# Percent Encoding str

This package provides a functional interface for encoding/decoding strs. It does its best to decode malformed strings (such as `"%2%20%" => " "`).

# usage

```rust
use uri-pct;

let x = uri-pct::encode("hello world!");
// x = "hello%20world%21"
let y = uri-pct::decode(x);
// y = "hello world!"
```
