
module async_fifo_tb (
    input logic clk,
    input logic rst_l,
    // inputs
    input logic read_en,
    input logic write_en,
    input logic [7:0] write_data,
    // outputs
    output logic [7:0] read_data,
    output logic mem_full,
    output logic mem_empty
);

  // control of the read and srite clocks
  logic read_clk;
  logic write_clk;
  always_ff @(posedge clk, negedge rst_l) begin : clk_ff
    if(~rst_l) begin
      read_clk <= '0;
      write_clk <= '0;
    end else begin
      read_clk <= clk;
      write_clk <= clk;
    end
  end


  async_fifo DUT (
      .write_clk(write_clk),
      .read_clk(read_clk),
      .rst_l(rst_l),
      .write_en(write_en),
      .read_en(read_en),
      .write_data(write_data),
      .read_data(read_data),
      .mem_full(mem_full),
      .mem_empty(mem_empty)
  );

endmodule : async_fifo_tb
