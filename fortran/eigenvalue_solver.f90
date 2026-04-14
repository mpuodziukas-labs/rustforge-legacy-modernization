program eigenvalue_solver
    implicit none

    integer, parameter :: n = 5
    real(kind=8) :: matrix(n, n)
    real(kind=8) :: eigenvalue
    real(kind=8) :: eigenvector(n)
    integer :: max_iterations
    real(kind=8) :: tolerance

    ! Initialize test matrix (symmetric, positive definite)
    call initialize_matrix(matrix, n)

    ! Compute dominant eigenvalue and eigenvector
    max_iterations = 100
    tolerance = 1.0d-10

    call power_iteration(matrix, n, eigenvalue, eigenvector, max_iterations, tolerance)

    ! Print results
    call print_results(eigenvalue, eigenvector, n)

contains

    subroutine initialize_matrix(A, n)
        implicit none
        integer, intent(in) :: n
        real(kind=8), intent(out) :: A(n, n)
        integer :: i, j

        ! Create a test matrix
        ! A = [5.0, 1.0, 0.5, 0.2, 0.1
        !      1.0, 4.0, 0.8, 0.3, 0.2
        !      0.5, 0.8, 3.0, 0.6, 0.4
        !      0.2, 0.3, 0.6, 2.0, 0.5
        !      0.1, 0.2, 0.4, 0.5, 1.0]

        A = 0.0d0
        A(1, 1) = 5.0d0
        A(1, 2) = 1.0d0
        A(1, 3) = 0.5d0
        A(1, 4) = 0.2d0
        A(1, 5) = 0.1d0

        A(2, 1) = 1.0d0
        A(2, 2) = 4.0d0
        A(2, 3) = 0.8d0
        A(2, 4) = 0.3d0
        A(2, 5) = 0.2d0

        A(3, 1) = 0.5d0
        A(3, 2) = 0.8d0
        A(3, 3) = 3.0d0
        A(3, 4) = 0.6d0
        A(3, 5) = 0.4d0

        A(4, 1) = 0.2d0
        A(4, 2) = 0.3d0
        A(4, 3) = 0.6d0
        A(4, 4) = 2.0d0
        A(4, 5) = 0.5d0

        A(5, 1) = 0.1d0
        A(5, 2) = 0.2d0
        A(5, 3) = 0.4d0
        A(5, 4) = 0.5d0
        A(5, 5) = 1.0d0
    end subroutine initialize_matrix

    subroutine power_iteration(A, n, lambda, v, max_iter, tol)
        implicit none
        integer, intent(in) :: n, max_iter
        real(kind=8), intent(in) :: A(n, n), tol
        real(kind=8), intent(out) :: lambda, v(n)

        real(kind=8) :: v_old(n), v_new(n), lambda_old, lambda_new
        real(kind=8) :: norm, error
        integer :: i, iter

        ! Initialize eigenvector
        v = 1.0d0 / dsqrt(dble(n))
        lambda_old = 0.0d0

        do iter = 1, max_iter
            ! Matrix-vector multiplication: v_new = A * v
            call matvec(A, v, v_new, n)

            ! Compute Euclidean norm
            norm = 0.0d0
            do i = 1, n
                norm = norm + v_new(i) ** 2
            end do
            norm = dsqrt(norm)

            ! Normalize
            v_new = v_new / norm

            ! Rayleigh quotient for eigenvalue
            call matvec(A, v_new, v_old, n)
            lambda_new = 0.0d0
            do i = 1, n
                lambda_new = lambda_new + v_new(i) * v_old(i)
            end do

            ! Check convergence
            error = dabs(lambda_new - lambda_old)
            if (error < tol) then
                lambda = lambda_new
                v = v_new
                return
            end if

            lambda_old = lambda_new
            v = v_new
        end do

        lambda = lambda_old
    end subroutine power_iteration

    subroutine matvec(A, x, y, n)
        implicit none
        integer, intent(in) :: n
        real(kind=8), intent(in) :: A(n, n), x(n)
        real(kind=8), intent(out) :: y(n)
        integer :: i, j

        y = 0.0d0
        do i = 1, n
            do j = 1, n
                y(i) = y(i) + A(i, j) * x(j)
            end do
        end do
    end subroutine matvec

    subroutine print_results(lambda, v, n)
        implicit none
        integer, intent(in) :: n
        real(kind=8), intent(in) :: lambda, v(n)
        integer :: i

        write(*, '(A)') "====================================="
        write(*, '(A)') "EIGENVALUE SOLVER RESULTS"
        write(*, '(A)') "====================================="
        write(*, '(A, F15.10)') "Dominant Eigenvalue: ", lambda
        write(*, '(A)') "Eigenvector:"
        do i = 1, n
            write(*, '(I2, A, F15.10)') i, ": ", v(i)
        end do
        write(*, '(A)') "====================================="
    end subroutine print_results

end program eigenvalue_solver
