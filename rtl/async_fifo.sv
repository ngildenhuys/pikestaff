`default_nettype none
/* verilator lint_off MULTIDRIVEN */

module async_fifo (
    input  logic       write_clk,  //input write clock
    input  logic       read_clk,   //input read clock
    input  logic       reset,      //input reset
    input  logic       write_en,   //input write enable
    input  logic       read_en,    //input read enable
    input  logic [7:0] data_in,    //input data
    output logic       mem_full,   //output memory full
    output logic       mem_empty,  //output memory empty
    output logic [7:0] out         //output data
);


  logic [7:0][7:0] mem;  //8 * 8 memory
  logic [2:0]      write_ptr;  //write pointer
  logic [2:0]      read_ptr;  //read pointer
  logic [3:0]      count;  //count

  //condition for mem_full and empty
  assign mem_full  = (count == 8);
  assign mem_empty = (count == 4'b0);

  /*Whenever the reset is 0 then make write pointer 0 otherwise if write enable
  * and memory is not full then write data into memory and increment write
  * pointer*/
  always_ff @(posedge write_clk, negedge reset) begin
    if (!reset) begin
      write_ptr <= 3'b0;  //reset pointer
    end else begin
      if (write_en == 1 && !mem_full) begin
        mem[write_ptr] <= data_in;  //data is written
        write_ptr      <= write_ptr + 1'b1;  //pointer increment
        out            <= 8'bxxxxxxxx;
      end else begin
      end
    end
  end

  /*Whenever the reset is 0 then make read pointer 0 otherwise if read enable
  * and memory is not empty then read data from memory and increment read
  * pointer*/
  always_ff @(posedge read_clk, negedge reset) begin
    if (!reset) begin
      read_ptr <= 3'b0;  //reset pointer
    end else begin
      if (read_en == 1 && !mem_empty) begin
        out      <= mem[read_ptr];  //data is read
        read_ptr <= read_ptr + 1'b1;  //pointer increment
      end else begin
      end
    end
  end

  /*increment count for write enable 1 and decrement count for read enable 1*/
  always_ff @(posedge write_clk, posedge read_clk, negedge reset) begin
    if (!reset) count <= 4'b0;  //reset counter
    else begin
      case ({
        write_en, read_en
      })
        2'b10:   count <= count + 1'b1;  //counter increment
        2'b01:   count <= count - 1'b1;  //counter increment
        default: count <= count;
      endcase
    end
  end
endmodule  //end of async FIFO
