use std::path::PathBuf;
use std::{env, fs};
use verilator::gen::Verilator;
extern crate verilator;
use verilator::module::ModuleGenerator;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let _ = fs::remove_dir_all(&out_dir);
    t!(fs::create_dir_all(&out_dir));

    // Generate CPP shim from RUST
    let mut module = ModuleGenerator::default();
    module.generate("src/main.rs");

    // Generate CPP from Verilog
    let _verilator = Verilator::default()
        .with_coverage(true)
        .with_trace(true)
        .file("rtl/async_fifo.sv")
        .file(out_dir.join("async_fifo.cpp"))
        .build("async_fifo");
}
