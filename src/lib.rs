pub mod ssvm_wasi_helper {

    #[no_mangle]
    pub fn _initialize() {
        extern "C" {
            fn __wasm_call_ctors();
        }
        static mut INITED: bool = false;
        if unsafe { INITED } {
            return;
        }
        unsafe { __wasm_call_ctors() };
        unsafe { INITED = true };
    }

}
