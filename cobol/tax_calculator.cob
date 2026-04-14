      *> ============================================================
      *> TAX-CALCULATOR.COB
      *> Purpose : Compute federal income tax liability using a flat
      *>           25% rate after applying the standard deduction.
      *> Inputs  : Hard-coded values for demonstration.
      *>           GROSS-INCOME      = $85,000.00
      *>           STANDARD-DEDUCTION= $14,600.00
      *>           TAX-RATE          =     0.25 (25%)
      *> Outputs : TAXABLE-INCOME, TAX-OWED, NET-INCOME displayed
      *>           to standard output.
      *> Compile : cobc -x -free tax_calculator.cob -o tax_calculator
      *>           OR (fixed format):
      *>           cobc -x tax_calculator.cob -o tax_calculator
      *> ============================================================
       IDENTIFICATION DIVISION.
       PROGRAM-ID.    TAX-CALCULATOR.
       AUTHOR.        RUSTFORGE-MIGRATION-ENGINE.
       DATE-WRITTEN.  2026-04-13.
       DATE-COMPILED. 2026-04-13.
       SECURITY.      NONE.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       SOURCE-COMPUTER. GENERIC-MAINFRAME.
       OBJECT-COMPUTER. GENERIC-MAINFRAME.

       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
      *>     No file I/O — all values are in WORKING-STORAGE.

       DATA DIVISION.

       FILE SECTION.
      *>     No files declared.

       WORKING-STORAGE SECTION.

      *> ---- Input parameters ----------------------------------------
       01  GROSS-INCOME             PIC 9(9)V99   VALUE 85000.00.
       01  STANDARD-DEDUCTION       PIC 9(9)V99   VALUE 14600.00.
       01  TAX-RATE                 PIC V99       VALUE .25.

      *> ---- Computed fields -----------------------------------------
       01  TAXABLE-INCOME           PIC 9(9)V99   VALUE ZEROS.
       01  TAX-OWED                 PIC 9(9)V99   VALUE ZEROS.
       01  NET-INCOME               PIC 9(9)V99   VALUE ZEROS.

      *> ---- Display / formatted output fields -----------------------
       01  FMT-GROSS-INCOME         PIC $$$,$$$,$$9.99.
       01  FMT-STANDARD-DEDUCTION   PIC $$$,$$$,$$9.99.
       01  FMT-TAXABLE-INCOME       PIC $$$,$$$,$$9.99.
       01  FMT-TAX-RATE-PCT         PIC ZZ9.99.
       01  FMT-TAX-OWED             PIC $$$,$$$,$$9.99.
       01  FMT-NET-INCOME           PIC $$$,$$$,$$9.99.

      *> ---- Separator line for DISPLAY output ----------------------
       01  DISPLAY-SEPARATOR        PIC X(50)
               VALUE '=================================================='.

       PROCEDURE DIVISION.

       000-MAIN-PROCEDURE.
           PERFORM 100-INITIALIZE
           PERFORM 200-COMPUTE-TAX
           PERFORM 300-DISPLAY-RESULTS
           STOP RUN.

       100-INITIALIZE.
      *>     Ensure computed fields start at zero before calculation.
           MOVE ZEROS TO TAXABLE-INCOME
           MOVE ZEROS TO TAX-OWED
           MOVE ZEROS TO NET-INCOME.

       200-COMPUTE-TAX.
      *>     Step 1: Taxable income = gross income - standard deduction.
           COMPUTE TAXABLE-INCOME ROUNDED =
               GROSS-INCOME - STANDARD-DEDUCTION

      *>     Step 2: Tax owed = taxable income * flat tax rate.
           COMPUTE TAX-OWED ROUNDED =
               TAXABLE-INCOME * TAX-RATE

      *>     Step 3: Net income = gross income - tax owed.
           COMPUTE NET-INCOME ROUNDED =
               GROSS-INCOME - TAX-OWED.

       300-DISPLAY-RESULTS.
      *>     Format numeric values into display fields.
           MOVE GROSS-INCOME       TO FMT-GROSS-INCOME
           MOVE STANDARD-DEDUCTION TO FMT-STANDARD-DEDUCTION
           MOVE TAXABLE-INCOME     TO FMT-TAXABLE-INCOME
           MOVE TAX-OWED           TO FMT-TAX-OWED
           MOVE NET-INCOME         TO FMT-NET-INCOME

      *>     Convert rate to percentage for display (0.25 -> 25.00).
           COMPUTE FMT-TAX-RATE-PCT = TAX-RATE * 100

           DISPLAY DISPLAY-SEPARATOR
           DISPLAY 'TAX CALCULATION REPORT'
           DISPLAY DISPLAY-SEPARATOR
           DISPLAY 'GROSS INCOME          : ' FMT-GROSS-INCOME
           DISPLAY 'STANDARD DEDUCTION    : ' FMT-STANDARD-DEDUCTION
           DISPLAY DISPLAY-SEPARATOR
           DISPLAY 'TAXABLE INCOME        : ' FMT-TAXABLE-INCOME
           DISPLAY 'TAX RATE (FLAT)       : ' FMT-TAX-RATE-PCT '%'
           DISPLAY 'TAX OWED              : ' FMT-TAX-OWED
           DISPLAY DISPLAY-SEPARATOR
           DISPLAY 'NET INCOME (AFTER TAX): ' FMT-NET-INCOME
           DISPLAY DISPLAY-SEPARATOR.
