       IDENTIFICATION DIVISION.
       PROGRAM-ID. LOAN-CALCULATOR.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-PRINCIPAL           PIC 9(9)V99 COMP-3 VALUE 0.
       01 WS-ANNUAL-RATE         PIC 9V9(4) COMP-3 VALUE 0.
       01 WS-TERM-MONTHS         PIC 9(3) VALUE 0.
       01 WS-MONTHLY-RATE        PIC 9V9(8) COMP-3 VALUE 0.
       01 WS-MONTHLY-PAYMENT     PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-TOTAL-PAYMENT       PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-TOTAL-INTEREST      PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-POWER-FACTOR        PIC S9(9)V9(8) COMP-3 VALUE 0.
       01 WS-DENOM               PIC S9(9)V9(8) COMP-3 VALUE 0.
       01 WS-MONTH-CTR           PIC 9(3) VALUE 0.
       01 WS-BALANCE             PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-INTEREST-CHARGE     PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-PRINCIPAL-PAID      PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-FMT-PAYMENT         PIC -(9)9.99.
       01 WS-FMT-TOTAL           PIC -(9)9.99.
       01 WS-FMT-INTEREST        PIC -(9)9.99.
       01 WS-REPORT-DATE         PIC X(10) VALUE '04-13-2026'.

       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           MOVE 300000 TO WS-PRINCIPAL.
           MOVE 0.0600 TO WS-ANNUAL-RATE.
           MOVE 360 TO WS-TERM-MONTHS.

           PERFORM COMPUTE-MONTHLY-RATE.
           PERFORM COMPUTE-PAYMENT.
           PERFORM DISPLAY-REPORT.
           STOP RUN.

       COMPUTE-MONTHLY-RATE.
           COMPUTE WS-MONTHLY-RATE =
               WS-ANNUAL-RATE / 12 / 100.

       COMPUTE-PAYMENT.
      *    payment = P * r / (1 - (1 + r)^-n)
           COMPUTE WS-POWER-FACTOR =
               (1 + WS-MONTHLY-RATE) ** (-WS-TERM-MONTHS).
           COMPUTE WS-DENOM =
               1 - WS-POWER-FACTOR.
           COMPUTE WS-MONTHLY-PAYMENT =
               WS-PRINCIPAL * WS-MONTHLY-RATE / WS-DENOM.
           COMPUTE WS-TOTAL-PAYMENT =
               WS-MONTHLY-PAYMENT * WS-TERM-MONTHS.
           COMPUTE WS-TOTAL-INTEREST =
               WS-TOTAL-PAYMENT - WS-PRINCIPAL.

       DISPLAY-REPORT.
           MOVE WS-MONTHLY-PAYMENT TO WS-FMT-PAYMENT.
           MOVE WS-TOTAL-PAYMENT TO WS-FMT-TOTAL.
           MOVE WS-TOTAL-INTEREST TO WS-FMT-INTEREST.

           DISPLAY "====================================".
           DISPLAY "LOAN CALCULATOR REPORT".
           DISPLAY "Date: " WS-REPORT-DATE.
           DISPLAY "====================================".
           DISPLAY "Principal:         $"
               WS-PRINCIPAL.
           DISPLAY "Annual Rate:       "
               WS-ANNUAL-RATE "%".
           DISPLAY "Term (months):     "
               WS-TERM-MONTHS.
           DISPLAY "Monthly Rate:      "
               WS-MONTHLY-RATE.
           DISPLAY "------------------------------------".
           DISPLAY "Monthly Payment:   $"
               WS-FMT-PAYMENT.
           DISPLAY "Total Payment:     $"
               WS-FMT-TOTAL.
           DISPLAY "Total Interest:    $"
               WS-FMT-INTEREST.
           DISPLAY "====================================".
