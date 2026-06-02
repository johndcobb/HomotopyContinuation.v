/* verilator lint_off DECLFILENAME */

module fixed_add #(parameter WIDTH = 16)
  (input signed [WIDTH-1:0] i_A,
   input signed [WIDTH-1:0] i_B,
   output signed [WIDTH-1:0] o_Result);

  localparam signed [WIDTH-1:0] MAX_RAW = {1'b0, {(WIDTH-1){1'b1}}};
  localparam signed [WIDTH-1:0] MIN_RAW = {1'b1, {(WIDTH-1){1'b0}}};

  wire signed [WIDTH:0] w_A_Ext;
  wire signed [WIDTH:0] w_B_Ext;
  wire signed [WIDTH:0] w_Sum;
  wire signed [WIDTH:0] w_Max_Ext;
  wire signed [WIDTH:0] w_Min_Ext;

  assign w_A_Ext = {i_A[WIDTH-1], i_A};
  assign w_B_Ext = {i_B[WIDTH-1], i_B};
  assign w_Sum = w_A_Ext + w_B_Ext;
  assign w_Max_Ext = {MAX_RAW[WIDTH-1], MAX_RAW};
  assign w_Min_Ext = {MIN_RAW[WIDTH-1], MIN_RAW};

  assign o_Result = (w_Sum > w_Max_Ext) ? MAX_RAW :
                    (w_Sum < w_Min_Ext) ? MIN_RAW :
                    w_Sum[WIDTH-1:0];
endmodule

module fixed_sub #(parameter WIDTH = 16)
  (input signed [WIDTH-1:0] i_A,
   input signed [WIDTH-1:0] i_B,
   output signed [WIDTH-1:0] o_Result);

  localparam signed [WIDTH-1:0] MAX_RAW = {1'b0, {(WIDTH-1){1'b1}}};
  localparam signed [WIDTH-1:0] MIN_RAW = {1'b1, {(WIDTH-1){1'b0}}};

  wire signed [WIDTH:0] w_A_Ext;
  wire signed [WIDTH:0] w_B_Ext;
  wire signed [WIDTH:0] w_Diff;
  wire signed [WIDTH:0] w_Max_Ext;
  wire signed [WIDTH:0] w_Min_Ext;

  assign w_A_Ext = {i_A[WIDTH-1], i_A};
  assign w_B_Ext = {i_B[WIDTH-1], i_B};
  assign w_Diff = w_A_Ext - w_B_Ext;
  assign w_Max_Ext = {MAX_RAW[WIDTH-1], MAX_RAW};
  assign w_Min_Ext = {MIN_RAW[WIDTH-1], MIN_RAW};

  assign o_Result = (w_Diff > w_Max_Ext) ? MAX_RAW :
                    (w_Diff < w_Min_Ext) ? MIN_RAW :
                    w_Diff[WIDTH-1:0];
endmodule

module fixed_mul #(parameter WIDTH = 16, parameter FRAC_BITS = 10)
  (input signed [WIDTH-1:0] i_A,
   input signed [WIDTH-1:0] i_B,
   output signed [WIDTH-1:0] o_Result);

  localparam signed [WIDTH-1:0] MAX_RAW = {1'b0, {(WIDTH-1){1'b1}}};
  localparam signed [WIDTH-1:0] MIN_RAW = {1'b1, {(WIDTH-1){1'b0}}};

  wire signed [(2*WIDTH)-1:0] w_Product;
  wire signed [(2*WIDTH)-1:0] w_Product_Negated;
  wire [(2*WIDTH)-1:0] w_Abs_Product;
  wire [(2*WIDTH)-1:0] w_Rounded_Abs;
  wire signed [(2*WIDTH)-1:0] w_Shifted_Positive;
  wire signed [(2*WIDTH)-1:0] w_Shifted;
  wire signed [(2*WIDTH)-1:0] w_Max_Ext;
  wire signed [(2*WIDTH)-1:0] w_Min_Ext;

  assign w_Product = i_A * i_B;
  assign w_Product_Negated = -w_Product;
  assign w_Abs_Product = w_Product[(2*WIDTH)-1] ? w_Product_Negated : w_Product;
  assign w_Rounded_Abs = w_Abs_Product + (1 << (FRAC_BITS - 1));
  assign w_Shifted_Positive = w_Rounded_Abs >>> FRAC_BITS;
  assign w_Shifted = w_Product[(2*WIDTH)-1] ? -w_Shifted_Positive : w_Shifted_Positive;
  assign w_Max_Ext = {{WIDTH{MAX_RAW[WIDTH-1]}}, MAX_RAW};
  assign w_Min_Ext = {{WIDTH{MIN_RAW[WIDTH-1]}}, MIN_RAW};

  assign o_Result = (w_Shifted > w_Max_Ext) ? MAX_RAW :
                    (w_Shifted < w_Min_Ext) ? MIN_RAW :
                    w_Shifted[WIDTH-1:0];
endmodule

module complex_add #(parameter WIDTH = 16)
  (input signed [WIDTH-1:0] i_A_Real,
   input signed [WIDTH-1:0] i_A_Imag,
   input signed [WIDTH-1:0] i_B_Real,
   input signed [WIDTH-1:0] i_B_Imag,
   output signed [WIDTH-1:0] o_Result_Real,
   output signed [WIDTH-1:0] o_Result_Imag);

  fixed_add #(.WIDTH(WIDTH)) Real_Add
    (.i_A(i_A_Real), .i_B(i_B_Real), .o_Result(o_Result_Real));
  fixed_add #(.WIDTH(WIDTH)) Imag_Add
    (.i_A(i_A_Imag), .i_B(i_B_Imag), .o_Result(o_Result_Imag));
endmodule

module complex_sub #(parameter WIDTH = 16)
  (input signed [WIDTH-1:0] i_A_Real,
   input signed [WIDTH-1:0] i_A_Imag,
   input signed [WIDTH-1:0] i_B_Real,
   input signed [WIDTH-1:0] i_B_Imag,
   output signed [WIDTH-1:0] o_Result_Real,
   output signed [WIDTH-1:0] o_Result_Imag);

  fixed_sub #(.WIDTH(WIDTH)) Real_Sub
    (.i_A(i_A_Real), .i_B(i_B_Real), .o_Result(o_Result_Real));
  fixed_sub #(.WIDTH(WIDTH)) Imag_Sub
    (.i_A(i_A_Imag), .i_B(i_B_Imag), .o_Result(o_Result_Imag));
endmodule

module complex_mul #(parameter WIDTH = 16, parameter FRAC_BITS = 10)
  (input signed [WIDTH-1:0] i_A_Real,
   input signed [WIDTH-1:0] i_A_Imag,
   input signed [WIDTH-1:0] i_B_Real,
   input signed [WIDTH-1:0] i_B_Imag,
   output signed [WIDTH-1:0] o_Result_Real,
   output signed [WIDTH-1:0] o_Result_Imag);

  wire signed [WIDTH-1:0] w_AC;
  wire signed [WIDTH-1:0] w_BD;
  wire signed [WIDTH-1:0] w_AD;
  wire signed [WIDTH-1:0] w_BC;

  fixed_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_AC
    (.i_A(i_A_Real), .i_B(i_B_Real), .o_Result(w_AC));
  fixed_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_BD
    (.i_A(i_A_Imag), .i_B(i_B_Imag), .o_Result(w_BD));
  fixed_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_AD
    (.i_A(i_A_Real), .i_B(i_B_Imag), .o_Result(w_AD));
  fixed_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_BC
    (.i_A(i_A_Imag), .i_B(i_B_Real), .o_Result(w_BC));

  fixed_sub #(.WIDTH(WIDTH)) Real_Sub
    (.i_A(w_AC), .i_B(w_BD), .o_Result(o_Result_Real));
  fixed_add #(.WIDTH(WIDTH)) Imag_Add
    (.i_A(w_AD), .i_B(w_BC), .o_Result(o_Result_Imag));
endmodule

module horner_degree3 #(parameter WIDTH = 16, parameter FRAC_BITS = 10)
  (input signed [WIDTH-1:0] i_Z_Real,
   input signed [WIDTH-1:0] i_Z_Imag,
   input signed [WIDTH-1:0] i_C0_Real,
   input signed [WIDTH-1:0] i_C0_Imag,
   input signed [WIDTH-1:0] i_C1_Real,
   input signed [WIDTH-1:0] i_C1_Imag,
   input signed [WIDTH-1:0] i_C2_Real,
   input signed [WIDTH-1:0] i_C2_Imag,
   input signed [WIDTH-1:0] i_C3_Real,
   input signed [WIDTH-1:0] i_C3_Imag,
   output signed [WIDTH-1:0] o_Result_Real,
   output signed [WIDTH-1:0] o_Result_Imag);

  wire signed [WIDTH-1:0] w_C3Z_Real;
  wire signed [WIDTH-1:0] w_C3Z_Imag;
  wire signed [WIDTH-1:0] w_Step2_Real;
  wire signed [WIDTH-1:0] w_Step2_Imag;
  wire signed [WIDTH-1:0] w_Step2Z_Real;
  wire signed [WIDTH-1:0] w_Step2Z_Imag;
  wire signed [WIDTH-1:0] w_Step1_Real;
  wire signed [WIDTH-1:0] w_Step1_Imag;
  wire signed [WIDTH-1:0] w_Step1Z_Real;
  wire signed [WIDTH-1:0] w_Step1Z_Imag;

  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_C3_Z
    (.i_A_Real(i_C3_Real), .i_A_Imag(i_C3_Imag),
     .i_B_Real(i_Z_Real), .i_B_Imag(i_Z_Imag),
     .o_Result_Real(w_C3Z_Real), .o_Result_Imag(w_C3Z_Imag));
  complex_add #(.WIDTH(WIDTH)) Add_C2
    (.i_A_Real(w_C3Z_Real), .i_A_Imag(w_C3Z_Imag),
     .i_B_Real(i_C2_Real), .i_B_Imag(i_C2_Imag),
     .o_Result_Real(w_Step2_Real), .o_Result_Imag(w_Step2_Imag));
  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_Step2_Z
    (.i_A_Real(w_Step2_Real), .i_A_Imag(w_Step2_Imag),
     .i_B_Real(i_Z_Real), .i_B_Imag(i_Z_Imag),
     .o_Result_Real(w_Step2Z_Real), .o_Result_Imag(w_Step2Z_Imag));
  complex_add #(.WIDTH(WIDTH)) Add_C1
    (.i_A_Real(w_Step2Z_Real), .i_A_Imag(w_Step2Z_Imag),
     .i_B_Real(i_C1_Real), .i_B_Imag(i_C1_Imag),
     .o_Result_Real(w_Step1_Real), .o_Result_Imag(w_Step1_Imag));
  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_Step1_Z
    (.i_A_Real(w_Step1_Real), .i_A_Imag(w_Step1_Imag),
     .i_B_Real(i_Z_Real), .i_B_Imag(i_Z_Imag),
     .o_Result_Real(w_Step1Z_Real), .o_Result_Imag(w_Step1Z_Imag));
  complex_add #(.WIDTH(WIDTH)) Add_C0
    (.i_A_Real(w_Step1Z_Real), .i_A_Imag(w_Step1Z_Imag),
     .i_B_Real(i_C0_Real), .i_B_Imag(i_C0_Imag),
     .o_Result_Real(o_Result_Real), .o_Result_Imag(o_Result_Imag));
endmodule

module newton_step #(parameter WIDTH = 16, parameter FRAC_BITS = 10)
  (input signed [WIDTH-1:0] i_Point_Real,
   input signed [WIDTH-1:0] i_Point_Imag,
   input signed [WIDTH-1:0] i_H_Real,
   input signed [WIDTH-1:0] i_H_Imag,
   input signed [WIDTH-1:0] i_Derivative_Real,
   input signed [WIDTH-1:0] i_Derivative_Imag,
   input signed [WIDTH-1:0] i_Inverse_Guess_Real,
   input signed [WIDTH-1:0] i_Inverse_Guess_Imag,
   output signed [WIDTH-1:0] o_Inverse_Next_Real,
   output signed [WIDTH-1:0] o_Inverse_Next_Imag,
   output signed [WIDTH-1:0] o_Point_Next_Real,
   output signed [WIDTH-1:0] o_Point_Next_Imag);

  localparam signed [WIDTH-1:0] TWO_RAW = (2 <<< FRAC_BITS);
  localparam signed [WIDTH-1:0] ZERO_RAW = 0;

  wire signed [WIDTH-1:0] w_Inv_Derivative_Real;
  wire signed [WIDTH-1:0] w_Inv_Derivative_Imag;
  wire signed [WIDTH-1:0] w_Correction_Real;
  wire signed [WIDTH-1:0] w_Correction_Imag;
  wire signed [WIDTH-1:0] w_H_Times_Inv_Real;
  wire signed [WIDTH-1:0] w_H_Times_Inv_Imag;

  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_Inv_Derivative
    (.i_A_Real(i_Inverse_Guess_Real), .i_A_Imag(i_Inverse_Guess_Imag),
     .i_B_Real(i_Derivative_Real), .i_B_Imag(i_Derivative_Imag),
     .o_Result_Real(w_Inv_Derivative_Real), .o_Result_Imag(w_Inv_Derivative_Imag));
  complex_sub #(.WIDTH(WIDTH)) Sub_From_Two
    (.i_A_Real(TWO_RAW), .i_A_Imag(ZERO_RAW),
     .i_B_Real(w_Inv_Derivative_Real), .i_B_Imag(w_Inv_Derivative_Imag),
     .o_Result_Real(w_Correction_Real), .o_Result_Imag(w_Correction_Imag));
  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_Inv_Correction
    (.i_A_Real(i_Inverse_Guess_Real), .i_A_Imag(i_Inverse_Guess_Imag),
     .i_B_Real(w_Correction_Real), .i_B_Imag(w_Correction_Imag),
     .o_Result_Real(o_Inverse_Next_Real), .o_Result_Imag(o_Inverse_Next_Imag));
  complex_mul #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Mul_H_Inv
    (.i_A_Real(i_H_Real), .i_A_Imag(i_H_Imag),
     .i_B_Real(o_Inverse_Next_Real), .i_B_Imag(o_Inverse_Next_Imag),
     .o_Result_Real(w_H_Times_Inv_Real), .o_Result_Imag(w_H_Times_Inv_Imag));
  complex_sub #(.WIDTH(WIDTH)) Sub_Correction
    (.i_A_Real(i_Point_Real), .i_A_Imag(i_Point_Imag),
     .i_B_Real(w_H_Times_Inv_Real), .i_B_Imag(w_H_Times_Inv_Imag),
     .o_Result_Real(o_Point_Next_Real), .o_Result_Imag(o_Point_Next_Imag));
endmodule

/* verilator lint_on DECLFILENAME */
