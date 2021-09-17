pub mod wasmedge_wasi_helper {

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
        let path = match std::env::var("WASMEDGE_DATA_TO_CALLEE") {
            Err(_) => Err("No data found from caller. Please check the WASMEDGE_DATA_TO_CALLEE is set"),
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
        let path = match std::env::var("WASMEDGE_DATA_TO_CALLEE") {
            Err(_) => Err("No data found from caller. Please check the WASMEDGE_DATA_TO_CALLEE is set"),
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

    pub fn send_string_to_caller(s: &str) -> std::io::Result<()> {
        // Get data path from the caller
        let path = match std::env::var("WASMEDGE_DATA_FROM_CALLEE") {
            Err(_) => Err("Cannot find path from caller. Please check the WASMEDGE_DATA_FROM_CALLEE is set"),
            Ok(val) => Ok(val),
        };

        // Read the data from the given path.
        std::fs::write(&path.unwrap(), s)?;
        Ok(())
    }

    pub fn send_bytes_to_caller(bytes: &Vec<u8>) -> std::io::Result<()> {
        // Get data path from the caller
        let path = match std::env::var("WASMEDGE_DATA_FROM_CALLEE") {
            Err(_) => Err("Cannot find path from caller. Please check the WASMEDGE_DATA_FROM_CALLEE is set"),
            Ok(val) => Ok(val),
        };

        // Read the data from the given path.
        std::fs::write(&path.unwrap(), bytes)?;
        Ok(())
    }
}
