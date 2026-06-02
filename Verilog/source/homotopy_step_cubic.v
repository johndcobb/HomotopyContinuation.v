module homotopy_step_cubic #(parameter WIDTH = 16, parameter FRAC_BITS = 10)
  (input i_Clk,
   input i_Rst_L,
   input i_Start,
   output reg o_Busy,
   output reg o_Done,

   input signed [WIDTH-1:0] i_Z_Real,
   input signed [WIDTH-1:0] i_Z_Imag,

   input signed [WIDTH-1:0] i_H_C0_Real,
   input signed [WIDTH-1:0] i_H_C0_Imag,
   input signed [WIDTH-1:0] i_H_C1_Real,
   input signed [WIDTH-1:0] i_H_C1_Imag,
   input signed [WIDTH-1:0] i_H_C2_Real,
   input signed [WIDTH-1:0] i_H_C2_Imag,
   input signed [WIDTH-1:0] i_H_C3_Real,
   input signed [WIDTH-1:0] i_H_C3_Imag,

   input signed [WIDTH-1:0] i_DH_C0_Real,
   input signed [WIDTH-1:0] i_DH_C0_Imag,
   input signed [WIDTH-1:0] i_DH_C1_Real,
   input signed [WIDTH-1:0] i_DH_C1_Imag,
   input signed [WIDTH-1:0] i_DH_C2_Real,
   input signed [WIDTH-1:0] i_DH_C2_Imag,

   input signed [WIDTH-1:0] i_Inverse_Guess_Real,
   input signed [WIDTH-1:0] i_Inverse_Guess_Imag,

   output reg signed [WIDTH-1:0] o_H_Real,
   output reg signed [WIDTH-1:0] o_H_Imag,
   output reg signed [WIDTH-1:0] o_DH_Real,
   output reg signed [WIDTH-1:0] o_DH_Imag,
   output reg signed [WIDTH-1:0] o_Inverse_Next_Real,
   output reg signed [WIDTH-1:0] o_Inverse_Next_Imag,
   output reg signed [WIDTH-1:0] o_Z_Next_Real,
   output reg signed [WIDTH-1:0] o_Z_Next_Imag);

  localparam IDLE = 2'd0;
  localparam EVAL = 2'd1;
  localparam CORRECT = 2'd2;
  localparam DONE = 2'd3;
  localparam signed [WIDTH-1:0] ZERO_RAW = 0;

  reg [1:0] r_State;

  reg signed [WIDTH-1:0] r_Z_Real;
  reg signed [WIDTH-1:0] r_Z_Imag;
  reg signed [WIDTH-1:0] r_H_C0_Real;
  reg signed [WIDTH-1:0] r_H_C0_Imag;
  reg signed [WIDTH-1:0] r_H_C1_Real;
  reg signed [WIDTH-1:0] r_H_C1_Imag;
  reg signed [WIDTH-1:0] r_H_C2_Real;
  reg signed [WIDTH-1:0] r_H_C2_Imag;
  reg signed [WIDTH-1:0] r_H_C3_Real;
  reg signed [WIDTH-1:0] r_H_C3_Imag;
  reg signed [WIDTH-1:0] r_DH_C0_Real;
  reg signed [WIDTH-1:0] r_DH_C0_Imag;
  reg signed [WIDTH-1:0] r_DH_C1_Real;
  reg signed [WIDTH-1:0] r_DH_C1_Imag;
  reg signed [WIDTH-1:0] r_DH_C2_Real;
  reg signed [WIDTH-1:0] r_DH_C2_Imag;
  reg signed [WIDTH-1:0] r_Inverse_Guess_Real;
  reg signed [WIDTH-1:0] r_Inverse_Guess_Imag;

  wire signed [WIDTH-1:0] w_H_Real;
  wire signed [WIDTH-1:0] w_H_Imag;
  wire signed [WIDTH-1:0] w_DH_Real;
  wire signed [WIDTH-1:0] w_DH_Imag;
  wire signed [WIDTH-1:0] w_Inverse_Next_Real;
  wire signed [WIDTH-1:0] w_Inverse_Next_Imag;
  wire signed [WIDTH-1:0] w_Z_Next_Real;
  wire signed [WIDTH-1:0] w_Z_Next_Imag;

  horner_degree3 #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) H_Eval
    (.i_Z_Real(r_Z_Real), .i_Z_Imag(r_Z_Imag),
     .i_C0_Real(r_H_C0_Real), .i_C0_Imag(r_H_C0_Imag),
     .i_C1_Real(r_H_C1_Real), .i_C1_Imag(r_H_C1_Imag),
     .i_C2_Real(r_H_C2_Real), .i_C2_Imag(r_H_C2_Imag),
     .i_C3_Real(r_H_C3_Real), .i_C3_Imag(r_H_C3_Imag),
     .o_Result_Real(w_H_Real), .o_Result_Imag(w_H_Imag));

  horner_degree3 #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) DH_Eval
    (.i_Z_Real(r_Z_Real), .i_Z_Imag(r_Z_Imag),
     .i_C0_Real(r_DH_C0_Real), .i_C0_Imag(r_DH_C0_Imag),
     .i_C1_Real(r_DH_C1_Real), .i_C1_Imag(r_DH_C1_Imag),
     .i_C2_Real(r_DH_C2_Real), .i_C2_Imag(r_DH_C2_Imag),
     .i_C3_Real(ZERO_RAW), .i_C3_Imag(ZERO_RAW),
     .o_Result_Real(w_DH_Real), .o_Result_Imag(w_DH_Imag));

  newton_step #(.WIDTH(WIDTH), .FRAC_BITS(FRAC_BITS)) Newton_Inst
    (.i_Point_Real(r_Z_Real), .i_Point_Imag(r_Z_Imag),
     .i_H_Real(o_H_Real), .i_H_Imag(o_H_Imag),
     .i_Derivative_Real(o_DH_Real), .i_Derivative_Imag(o_DH_Imag),
     .i_Inverse_Guess_Real(r_Inverse_Guess_Real),
     .i_Inverse_Guess_Imag(r_Inverse_Guess_Imag),
     .o_Inverse_Next_Real(w_Inverse_Next_Real),
     .o_Inverse_Next_Imag(w_Inverse_Next_Imag),
     .o_Point_Next_Real(w_Z_Next_Real),
     .o_Point_Next_Imag(w_Z_Next_Imag));

  always @(posedge i_Clk or negedge i_Rst_L)
  begin
    if (!i_Rst_L)
    begin
      r_State <= IDLE;
      o_Busy <= 1'b0;
      o_Done <= 1'b0;
      r_Z_Real <= 0;
      r_Z_Imag <= 0;
      r_H_C0_Real <= 0;
      r_H_C0_Imag <= 0;
      r_H_C1_Real <= 0;
      r_H_C1_Imag <= 0;
      r_H_C2_Real <= 0;
      r_H_C2_Imag <= 0;
      r_H_C3_Real <= 0;
      r_H_C3_Imag <= 0;
      r_DH_C0_Real <= 0;
      r_DH_C0_Imag <= 0;
      r_DH_C1_Real <= 0;
      r_DH_C1_Imag <= 0;
      r_DH_C2_Real <= 0;
      r_DH_C2_Imag <= 0;
      r_Inverse_Guess_Real <= 0;
      r_Inverse_Guess_Imag <= 0;
      o_H_Real <= 0;
      o_H_Imag <= 0;
      o_DH_Real <= 0;
      o_DH_Imag <= 0;
      o_Inverse_Next_Real <= 0;
      o_Inverse_Next_Imag <= 0;
      o_Z_Next_Real <= 0;
      o_Z_Next_Imag <= 0;
    end
    else
    begin
      o_Done <= 1'b0;

      case (r_State)
        IDLE:
        begin
          o_Busy <= 1'b0;
          if (i_Start)
          begin
            r_Z_Real <= i_Z_Real;
            r_Z_Imag <= i_Z_Imag;
            r_H_C0_Real <= i_H_C0_Real;
            r_H_C0_Imag <= i_H_C0_Imag;
            r_H_C1_Real <= i_H_C1_Real;
            r_H_C1_Imag <= i_H_C1_Imag;
            r_H_C2_Real <= i_H_C2_Real;
            r_H_C2_Imag <= i_H_C2_Imag;
            r_H_C3_Real <= i_H_C3_Real;
            r_H_C3_Imag <= i_H_C3_Imag;
            r_DH_C0_Real <= i_DH_C0_Real;
            r_DH_C0_Imag <= i_DH_C0_Imag;
            r_DH_C1_Real <= i_DH_C1_Real;
            r_DH_C1_Imag <= i_DH_C1_Imag;
            r_DH_C2_Real <= i_DH_C2_Real;
            r_DH_C2_Imag <= i_DH_C2_Imag;
            r_Inverse_Guess_Real <= i_Inverse_Guess_Real;
            r_Inverse_Guess_Imag <= i_Inverse_Guess_Imag;
            o_Busy <= 1'b1;
            r_State <= EVAL;
          end
        end

        EVAL:
        begin
          o_H_Real <= w_H_Real;
          o_H_Imag <= w_H_Imag;
          o_DH_Real <= w_DH_Real;
          o_DH_Imag <= w_DH_Imag;
          r_State <= CORRECT;
        end

        CORRECT:
        begin
          o_Inverse_Next_Real <= w_Inverse_Next_Real;
          o_Inverse_Next_Imag <= w_Inverse_Next_Imag;
          o_Z_Next_Real <= w_Z_Next_Real;
          o_Z_Next_Imag <= w_Z_Next_Imag;
          r_State <= DONE;
        end

        DONE:
        begin
          o_Busy <= 1'b0;
          o_Done <= 1'b1;
          r_State <= IDLE;
        end

        default:
        begin
          o_Busy <= 1'b0;
          r_State <= IDLE;
        end
      endcase
    end
  end
endmodule
