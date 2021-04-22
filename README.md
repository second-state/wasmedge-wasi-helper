# SSVM wasi helper

## Usage

In the Cargo.toml
```toml
[dependencies]
ssvm-wasi-helper = "=0.1.2"
```

## To enable the wasi feature

In your wasi functions
```rs
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
pub fn func1() {
  _initialize();
  // do something which is related to wasi environment variables, arguments, and preopens.
}
```


## To get string data from caller (with SSVM-go support)

```rs
use ssvm_wasi_helper::ssvm_wasi_helper::get_string_from_caller;
pub fn func1() {
  let s = get_string_from_caller();
  // do something with the string `s`
}
```


## To get byte array data from caller (with SSVM-go support)

```rs
use ssvm_wasi_helper::ssvm_wasi_helper::get_bytes_from_caller;
pub fn func1() {
  let bs = get_bytes_from_caller();
  // do something with the Vec<u8> `bs`
}
```
