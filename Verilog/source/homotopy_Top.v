module homotopy_Top
  (input i_Clk,
   input i_Switch_1,
   input i_Switch_2,
   input i_Switch_3,
   input i_Switch_4,
   output o_LED_1,
   output o_LED_2,
   output o_LED_3,
   output o_LED_4);

  reg signed [15:0] r_A = 16'sd512;
  reg signed [15:0] r_B = 16'sd1024;
  wire signed [15:0] w_Product;

  always @(posedge i_Clk)
  begin
    r_A <= i_Switch_1 ? 16'sd1536 : 16'sd512;
    r_B <= i_Switch_2 ? -16'sd1024 : 16'sd2048;
  end

  fixed_mul Product_Inst
    (.i_A(r_A),
     .i_B(r_B),
     .o_Result(w_Product));

  assign o_LED_1 = w_Product[10];
  assign o_LED_2 = w_Product[11];
  assign o_LED_3 = i_Switch_3;
  assign o_LED_4 = i_Switch_4;
endmodule
