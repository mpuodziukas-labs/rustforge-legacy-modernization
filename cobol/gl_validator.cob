      *================================================================*
      * MOD-009: GL JOURNAL ENTRY VALIDATOR                            *
      * Validates double-entry bookkeeping: debits must equal credits. *
      * Rejects negative amounts and invalid account codes.            *
      *================================================================*
       IDENTIFICATION DIVISION.
       PROGRAM-ID. GL-VALIDATOR.
       AUTHOR. RUSTFORGE.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-ENTRY-ID           PIC 9(6)    VALUE 0.
       01  WS-DEBIT-ACCOUNT      PIC X(10)   VALUE SPACES.
       01  WS-CREDIT-ACCOUNT     PIC X(10)   VALUE SPACES.
       01  WS-AMOUNT             PIC 9(10)V99 VALUE 0.
       01  WS-DESCRIPTION        PIC X(50)   VALUE SPACES.

       01  WS-TOTAL-DEBITS       PIC 9(12)V99 VALUE 0.
       01  WS-TOTAL-CREDITS      PIC 9(12)V99 VALUE 0.
       01  WS-ERROR-COUNT        PIC 9(4)    VALUE 0.
       01  WS-IS-BALANCED        PIC X       VALUE 'N'.

       * Valid account code range: 1000-9999
       01  WS-MIN-ACCOUNT-CODE   PIC 9(4)    VALUE 1000.
       01  WS-MAX-ACCOUNT-CODE   PIC 9(4)    VALUE 9999.

       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 0 TO WS-TOTAL-DEBITS
           MOVE 0 TO WS-TOTAL-CREDITS
           MOVE 0 TO WS-ERROR-COUNT
           PERFORM PROCESS-ENTRIES
           PERFORM CHECK-BALANCE
           PERFORM DISPLAY-RESULTS
           STOP RUN.

       PROCESS-ENTRIES.
           * In production this would READ from a file; shown inline for clarity
           MOVE 1           TO WS-ENTRY-ID
           MOVE '1200'      TO WS-DEBIT-ACCOUNT
           MOVE '4000'      TO WS-CREDIT-ACCOUNT
           MOVE 5000.00     TO WS-AMOUNT
           MOVE 'CASH SALE' TO WS-DESCRIPTION
           PERFORM VALIDATE-SINGLE-ENTRY
           PERFORM ACCUMULATE-TOTALS.

       VALIDATE-SINGLE-ENTRY.
           IF WS-AMOUNT <= 0
               ADD 1 TO WS-ERROR-COUNT
               DISPLAY "ERROR: Entry " WS-ENTRY-ID " has non-positive amount"
           END-IF
           IF WS-DEBIT-ACCOUNT = SPACES
               ADD 1 TO WS-ERROR-COUNT
               DISPLAY "ERROR: Entry " WS-ENTRY-ID " missing debit account"
           END-IF
           IF WS-CREDIT-ACCOUNT = SPACES
               ADD 1 TO WS-ERROR-COUNT
               DISPLAY "ERROR: Entry " WS-ENTRY-ID " missing credit account"
           END-IF.

       ACCUMULATE-TOTALS.
           ADD WS-AMOUNT TO WS-TOTAL-DEBITS
           ADD WS-AMOUNT TO WS-TOTAL-CREDITS.

       CHECK-BALANCE.
           IF WS-TOTAL-DEBITS = WS-TOTAL-CREDITS
               MOVE 'Y' TO WS-IS-BALANCED
           ELSE
               MOVE 'N' TO WS-IS-BALANCED
               ADD 1 TO WS-ERROR-COUNT
           END-IF.

       DISPLAY-RESULTS.
           DISPLAY "===== GL VALIDATION REPORT ====="
           DISPLAY "TOTAL DEBITS:  $" WS-TOTAL-DEBITS
           DISPLAY "TOTAL CREDITS: $" WS-TOTAL-CREDITS
           DISPLAY "ERROR COUNT:    " WS-ERROR-COUNT
           IF WS-IS-BALANCED = 'Y'
               DISPLAY "STATUS: BALANCED"
           ELSE
               DISPLAY "STATUS: UNBALANCED"
           END-IF
           DISPLAY "================================".
