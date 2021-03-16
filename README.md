isin
====
An `ISIN` type for working with validated International Security Identifiers (ISINs) as defined in
[ISO 6166](https://www.iso.org/standard/78502.html).

The checksum calculation uses a table-driven algorithm to minimize overhead _vs._ a direct translation of the formula
definition.
 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
isin = "0.1"
```


## Example

```rust
use isin::ISIN;

fn main() -> Result<(), String> {
    let isin = ISIN::parse_strict("US0378331005")?;
    println!("ISIN: {}", isin.value()); // "US0378331005"
    println!("Country code: {}", isin.country_code()); // "US"
    println!("Security identifier: {}", isin.security_identifier()); // "037833100"
    println!("Check digit: {}", isin.check_digit()); // "5"
    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.