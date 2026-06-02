module Homotopy_Core_TB();
    reg r_Clk = 1'b0;
    always #2 r_Clk <= !r_Clk;

    reg signed [15:0] r_Fixed_A;
    reg signed [15:0] r_Fixed_B;
    wire signed [15:0] w_Fixed_Add;
    wire signed [15:0] w_Fixed_Sub;
    wire signed [15:0] w_Fixed_Mul;

    reg signed [15:0] r_Complex_A_Real;
    reg signed [15:0] r_Complex_A_Imag;
    reg signed [15:0] r_Complex_B_Real;
    reg signed [15:0] r_Complex_B_Imag;
    wire signed [15:0] w_Complex_Add_Real;
    wire signed [15:0] w_Complex_Add_Imag;
    wire signed [15:0] w_Complex_Sub_Real;
    wire signed [15:0] w_Complex_Sub_Imag;
    wire signed [15:0] w_Complex_Mul_Real;
    wire signed [15:0] w_Complex_Mul_Imag;

    reg signed [15:0] r_Horner_Z_Real;
    reg signed [15:0] r_Horner_Z_Imag;
    reg signed [15:0] r_C0_Real;
    reg signed [15:0] r_C0_Imag;
    reg signed [15:0] r_C1_Real;
    reg signed [15:0] r_C1_Imag;
    reg signed [15:0] r_C2_Real;
    reg signed [15:0] r_C2_Imag;
    reg signed [15:0] r_C3_Real;
    reg signed [15:0] r_C3_Imag;
    wire signed [15:0] w_Horner_Real;
    wire signed [15:0] w_Horner_Imag;

    reg signed [15:0] r_Point_Real;
    reg signed [15:0] r_Point_Imag;
    reg signed [15:0] r_H_Real;
    reg signed [15:0] r_H_Imag;
    reg signed [15:0] r_Derivative_Real;
    reg signed [15:0] r_Derivative_Imag;
    reg signed [15:0] r_Inverse_Guess_Real;
    reg signed [15:0] r_Inverse_Guess_Imag;
    wire signed [15:0] w_Inverse_Next_Real;
    wire signed [15:0] w_Inverse_Next_Imag;
    wire signed [15:0] w_Point_Next_Real;
    wire signed [15:0] w_Point_Next_Imag;

    fixed_add Fixed_Add_Inst
      (.i_A(r_Fixed_A), .i_B(r_Fixed_B), .o_Result(w_Fixed_Add));
    fixed_sub Fixed_Sub_Inst
      (.i_A(r_Fixed_A), .i_B(r_Fixed_B), .o_Result(w_Fixed_Sub));
    fixed_mul Fixed_Mul_Inst
      (.i_A(r_Fixed_A), .i_B(r_Fixed_B), .o_Result(w_Fixed_Mul));

    complex_add Complex_Add_Inst
      (.i_A_Real(r_Complex_A_Real), .i_A_Imag(r_Complex_A_Imag),
       .i_B_Real(r_Complex_B_Real), .i_B_Imag(r_Complex_B_Imag),
       .o_Result_Real(w_Complex_Add_Real), .o_Result_Imag(w_Complex_Add_Imag));
    complex_sub Complex_Sub_Inst
      (.i_A_Real(r_Complex_A_Real), .i_A_Imag(r_Complex_A_Imag),
       .i_B_Real(r_Complex_B_Real), .i_B_Imag(r_Complex_B_Imag),
       .o_Result_Real(w_Complex_Sub_Real), .o_Result_Imag(w_Complex_Sub_Imag));
    complex_mul Complex_Mul_Inst
      (.i_A_Real(r_Complex_A_Real), .i_A_Imag(r_Complex_A_Imag),
       .i_B_Real(r_Complex_B_Real), .i_B_Imag(r_Complex_B_Imag),
       .o_Result_Real(w_Complex_Mul_Real), .o_Result_Imag(w_Complex_Mul_Imag));

    horner_degree3 Horner_Inst
      (.i_Z_Real(r_Horner_Z_Real), .i_Z_Imag(r_Horner_Z_Imag),
       .i_C0_Real(r_C0_Real), .i_C0_Imag(r_C0_Imag),
       .i_C1_Real(r_C1_Real), .i_C1_Imag(r_C1_Imag),
       .i_C2_Real(r_C2_Real), .i_C2_Imag(r_C2_Imag),
       .i_C3_Real(r_C3_Real), .i_C3_Imag(r_C3_Imag),
       .o_Result_Real(w_Horner_Real), .o_Result_Imag(w_Horner_Imag));

    newton_step dut
      (.i_Point_Real(r_Point_Real), .i_Point_Imag(r_Point_Imag),
       .i_H_Real(r_H_Real), .i_H_Imag(r_H_Imag),
       .i_Derivative_Real(r_Derivative_Real), .i_Derivative_Imag(r_Derivative_Imag),
       .i_Inverse_Guess_Real(r_Inverse_Guess_Real), .i_Inverse_Guess_Imag(r_Inverse_Guess_Imag),
       .o_Inverse_Next_Real(w_Inverse_Next_Real), .o_Inverse_Next_Imag(w_Inverse_Next_Imag),
       .o_Point_Next_Real(w_Point_Next_Real), .o_Point_Next_Imag(w_Point_Next_Imag));

    integer r_File;
    integer r_Count;
    integer r_Line_Status;
    integer r_Vector_Count;
    string r_Line;

    integer idx;
    integer fixed_a;
    integer fixed_b;
    integer fixed_add_expected;
    integer fixed_sub_expected;
    integer fixed_mul_expected;
    integer complex_a_real;
    integer complex_a_imag;
    integer complex_b_real;
    integer complex_b_imag;
    integer complex_add_real_expected;
    integer complex_add_imag_expected;
    integer complex_sub_real_expected;
    integer complex_sub_imag_expected;
    integer complex_mul_real_expected;
    integer complex_mul_imag_expected;
    integer horner_z_real;
    integer horner_z_imag;
    integer c0_real;
    integer c0_imag;
    integer c1_real;
    integer c1_imag;
    integer c2_real;
    integer c2_imag;
    integer c3_real;
    integer c3_imag;
    integer horner_real_expected;
    integer horner_imag_expected;
    integer point_real;
    integer point_imag;
    integer h_real;
    integer h_imag;
    integer derivative_real;
    integer derivative_imag;
    integer inverse_guess_real;
    integer inverse_guess_imag;
    integer inverse_next_real_expected;
    integer inverse_next_imag_expected;
    integer point_next_real_expected;
    integer point_next_imag_expected;

    function signed [15:0] to_q10;
        input integer i_Value;
        begin
            if (i_Value < -32768 || i_Value > 32767) begin
                $display("Vector %0d has out-of-range Q6.10 value %0d", idx, i_Value);
                $fatal;
            end
            to_q10 = i_Value[15:0];
        end
    endfunction

    task assert_eq16;
        input signed [15:0] i_Actual;
        input integer i_Expected;
        input [255:0] i_Name;
        begin
            if ($signed({{16{i_Actual[15]}}, i_Actual}) != i_Expected) begin
                $display("Vector %0d failed %0s: expected %0d, got %0d",
                         idx, i_Name, i_Expected, $signed(i_Actual));
                $fatal;
            end
        end
    endtask

    task apply_and_check_vector;
        begin
            r_Fixed_A = to_q10(fixed_a);
            r_Fixed_B = to_q10(fixed_b);

            r_Complex_A_Real = to_q10(complex_a_real);
            r_Complex_A_Imag = to_q10(complex_a_imag);
            r_Complex_B_Real = to_q10(complex_b_real);
            r_Complex_B_Imag = to_q10(complex_b_imag);

            r_Horner_Z_Real = to_q10(horner_z_real);
            r_Horner_Z_Imag = to_q10(horner_z_imag);
            r_C0_Real = to_q10(c0_real);
            r_C0_Imag = to_q10(c0_imag);
            r_C1_Real = to_q10(c1_real);
            r_C1_Imag = to_q10(c1_imag);
            r_C2_Real = to_q10(c2_real);
            r_C2_Imag = to_q10(c2_imag);
            r_C3_Real = to_q10(c3_real);
            r_C3_Imag = to_q10(c3_imag);

            r_Point_Real = to_q10(point_real);
            r_Point_Imag = to_q10(point_imag);
            r_H_Real = to_q10(h_real);
            r_H_Imag = to_q10(h_imag);
            r_Derivative_Real = to_q10(derivative_real);
            r_Derivative_Imag = to_q10(derivative_imag);
            r_Inverse_Guess_Real = to_q10(inverse_guess_real);
            r_Inverse_Guess_Imag = to_q10(inverse_guess_imag);

            #1;

            assert_eq16(w_Fixed_Add, fixed_add_expected, "fixed_add");
            assert_eq16(w_Fixed_Sub, fixed_sub_expected, "fixed_sub");
            assert_eq16(w_Fixed_Mul, fixed_mul_expected, "fixed_mul");

            assert_eq16(w_Complex_Add_Real, complex_add_real_expected, "complex_add.real");
            assert_eq16(w_Complex_Add_Imag, complex_add_imag_expected, "complex_add.imag");
            assert_eq16(w_Complex_Sub_Real, complex_sub_real_expected, "complex_sub.real");
            assert_eq16(w_Complex_Sub_Imag, complex_sub_imag_expected, "complex_sub.imag");
            assert_eq16(w_Complex_Mul_Real, complex_mul_real_expected, "complex_mul.real");
            assert_eq16(w_Complex_Mul_Imag, complex_mul_imag_expected, "complex_mul.imag");

            assert_eq16(w_Horner_Real, horner_real_expected, "horner.real");
            assert_eq16(w_Horner_Imag, horner_imag_expected, "horner.imag");

            assert_eq16(w_Inverse_Next_Real, inverse_next_real_expected, "newton.inverse.real");
            assert_eq16(w_Inverse_Next_Imag, inverse_next_imag_expected, "newton.inverse.imag");
            assert_eq16(w_Point_Next_Real, point_next_real_expected, "newton.point.real");
            assert_eq16(w_Point_Next_Imag, point_next_imag_expected, "newton.point.imag");
        end
    endtask

    initial begin
        $dumpfile("homotopy_core.fst"); $dumpvars;
        r_Vector_Count = 0;
        r_File = $fopen("Verilog/sim/vectors/homotopy_core_vectors.mem", "r");
        if (r_File == 0) begin
            $display("Could not open Verilog/sim/vectors/homotopy_core_vectors.mem");
            $fatal;
        end

        while (!$feof(r_File)) begin
            r_Line_Status = $fgets(r_Line, r_File);
            if (r_Line_Status == 0) begin
                r_Count = -1;
            end
            else begin
                r_Count = $sscanf(r_Line,
                "%d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d\n",
                idx,
                fixed_a, fixed_b, fixed_add_expected, fixed_sub_expected, fixed_mul_expected,
                complex_a_real, complex_a_imag, complex_b_real, complex_b_imag,
                complex_add_real_expected, complex_add_imag_expected,
                complex_sub_real_expected, complex_sub_imag_expected,
                complex_mul_real_expected, complex_mul_imag_expected,
                horner_z_real, horner_z_imag,
                c0_real, c0_imag, c1_real, c1_imag, c2_real, c2_imag, c3_real, c3_imag,
                horner_real_expected, horner_imag_expected,
                point_real, point_imag, h_real, h_imag,
                derivative_real, derivative_imag,
                inverse_guess_real, inverse_guess_imag,
                inverse_next_real_expected, inverse_next_imag_expected,
                point_next_real_expected, point_next_imag_expected);
            end

            if (r_Count == 40) begin
                apply_and_check_vector();
                r_Vector_Count = r_Vector_Count + 1;
            end
            else if (r_Count == 0) begin
            end
            else if (r_Count != -1) begin
                $display("Malformed vector file line, parsed %0d fields", r_Count);
                $fatal;
            end
        end

        if (r_Vector_Count == 0) begin
            $display("No vectors were checked");
            $fatal;
        end

        $display("Test Complete: checked %0d homotopy core vectors", r_Vector_Count);
        $finish();
    end
endmodule
