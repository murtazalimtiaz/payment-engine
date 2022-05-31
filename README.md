<div align="center">
  <h1>
    <code>Payments Engine</code>
  </h1>
  <strong>A simple toy payments engine</sup>
  
  <sub>Built with 🦀 <a href="https://www.rust-lang.org" target="_blank">Rust</a> and  lots of ❤️</sub>

[![Build and test](https://github.com/murtazalimtiaz/payments-engine/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/murtazalimtiaz/payments-engine/actions/workflows/build_and_test.yml) [![Security audit](https://github.com/murtazalimtiaz/payments-engine/actions/workflows/security_audit.yml/badge.svg?branch=master)](https://github.com/murtazalimtiaz/payments-engine/actions/workflows/security_audit.yml)

</div>

Payments Engine is a simple toy payments engine that reads a series of transactions from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the state of clients accounts as a CSV.

## Installation

```bash
git clone git@github.com:murtazalimtiaz/payments-engine.git
```

## Usage

```bash
cargo run -- transactions.csv > accounts.csv
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Roadmap

- [x] Read a series of transactions from a file
- [x] Update client accounts
- [x] Handle disputes
- [x] Handle resolutions
- [x] Handle chargebacks
- [x] Output the state of accounts
- [x] Implement meaningful error handling
- [x] Add integration tests
- [x] Add CI
- [ ] Implement test data generator

## License

[MIT](https://choosealicense.com/licenses/mit/)
