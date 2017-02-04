StringBuf
----

A nice and concise string concatenation library for the Rust language.

Preparation
===

Put this in your `Cargo.toml`:

```
[dependencies]
stringbuf = "0.1.0"
```

Usage
====

Instantiation:

``` rust
extern crate stringbuf;

use stringbuf::StringBuf;

fn main() {
    let mut sb = StringBuf::from("Hello");
    sb = sb + " " + "world!";
    sb += " AddAssign works,";

    assert_eq!("Hello world! AddAssign works, as does appending by method.",
               *sb.append(" as does appending by method."));
}
```
