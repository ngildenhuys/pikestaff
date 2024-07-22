use std::time::Duration;

use verilated_module::module;

#[module(async_fifo)]
pub struct AsyncFifo {
    #[port(input)]
    pub write_clk: bool, //input write clock
    #[port(input)]
    pub read_clk: bool, //input read clock
    #[port(reset)]
    pub reset: bool, //input reset
    #[port(input)]
    pub write_en: bool, //input write enable
    #[port(input)]
    pub read_en: bool, //input read enable
    #[port(input)]
    pub data_in: [bool; 8], //input data
    #[port(output)]
    pub mem_full: bool, //output memory full
    #[port(output)]
    pub mem_empty: bool, //output memory empty
    #[port(output)]
    pub out: [bool; 8], //output data
}

fn main() {
    let mut tb = AsyncFifo::default();
    tb.eval();
    tb.eval();

    // tb.open_trace("counter.vcd", 99).unwrap();

    let mut clocks: u64 = 0;
    for _ in 0..10 {
        if clocks == 0 {
            tb.reset_toggle();
        } else if clocks == 2 {
            tb.reset_toggle();
        }

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks));

        tb.clock_toggle();
        tb.eval();
        tb.trace_at(Duration::from_nanos(20 * clocks + 10));

        // println!("{}: count_o = {}", clocks, tb.count_o());

        clocks += 1;
    }
    tb.trace_at(Duration::from_nanos(20 * clocks));

    tb.finish();
}
