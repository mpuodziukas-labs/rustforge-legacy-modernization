module matrix_operations
    implicit none

contains

    subroutine matrix_multiply(A, B, C, n, m, p)
        implicit none
        integer, intent(in) :: n, m, p
        real(kind=8), intent(in) :: A(n, m), B(m, p)
        real(kind=8), intent(out) :: C(n, p)
        integer :: i, j, k

        C = 0.0d0
        do i = 1, n
            do j = 1, p
                do k = 1, m
                    C(i, j) = C(i, j) + A(i, k) * B(k, j)
                end do
            end do
        end do
    end subroutine matrix_multiply

    subroutine lu_decomposition(A, L, U, n)
        implicit none
        integer, intent(in) :: n
        real(kind=8), intent(in) :: A(n, n)
        real(kind=8), intent(out) :: L(n, n), U(n, n)
        integer :: i, j, k
        real(kind=8) :: sum_val

        L = 0.0d0
        U = 0.0d0

        do i = 1, n
            do j = i, n
                sum_val = 0.0d0
                do k = 1, i - 1
                    sum_val = sum_val + L(i, k) * U(k, j)
                end do
                U(i, j) = A(i, j) - sum_val
            end do

            do j = i + 1, n
                sum_val = 0.0d0
                do k = 1, i - 1
                    sum_val = sum_val + L(j, k) * U(k, i)
                end do
                if (dabs(U(i, i)) < 1.0d-15) then
                    L(j, i) = 0.0d0
                else
                    L(j, i) = (A(j, i) - sum_val) / U(i, i)
                end if
            end do
        end do

        do i = 1, n
            L(i, i) = 1.0d0
        end do
    end subroutine lu_decomposition

    subroutine gaussian_elimination(A, b, x, n)
        implicit none
        integer, intent(in) :: n
        real(kind=8), intent(inout) :: A(n, n), b(n)
        real(kind=8), intent(out) :: x(n)
        integer :: i, j, k
        real(kind=8) :: factor, temp
        integer :: pivot_row

        ! Forward elimination
        do k = 1, n - 1
            ! Partial pivoting
            pivot_row = k
            do i = k + 1, n
                if (dabs(A(i, k)) > dabs(A(pivot_row, k))) then
                    pivot_row = i
                end if
            end do

            ! Swap rows if needed
            if (pivot_row /= k) then
                do j = k, n
                    temp = A(k, j)
                    A(k, j) = A(pivot_row, j)
                    A(pivot_row, j) = temp
                end do
                temp = b(k)
                b(k) = b(pivot_row)
                b(pivot_row) = temp
            end if

            ! Eliminate below pivot
            do i = k + 1, n
                if (dabs(A(k, k)) > 1.0d-15) then
                    factor = A(i, k) / A(k, k)
                    do j = k, n
                        A(i, j) = A(i, j) - factor * A(k, j)
                    end do
                    b(i) = b(i) - factor * b(k)
                end if
            end do
        end do

        ! Back substitution
        do i = n, 1, -1
            x(i) = b(i)
            do j = i + 1, n
                x(i) = x(i) - A(i, j) * x(j)
            end do
            if (dabs(A(i, i)) > 1.0d-15) then
                x(i) = x(i) / A(i, i)
            end if
        end do
    end subroutine gaussian_elimination

    subroutine print_matrix(A, n, m, name)
        implicit none
        integer, intent(in) :: n, m
        real(kind=8), intent(in) :: A(n, m)
        character(len=*), intent(in) :: name
        integer :: i, j

        write(*, '(A)') trim(name)
        do i = 1, n
            do j = 1, m
                write(*, '(F12.6)', advance='no') A(i, j)
            end do
            write(*, '(A)') ''
        end do
        write(*, '(A)') ''
    end subroutine print_matrix

end module matrix_operations

program matrix_ops_test
    use matrix_operations
    implicit none

    integer, parameter :: n = 3
    real(kind=8) :: A(n, n), B(n, n), C(n, n)
    real(kind=8) :: L(n, n), U(n, n)
    integer :: i, j

    ! Initialize test matrices
    A = reshape([4.0d0, 3.0d0, 1.0d0, &
                 3.0d0, 5.0d0, 2.0d0, &
                 1.0d0, 2.0d0, 3.0d0], shape=[n, n])

    B = reshape([1.0d0, 0.0d0, 0.0d0, &
                 0.0d0, 1.0d0, 0.0d0, &
                 0.0d0, 0.0d0, 1.0d0], shape=[n, n])

    ! Test matrix multiplication
    call matrix_multiply(A, B, C, n, n, n)
    call print_matrix(C, n, n, "Matrix A * B:")

    ! Test LU decomposition
    call lu_decomposition(A, L, U, n)
    call print_matrix(L, n, n, "L matrix from LU decomposition:")
    call print_matrix(U, n, n, "U matrix from LU decomposition:")

    write(*, '(A)') "====================================="
    write(*, '(A)') "Matrix Operations Complete"
    write(*, '(A)') "====================================="
end program matrix_ops_test
