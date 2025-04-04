# I2C Scanner

A simple `no_std` scanner for I2C buses.

## Features

- [x] `no_std` support
- [x] blocking impl
- [x] `async` impl (via `async` feature)
- [x] Scan all addresses on an I2C bus
- [x] Check a specific address on an I2C bus
- [ ] Scan/check at different speeds
- [x] 7-bit Tests
- [ ] 10-bit Tests

## Development
* Run blocking tests: `$ cargo test`
* Run async tests: `$ cargo test -F async`

## License

Your choice:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
