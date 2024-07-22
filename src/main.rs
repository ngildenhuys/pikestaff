use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use verilated_module::module;

#[module(async_fifo_tb)]
pub struct AsyncFifoTb {
    #[port(clock)]
    pub clk: bool,
    #[port(reset)]
    pub rst_l: bool,
    // inputs
    #[port(input)]
    pub write_en: bool,
    #[port(input)]
    pub read_en: bool,
    #[port(input)]
    pub write_data: [bool; 8],
    // outputs
    #[port(output)]
    pub mem_full: bool,
    #[port(output)]
    pub mem_empty: bool,
    #[port(output)]
    pub read_data: [bool; 8],
}

pub fn run_testbench(seed: [u8; 32]) {
    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut tb = AsyncFifoTb::default();
    tb.eval();
    tb.eval();
    // tb.open_trace("counter.vcd", 99).unwrap();
    let mut clocks: u64 = 0;
    for _ in 0..1000 {
        if clocks == 0 {
            tb.reset_toggle();
        } else if clocks == 2 {
            tb.reset_toggle();
        }

        tb.clock_toggle();
        tb.eval();

        // change the write data;
        let rand_data = (rng.next_u32() & 0xff) as u8;
        // let write_data = (0..8).map(|v| bool::from((rand_data >> v) & 1)).collect();
        tb.set_write_data(rand_data);
        tb.set_read_en(((rng.next_u32() % 2) & 0x1) as u8);
        tb.set_write_en(((rng.next_u32() % 2) & 0x1) as u8);

        tb.clock_toggle();
        tb.eval();
        clocks += 1;
    }

    tb.finish();
}

fn main() {
    run_testbench([0; 32]);
}
