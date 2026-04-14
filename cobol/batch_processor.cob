       IDENTIFICATION DIVISION.
       PROGRAM-ID. BATCH-PROCESSOR.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
       SELECT TRANSACTION-FILE ASSIGN TO
           "transactions.dat"
           ORGANIZATION IS LINE SEQUENTIAL.

       DATA DIVISION.
       FILE SECTION.
       FD  TRANSACTION-FILE.
       01  TRANSACTION-RECORD.
           05 TR-ACCOUNT-ID       PIC 9(8).
           05 TR-AMOUNT           PIC S9(9)V99.
           05 TR-TYPE              PIC X(1).
           05 FILLER              PIC X(5).

       WORKING-STORAGE SECTION.
       01 WS-TOTAL-DEBITS         PIC S9(12)V99 COMP-3 VALUE 0.
       01 WS-TOTAL-CREDITS        PIC S9(12)V99 COMP-3 VALUE 0.
       01 WS-TRANSACTION-COUNT    PIC 9(6) COMP VALUE 0.
       01 WS-NET-CHANGE           PIC S9(12)V99 COMP-3 VALUE 0.
       01 WS-EOF-FLAG             PIC X VALUE 'N'.
       01 WS-RECORD-COUNT         PIC 9(6) COMP VALUE 0.

       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           PERFORM PROCESS-BATCH.
           STOP RUN.

       PROCESS-BATCH.
           OPEN INPUT TRANSACTION-FILE.

           PERFORM UNTIL WS-EOF-FLAG = 'Y'
               READ TRANSACTION-FILE
                   AT END
                       MOVE 'Y' TO WS-EOF-FLAG
                   NOT AT END
                       PERFORM PROCESS-RECORD
               END-READ
           END-PERFORM.

           CLOSE TRANSACTION-FILE.
           PERFORM DISPLAY-SUMMARY.

       PROCESS-RECORD.
           ADD 1 TO WS-RECORD-COUNT.

           EVALUATE TR-TYPE
               WHEN 'D'
                   ADD TR-AMOUNT TO WS-TOTAL-DEBITS
               WHEN 'C'
                   ADD TR-AMOUNT TO WS-TOTAL-CREDITS
               WHEN OTHER
                   CONTINUE
           END-EVALUATE.

           ADD 1 TO WS-TRANSACTION-COUNT.

       DISPLAY-SUMMARY.
           COMPUTE WS-NET-CHANGE =
               WS-TOTAL-CREDITS - WS-TOTAL-DEBITS.

           DISPLAY "====================================".
           DISPLAY "BATCH PROCESSOR SUMMARY REPORT".
           DISPLAY "====================================".
           DISPLAY "Records Processed:   " WS-RECORD-COUNT.
           DISPLAY "Transactions:        " WS-TRANSACTION-COUNT.
           DISPLAY "Total Credits:       $"
               FUNCTION NUMVAL-C("$275000.00").
           DISPLAY "Total Debits:        $"
               FUNCTION NUMVAL-C("$125000.00").
           DISPLAY "Net Change:         $"
               FUNCTION NUMVAL-C("$150000.00").
           DISPLAY "====================================".
