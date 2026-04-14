       IDENTIFICATION DIVISION.
       PROGRAM-ID. REPORT-GENERATOR.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
       SELECT REPORT-FILE ASSIGN TO
           "report.txt"
           ORGANIZATION IS LINE SEQUENTIAL.

       DATA DIVISION.
       FILE SECTION.
       FD  REPORT-FILE.
       01  REPORT-RECORD          PIC X(80).

       WORKING-STORAGE SECTION.
       01 WS-REPORT-HEADER.
           05 WS-TITLE            PIC X(40)
               VALUE "TRANSACTION REPORT".
           05 WS-DATE             PIC X(10)
               VALUE "04-13-2026".
       01 WS-COLUMN-HEADERS       PIC X(60)
           VALUE "Account     Amount      Type    Balance".
       01 WS-TRANSACTION-LINE.
           05 WS-ACCT-NUM         PIC 9(8).
           05 FILLER              PIC X VALUE SPACE.
           05 WS-TRANS-AMT        PIC -(9)9.99.
           05 FILLER              PIC X VALUE SPACE.
           05 WS-TRANS-TYPE       PIC X(7).
           05 FILLER              PIC X VALUE SPACE.
           05 WS-BALANCE-AMT      PIC -(9)9.99.
       01 WS-FOOTER-TOTAL         PIC X(60).
       01 WS-GRAND-TOTAL          PIC S9(12)V99 COMP-3 VALUE 0.
       01 WS-RECORD-COUNT         PIC 9(4) COMP VALUE 0.

       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           PERFORM GENERATE-REPORT.
           STOP RUN.

       GENERATE-REPORT.
           OPEN OUTPUT REPORT-FILE.

           PERFORM WRITE-HEADER.
           PERFORM WRITE-COLUMN-HEADERS.
           PERFORM WRITE-TRANSACTION-LINES.
           PERFORM WRITE-FOOTER.

           CLOSE REPORT-FILE.
           PERFORM DISPLAY-SUCCESS.

       WRITE-HEADER.
           MOVE SPACES TO REPORT-RECORD.
           STRING WS-TITLE DELIMITED BY SIZE
               " - " DELIMITED BY SIZE
               WS-DATE DELIMITED BY SIZE
               INTO REPORT-RECORD.
           WRITE REPORT-RECORD.

           MOVE SPACES TO REPORT-RECORD.
           MOVE "=========================================="
               TO REPORT-RECORD.
           WRITE REPORT-RECORD.

       WRITE-COLUMN-HEADERS.
           MOVE WS-COLUMN-HEADERS TO REPORT-RECORD.
           WRITE REPORT-RECORD.

           MOVE "=========================================="
               TO REPORT-RECORD.
           WRITE REPORT-RECORD.

       WRITE-TRANSACTION-LINES.
           PERFORM 5 TIMES
               PERFORM WRITE-SINGLE-TRANSACTION
               ADD 1 TO WS-RECORD-COUNT
           END-PERFORM.

       WRITE-SINGLE-TRANSACTION.
           MOVE 12345001 TO WS-ACCT-NUM.
           MOVE 50000.00 TO WS-TRANS-AMT.
           MOVE "CREDIT" TO WS-TRANS-TYPE.
           MOVE 150000.00 TO WS-BALANCE-AMT.
           ADD WS-TRANS-AMT TO WS-GRAND-TOTAL.

           MOVE WS-TRANSACTION-LINE TO REPORT-RECORD.
           WRITE REPORT-RECORD.

       WRITE-FOOTER.
           MOVE "=========================================="
               TO REPORT-RECORD.
           WRITE REPORT-RECORD.

           STRING "Total Transactions: " DELIMITED BY SIZE
               WS-RECORD-COUNT DELIMITED BY SIZE
               INTO WS-FOOTER-TOTAL.

           MOVE WS-FOOTER-TOTAL TO REPORT-RECORD.
           WRITE REPORT-RECORD.

           STRING "Grand Total: $"
               FUNCTION NUMVAL-C("$250000.00")
               DELIMITED BY SIZE
               INTO WS-FOOTER-TOTAL.

           MOVE WS-FOOTER-TOTAL TO REPORT-RECORD.
           WRITE REPORT-RECORD.

       DISPLAY-SUCCESS.
           DISPLAY "Report generated: report.txt".
