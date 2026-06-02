module Homotopy_Step_TB();
    reg r_Clk = 1'b0;
    reg r_Rst_L = 1'b0;
    reg r_Start = 1'b0;
    wire w_Busy;
    wire w_Done;

    always #2 r_Clk <= !r_Clk;

    reg signed [15:0] r_Z_Real;
    reg signed [15:0] r_Z_Imag;
    reg signed [15:0] r_H_C0_Real;
    reg signed [15:0] r_H_C0_Imag;
    reg signed [15:0] r_H_C1_Real;
    reg signed [15:0] r_H_C1_Imag;
    reg signed [15:0] r_H_C2_Real;
    reg signed [15:0] r_H_C2_Imag;
    reg signed [15:0] r_H_C3_Real;
    reg signed [15:0] r_H_C3_Imag;
    reg signed [15:0] r_DH_C0_Real;
    reg signed [15:0] r_DH_C0_Imag;
    reg signed [15:0] r_DH_C1_Real;
    reg signed [15:0] r_DH_C1_Imag;
    reg signed [15:0] r_DH_C2_Real;
    reg signed [15:0] r_DH_C2_Imag;
    reg signed [15:0] r_Inverse_Guess_Real;
    reg signed [15:0] r_Inverse_Guess_Imag;

    wire signed [15:0] w_H_Real;
    wire signed [15:0] w_H_Imag;
    wire signed [15:0] w_DH_Real;
    wire signed [15:0] w_DH_Imag;
    wire signed [15:0] w_Inverse_Next_Real;
    wire signed [15:0] w_Inverse_Next_Imag;
    wire signed [15:0] w_Z_Next_Real;
    wire signed [15:0] w_Z_Next_Imag;

    homotopy_step_cubic dut
      (.i_Clk(r_Clk),
       .i_Rst_L(r_Rst_L),
       .i_Start(r_Start),
       .o_Busy(w_Busy),
       .o_Done(w_Done),
       .i_Z_Real(r_Z_Real),
       .i_Z_Imag(r_Z_Imag),
       .i_H_C0_Real(r_H_C0_Real),
       .i_H_C0_Imag(r_H_C0_Imag),
       .i_H_C1_Real(r_H_C1_Real),
       .i_H_C1_Imag(r_H_C1_Imag),
       .i_H_C2_Real(r_H_C2_Real),
       .i_H_C2_Imag(r_H_C2_Imag),
       .i_H_C3_Real(r_H_C3_Real),
       .i_H_C3_Imag(r_H_C3_Imag),
       .i_DH_C0_Real(r_DH_C0_Real),
       .i_DH_C0_Imag(r_DH_C0_Imag),
       .i_DH_C1_Real(r_DH_C1_Real),
       .i_DH_C1_Imag(r_DH_C1_Imag),
       .i_DH_C2_Real(r_DH_C2_Real),
       .i_DH_C2_Imag(r_DH_C2_Imag),
       .i_Inverse_Guess_Real(r_Inverse_Guess_Real),
       .i_Inverse_Guess_Imag(r_Inverse_Guess_Imag),
       .o_H_Real(w_H_Real),
       .o_H_Imag(w_H_Imag),
       .o_DH_Real(w_DH_Real),
       .o_DH_Imag(w_DH_Imag),
       .o_Inverse_Next_Real(w_Inverse_Next_Real),
       .o_Inverse_Next_Imag(w_Inverse_Next_Imag),
       .o_Z_Next_Real(w_Z_Next_Real),
       .o_Z_Next_Imag(w_Z_Next_Imag));

    integer r_File;
    integer r_Count;
    integer r_Line_Status;
    integer r_Vector_Count;
    integer r_Timeout;
    string r_Line;

    integer idx;
    integer step;
    integer z_real;
    integer z_imag;
    integer h_c0_real;
    integer h_c0_imag;
    integer h_c1_real;
    integer h_c1_imag;
    integer h_c2_real;
    integer h_c2_imag;
    integer h_c3_real;
    integer h_c3_imag;
    integer dh_c0_real;
    integer dh_c0_imag;
    integer dh_c1_real;
    integer dh_c1_imag;
    integer dh_c2_real;
    integer dh_c2_imag;
    integer inverse_guess_real;
    integer inverse_guess_imag;
    integer h_real_expected;
    integer h_imag_expected;
    integer dh_real_expected;
    integer dh_imag_expected;
    integer inverse_next_real_expected;
    integer inverse_next_imag_expected;
    integer z_next_real_expected;
    integer z_next_imag_expected;

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
                $display("Vector %0d step %0d failed %0s: expected %0d, got %0d",
                         idx, step, i_Name, i_Expected, $signed(i_Actual));
                $fatal;
            end
        end
    endtask

    task apply_vector_inputs;
        begin
            r_Z_Real = to_q10(z_real);
            r_Z_Imag = to_q10(z_imag);
            r_H_C0_Real = to_q10(h_c0_real);
            r_H_C0_Imag = to_q10(h_c0_imag);
            r_H_C1_Real = to_q10(h_c1_real);
            r_H_C1_Imag = to_q10(h_c1_imag);
            r_H_C2_Real = to_q10(h_c2_real);
            r_H_C2_Imag = to_q10(h_c2_imag);
            r_H_C3_Real = to_q10(h_c3_real);
            r_H_C3_Imag = to_q10(h_c3_imag);
            r_DH_C0_Real = to_q10(dh_c0_real);
            r_DH_C0_Imag = to_q10(dh_c0_imag);
            r_DH_C1_Real = to_q10(dh_c1_real);
            r_DH_C1_Imag = to_q10(dh_c1_imag);
            r_DH_C2_Real = to_q10(dh_c2_real);
            r_DH_C2_Imag = to_q10(dh_c2_imag);
            r_Inverse_Guess_Real = to_q10(inverse_guess_real);
            r_Inverse_Guess_Imag = to_q10(inverse_guess_imag);
        end
    endtask

    task start_and_wait_for_done;
        begin
            @(posedge r_Clk);
            r_Start = 1'b1;
            @(posedge r_Clk);
            r_Start = 1'b0;

            if (!w_Busy) begin
                $display("Vector %0d step %0d did not assert o_Busy after i_Start", idx, step);
                $fatal;
            end

            r_Timeout = 0;
            while (!w_Done && r_Timeout < 16) begin
                @(posedge r_Clk);
                r_Timeout = r_Timeout + 1;
            end

            if (!w_Done) begin
                $display("Vector %0d step %0d timed out waiting for o_Done", idx, step);
                $fatal;
            end
        end
    endtask

    task check_outputs;
        begin
            assert_eq16(w_H_Real, h_real_expected, "H.real");
            assert_eq16(w_H_Imag, h_imag_expected, "H.imag");
            assert_eq16(w_DH_Real, dh_real_expected, "dH.real");
            assert_eq16(w_DH_Imag, dh_imag_expected, "dH.imag");
            assert_eq16(w_Inverse_Next_Real, inverse_next_real_expected, "inverse_next.real");
            assert_eq16(w_Inverse_Next_Imag, inverse_next_imag_expected, "inverse_next.imag");
            assert_eq16(w_Z_Next_Real, z_next_real_expected, "z_next.real");
            assert_eq16(w_Z_Next_Imag, z_next_imag_expected, "z_next.imag");
        end
    endtask

    initial begin
        $dumpfile("homotopy_step.fst"); $dumpvars;
        r_Vector_Count = 0;

        repeat(2) @(posedge r_Clk);
        r_Rst_L = 1'b1;

        r_File = $fopen("Verilog/sim/vectors/homotopy_step_vectors.mem", "r");
        if (r_File == 0) begin
            $display("Could not open Verilog/sim/vectors/homotopy_step_vectors.mem");
            $fatal;
        end

        while (!$feof(r_File)) begin
            r_Line_Status = $fgets(r_Line, r_File);
            if (r_Line_Status == 0) begin
                r_Count = -1;
            end
            else begin
                r_Count = $sscanf(r_Line,
                    "%d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d\n",
                    idx, step,
                    z_real, z_imag,
                    h_c0_real, h_c0_imag, h_c1_real, h_c1_imag,
                    h_c2_real, h_c2_imag, h_c3_real, h_c3_imag,
                    dh_c0_real, dh_c0_imag, dh_c1_real, dh_c1_imag,
                    dh_c2_real, dh_c2_imag,
                    inverse_guess_real, inverse_guess_imag,
                    h_real_expected, h_imag_expected,
                    dh_real_expected, dh_imag_expected,
                    inverse_next_real_expected, inverse_next_imag_expected,
                    z_next_real_expected, z_next_imag_expected);
            end

            if (r_Count == 28) begin
                apply_vector_inputs();
                start_and_wait_for_done();
                check_outputs();
                r_Vector_Count = r_Vector_Count + 1;
            end
            else if (r_Count == 0) begin
            end
            else if (r_Count != -1) begin
                $display("Malformed homotopy step vector line, parsed %0d fields", r_Count);
                $fatal;
            end
        end

        if (r_Vector_Count == 0) begin
            $display("No homotopy step vectors were checked");
            $fatal;
        end

        $display("Test Complete: checked %0d homotopy step vectors", r_Vector_Count);
        $finish();
    end
endmodule
