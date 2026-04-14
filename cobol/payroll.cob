      *================================================================*
      * MOD-008: PAYROLL CALCULATOR                                    *
      * Computes gross pay, federal tax, FICA, and net pay.            *
      * Overtime: 1.5x rate for hours > 40.                           *
      * Federal brackets (2024 Single): 10/12/22/24%.                 *
      *================================================================*
       IDENTIFICATION DIVISION.
       PROGRAM-ID. PAYROLL-CALC.
       AUTHOR. RUSTFORGE.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-EMPLOYEE-ID        PIC 9(6)    VALUE 0.
       01  WS-HOURS-WORKED       PIC 9(3)V99 VALUE 0.
       01  WS-HOURLY-RATE        PIC 9(4)V99 VALUE 0.
       01  WS-REGULAR-HOURS      PIC 9(3)V99 VALUE 0.
       01  WS-OVERTIME-HOURS     PIC 9(3)V99 VALUE 0.
       01  WS-REGULAR-PAY        PIC 9(8)V99 VALUE 0.
       01  WS-OVERTIME-PAY       PIC 9(8)V99 VALUE 0.
       01  WS-GROSS-PAY          PIC 9(8)V99 VALUE 0.
       01  WS-FEDERAL-TAX        PIC 9(8)V99 VALUE 0.
       01  WS-SOCIAL-SECURITY    PIC 9(8)V99 VALUE 0.
       01  WS-MEDICARE           PIC 9(8)V99 VALUE 0.
       01  WS-NET-PAY            PIC 9(8)V99 VALUE 0.

       * Federal tax bracket thresholds (annual, Single filer 2024)
       01  WS-BRACKET-1-MAX      PIC 9(7)V99 VALUE 11600.00.
       01  WS-BRACKET-2-MAX      PIC 9(7)V99 VALUE 47150.00.
       01  WS-BRACKET-3-MAX      PIC 9(7)V99 VALUE 100525.00.
       01  WS-BRACKET-4-MAX      PIC 9(7)V99 VALUE 191950.00.

       * FICA constants
       01  WS-SS-RATE            PIC V9(4)    VALUE .0620.
       01  WS-MEDICARE-RATE      PIC V9(4)    VALUE .0145.
       01  WS-SS-WAGE-BASE       PIC 9(7)V99 VALUE 168600.00.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 1001          TO WS-EMPLOYEE-ID
           MOVE 45.00         TO WS-HOURS-WORKED
           MOVE 25.00         TO WS-HOURLY-RATE
           PERFORM CALC-GROSS-PAY
           PERFORM CALC-FEDERAL-TAX
           PERFORM CALC-FICA
           PERFORM CALC-NET-PAY
           PERFORM DISPLAY-RESULTS
           STOP RUN.

       CALC-GROSS-PAY.
           IF WS-HOURS-WORKED > 40
               COMPUTE WS-REGULAR-HOURS = 40
               COMPUTE WS-OVERTIME-HOURS = WS-HOURS-WORKED - 40
           ELSE
               MOVE WS-HOURS-WORKED TO WS-REGULAR-HOURS
               MOVE 0 TO WS-OVERTIME-HOURS
           END-IF
           COMPUTE WS-REGULAR-PAY  = WS-REGULAR-HOURS  * WS-HOURLY-RATE
           COMPUTE WS-OVERTIME-PAY = WS-OVERTIME-HOURS * WS-HOURLY-RATE * 1.5
           COMPUTE WS-GROSS-PAY    = WS-REGULAR-PAY + WS-OVERTIME-PAY.

       CALC-FEDERAL-TAX.
           EVALUATE TRUE
               WHEN WS-GROSS-PAY <= WS-BRACKET-1-MAX
                   COMPUTE WS-FEDERAL-TAX = WS-GROSS-PAY * 0.10
               WHEN WS-GROSS-PAY <= WS-BRACKET-2-MAX
                   COMPUTE WS-FEDERAL-TAX =
                       1160.00 + (WS-GROSS-PAY - 11600.00) * 0.12
               WHEN WS-GROSS-PAY <= WS-BRACKET-3-MAX
                   COMPUTE WS-FEDERAL-TAX =
                       5426.00 + (WS-GROSS-PAY - 47150.00) * 0.22
               WHEN WS-GROSS-PAY <= WS-BRACKET-4-MAX
                   COMPUTE WS-FEDERAL-TAX =
                       17168.50 + (WS-GROSS-PAY - 100525.00) * 0.24
               WHEN OTHER
                   COMPUTE WS-FEDERAL-TAX =
                       40426.50 + (WS-GROSS-PAY - 191950.00) * 0.32
           END-EVALUATE.

       CALC-FICA.
           IF WS-GROSS-PAY > WS-SS-WAGE-BASE
               COMPUTE WS-SOCIAL-SECURITY = WS-SS-WAGE-BASE * WS-SS-RATE
           ELSE
               COMPUTE WS-SOCIAL-SECURITY = WS-GROSS-PAY * WS-SS-RATE
           END-IF
           COMPUTE WS-MEDICARE = WS-GROSS-PAY * WS-MEDICARE-RATE.

       CALC-NET-PAY.
           COMPUTE WS-NET-PAY = WS-GROSS-PAY
               - WS-FEDERAL-TAX
               - WS-SOCIAL-SECURITY
               - WS-MEDICARE.

       DISPLAY-RESULTS.
           DISPLAY "===== PAYROLL REPORT ====="
           DISPLAY "EMPLOYEE:     " WS-EMPLOYEE-ID
           DISPLAY "GROSS PAY:   $" WS-GROSS-PAY
           DISPLAY "FEDERAL TAX: $" WS-FEDERAL-TAX
           DISPLAY "SOC SEC:     $" WS-SOCIAL-SECURITY
           DISPLAY "MEDICARE:    $" WS-MEDICARE
           DISPLAY "NET PAY:     $" WS-NET-PAY
           DISPLAY "=========================".
