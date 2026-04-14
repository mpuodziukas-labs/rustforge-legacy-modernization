       IDENTIFICATION DIVISION.
       PROGRAM-ID. INVENTORY-VALUATION.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.

       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-ITEM-CODE           PIC X(10) VALUE SPACES.
       01 WS-LOT-TABLE.
          05 WS-LOT OCCURS 100 TIMES.
             10 WS-LOT-QTY      PIC 9(6) VALUE 0.
             10 WS-LOT-COST     PIC 9(6)V99 COMP-3 VALUE 0.
       01 WS-LOT-COUNT           PIC 9(3) VALUE 0.
       01 WS-TOTAL-COGS          PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-UNITS-SOLD          PIC 9(6) VALUE 0.
       01 WS-SELL-QTY            PIC 9(6) VALUE 0.
       01 WS-REMAINING           PIC 9(6) VALUE 0.
       01 WS-LOT-IDX             PIC 9(3) VALUE 0.
       01 WS-TAKE-QTY            PIC 9(6) VALUE 0.
       01 WS-COGS-THIS-SALE      PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-INVENTORY-VALUE     PIC S9(9)V99 COMP-3 VALUE 0.
       01 WS-FMT-COGS            PIC -(9)9.99.
       01 WS-FMT-VALUE           PIC -(9)9.99.
       01 WS-REPORT-DATE         PIC X(10) VALUE '04-13-2026'.

       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           MOVE "WIDGET-A  " TO WS-ITEM-CODE.
           PERFORM LOAD-SAMPLE-DATA.
           PERFORM PROCESS-SALES.
           PERFORM COMPUTE-INVENTORY-VALUE.
           PERFORM DISPLAY-REPORT.
           STOP RUN.

       LOAD-SAMPLE-DATA.
      *    Purchase lot 1: 100 units @ $10.00
           ADD 1 TO WS-LOT-COUNT.
           MOVE 100 TO WS-LOT-QTY(WS-LOT-COUNT).
           MOVE 10.00 TO WS-LOT-COST(WS-LOT-COUNT).
      *    Purchase lot 2: 200 units @ $12.50
           ADD 1 TO WS-LOT-COUNT.
           MOVE 200 TO WS-LOT-QTY(WS-LOT-COUNT).
           MOVE 12.50 TO WS-LOT-COST(WS-LOT-COUNT).
      *    Purchase lot 3: 150 units @ $15.00
           ADD 1 TO WS-LOT-COUNT.
           MOVE 150 TO WS-LOT-QTY(WS-LOT-COUNT).
           MOVE 15.00 TO WS-LOT-COST(WS-LOT-COUNT).

       PROCESS-SALES.
      *    Sell 250 units FIFO
           MOVE 250 TO WS-SELL-QTY.
           PERFORM FIFO-SELL.

       FIFO-SELL.
           MOVE WS-SELL-QTY TO WS-REMAINING.
           MOVE 1 TO WS-LOT-IDX.
           MOVE 0 TO WS-COGS-THIS-SALE.

           PERFORM UNTIL WS-REMAINING = 0 OR
                         WS-LOT-IDX > WS-LOT-COUNT
               IF WS-LOT-QTY(WS-LOT-IDX) > 0
                   IF WS-LOT-QTY(WS-LOT-IDX) >= WS-REMAINING
                       COMPUTE WS-COGS-THIS-SALE =
                           WS-COGS-THIS-SALE +
                           WS-REMAINING * WS-LOT-COST(WS-LOT-IDX)
                       SUBTRACT WS-REMAINING FROM
                           WS-LOT-QTY(WS-LOT-IDX)
                       MOVE 0 TO WS-REMAINING
                   ELSE
                       COMPUTE WS-COGS-THIS-SALE =
                           WS-COGS-THIS-SALE +
                           WS-LOT-QTY(WS-LOT-IDX) *
                           WS-LOT-COST(WS-LOT-IDX)
                       SUBTRACT WS-LOT-QTY(WS-LOT-IDX)
                           FROM WS-REMAINING
                       MOVE 0 TO WS-LOT-QTY(WS-LOT-IDX)
                   END-IF
               END-IF
               ADD 1 TO WS-LOT-IDX
           END-PERFORM.

           ADD WS-COGS-THIS-SALE TO WS-TOTAL-COGS.
           ADD WS-SELL-QTY TO WS-UNITS-SOLD.

       COMPUTE-INVENTORY-VALUE.
           MOVE 0 TO WS-INVENTORY-VALUE.
           PERFORM VARYING WS-LOT-IDX FROM 1 BY 1
               UNTIL WS-LOT-IDX > WS-LOT-COUNT
               COMPUTE WS-INVENTORY-VALUE =
                   WS-INVENTORY-VALUE +
                   WS-LOT-QTY(WS-LOT-IDX) *
                   WS-LOT-COST(WS-LOT-IDX)
           END-PERFORM.

       DISPLAY-REPORT.
           MOVE WS-TOTAL-COGS TO WS-FMT-COGS.
           MOVE WS-INVENTORY-VALUE TO WS-FMT-VALUE.

           DISPLAY "====================================".
           DISPLAY "INVENTORY VALUATION REPORT (FIFO)".
           DISPLAY "Date: " WS-REPORT-DATE.
           DISPLAY "====================================".
           DISPLAY "Item Code:         " WS-ITEM-CODE.
           DISPLAY "Units Sold:        " WS-UNITS-SOLD.
           DISPLAY "Total COGS:        $" WS-FMT-COGS.
           DISPLAY "Remaining Value:   $" WS-FMT-VALUE.
           DISPLAY "====================================".
