! statistics.f90 — Descriptive statistics subroutine (Fortran 90)
! MOD-005: Statistical Report Generator
! Rust translation in src/statistics.rs mirrors this formula exactly.

SUBROUTINE compute_stats(data, n, mean, variance, std_dev, min_val, max_val)
    IMPLICIT NONE
    INTEGER, INTENT(IN) :: n
    REAL(KIND=8), INTENT(IN) :: data(n)
    REAL(KIND=8), INTENT(OUT) :: mean, variance, std_dev, min_val, max_val

    INTEGER :: i
    REAL(KIND=8) :: sum_x, sum_sq, diff

    ! Guard against empty array
    IF (n == 0) THEN
        mean = 0.0D0
        variance = 0.0D0
        std_dev = 0.0D0
        min_val = 0.0D0
        max_val = 0.0D0
        RETURN
    END IF

    ! Initialise from first element
    sum_x = 0.0D0
    min_val = data(1)
    max_val = data(1)

    ! Single pass: sum and min/max
    DO i = 1, n
        sum_x = sum_x + data(i)
        IF (data(i) < min_val) min_val = data(i)
        IF (data(i) > max_val) max_val = data(i)
    END DO

    mean = sum_x / REAL(n, KIND=8)

    ! Unbiased variance (N-1 denominator)
    sum_sq = 0.0D0
    DO i = 1, n
        diff = data(i) - mean
        sum_sq = sum_sq + diff * diff
    END DO

    IF (n > 1) THEN
        variance = sum_sq / REAL(n - 1, KIND=8)
    ELSE
        variance = 0.0D0
    END IF

    std_dev = SQRT(variance)

END SUBROUTINE compute_stats

! Median — sorts a copy, returns middle value (or average of two middles)
SUBROUTINE compute_median(data, n, median)
    IMPLICIT NONE
    INTEGER, INTENT(IN) :: n
    REAL(KIND=8), INTENT(IN) :: data(n)
    REAL(KIND=8), INTENT(OUT) :: median

    REAL(KIND=8) :: tmp(n)
    REAL(KIND=8) :: swap_val
    INTEGER :: i, j

    ! Copy data
    DO i = 1, n
        tmp(i) = data(i)
    END DO

    ! Bubble sort (simple, correct for test sizes)
    DO i = 1, n - 1
        DO j = 1, n - i
            IF (tmp(j) > tmp(j + 1)) THEN
                swap_val = tmp(j)
                tmp(j) = tmp(j + 1)
                tmp(j + 1) = swap_val
            END IF
        END DO
    END DO

    IF (MOD(n, 2) == 1) THEN
        median = tmp((n + 1) / 2)
    ELSE
        median = (tmp(n / 2) + tmp(n / 2 + 1)) / 2.0D0
    END IF

END SUBROUTINE compute_median

! Driver program for standalone testing
PROGRAM statistics_demo
    IMPLICIT NONE
    INTEGER, PARAMETER :: N = 7
    REAL(KIND=8) :: data(N)
    REAL(KIND=8) :: mean, variance, std_dev, min_val, max_val, median

    data = (/ 2.0D0, 4.0D0, 4.0D0, 4.0D0, 5.0D0, 5.0D0, 7.0D0 /)

    CALL compute_stats(data, N, mean, variance, std_dev, min_val, max_val)
    CALL compute_median(data, N, median)

    WRITE(*,'(A)') "===================================="
    WRITE(*,'(A)') "STATISTICS REPORT"
    WRITE(*,'(A)') "===================================="
    WRITE(*,'(A,F12.6)') "Mean:      ", mean
    WRITE(*,'(A,F12.6)') "Variance:  ", variance
    WRITE(*,'(A,F12.6)') "Std Dev:   ", std_dev
    WRITE(*,'(A,F12.6)') "Min:       ", min_val
    WRITE(*,'(A,F12.6)') "Max:       ", max_val
    WRITE(*,'(A,F12.6)') "Median:    ", median
    WRITE(*,'(A)') "===================================="

END PROGRAM statistics_demo
