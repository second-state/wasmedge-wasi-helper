# SSVM wasi helper

## Usage

In the Cargo.toml
```toml
[dependencies]
ssvm-wasi-helper = "=0.1.0"
```

In your wasi functions
```rs
pub fn func1() {
  ssvm-wasi-helper::_initialize();
  // do something
}
```
