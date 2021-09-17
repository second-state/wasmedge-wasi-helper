# WASMEDGE wasi helper

## Usage

In the Cargo.toml
```toml
[dependencies]
wasmedge-wasi-helper = "=0.2.0"
```

## To enable the wasi feature

In your wasi functions
```rs
use wasmedge_wasi_helper::wasmedge_wasi_helper::_initialize;
pub fn func1() {
  _initialize();
  // do something which is related to wasi environment variables, arguments, and preopens.
}
```


## To get string data from caller (with WASMEDGE-go support)

```rs
use wasmedge_wasi_helper::wasmedge_wasi_helper::get_string_from_caller;
pub fn func1() {
  let s = get_string_from_caller();
  // do something with the string `s`
}
```


## To get byte array data from caller (with WASMEDGE-go support)

```rs
use wasmedge_wasi_helper::wasmedge_wasi_helper::get_bytes_from_caller;
pub fn func1() {
  let bs = get_bytes_from_caller();
  // do something with the Vec<u8> `bs`
}
```


## To send string data to caller (with WASMEDGE-go support)

```rs
use wasmedge_wasi_helper::wasmedge_wasi_helper::send_string_to_caller;
pub fn func1() {
  let s = "hello";
  send_string_to_caller(s);
}
```


## To get byte array data from caller (with WASMEDGE-go support)

```rs
use wasmedge_wasi_helper::wasmedge_wasi_helper::send_bytes_to_caller;
pub fn func1() {
  let bs = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  send_bytes_to_caller(bs);
}
```
