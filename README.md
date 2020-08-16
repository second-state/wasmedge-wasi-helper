# SSVM wasi helper

## Usage

In the Cargo.toml
```toml
[dependencies]
ssvm-wasi-helper = "=0.1.1"
```

In your wasi functions
```rs
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
pub fn func1() {
  _initialize();
  // do something which is related to wasi environment variables, arguments, and preopens.
}
```
