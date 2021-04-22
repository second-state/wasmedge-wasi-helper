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

    pub fn get_bytes_from_caller() -> Result<Vec<u8>, &'static str> {
        // Get data path from the caller
        let path = match std::env::var("SSVM_DATA_TO_CALLEE") {
            Err(_) => Err("No data found from caller. Please check the SSVM_DATA_TO_CALLEE is set"),
            Ok(val) => Ok(val),
        };

        // Read the data from the given path.
        match std::fs::read(&path.unwrap()) {
            Err(err) => {
                eprintln!("Failed to open file. Due to: {:?}", err);
                return Err("Cannot retrieve data from caller");
            },
            Ok(buf) => Ok(buf),
        }
    }

    pub fn get_string_from_caller() -> Result<String, &'static str> {
        // Get data path from the caller
        let path = match std::env::var("SSVM_DATA_TO_CALLEE") {
            Err(_) => Err("No data found from caller. Please check the SSVM_DATA_TO_CALLEE is set"),
            Ok(val) => Ok(val),
        };

        // Read the data from the given path.
        match std::fs::read(&path.unwrap()) {
            Err(err) => {
                eprintln!("Failed to open file. Due to: {:?}", err);
                return Err("Cannot retrieve data from caller");
            },
            Ok(buf) => Ok(String::from_utf8_lossy(&buf).to_string()),
        }
    }
}
