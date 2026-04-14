use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rustforge::account_balance::process_account;
use rustforge::batch_processor::{BatchSummary, Transaction};
use rustforge::eigenvalue_solver::PowerIterationSolver;
use rustforge::matrix_operations::MatrixOps;
use rustforge::loan_calculator::{calculate_loan, LoanParams};

fn bench_account_balance(c: &mut Criterion) {
    c.bench_function("account_balance_single", |b| {
        b.iter(|| {
            black_box(process_account(
                black_box(100_000.0),
                black_box(50_000.0),
                black_box(0.035),
            ))
        })
    });

    let mut group = c.benchmark_group("account_balance_rates");
    for rate in [0.03, 0.035, 0.05, 0.07].iter() {
        group.bench_with_input(BenchmarkId::new("rate", rate), rate, |b, &rate| {
            b.iter(|| black_box(process_account(black_box(100_000.0), black_box(50_000.0), black_box(rate))))
        });
    }
    group.finish();
}

fn bench_batch_processor(c: &mut Criterion) {
    let transactions = vec![
        Transaction::new(1, 100_000.0, 'C'),
        Transaction::new(2, 75_000.0, 'C'),
        Transaction::new(3, 50_000.0, 'D'),
        Transaction::new(4, 25_000.0, 'D'),
        Transaction::new(5, 150_000.0, 'C'),
    ];

    c.bench_function("batch_processor_5tx", |b| {
        b.iter(|| {
            let mut s = BatchSummary::new();
            s.process_transactions(black_box(&transactions));
            black_box(s.net_change)
        })
    });
}

fn bench_eigenvalue(c: &mut Criterion) {
    let solver = PowerIterationSolver::new(100, 1e-10);
    let matrix = PowerIterationSolver::initialize_matrix(5);

    c.bench_function("eigenvalue_5x5", |b| {
        b.iter(|| black_box(solver.solve(black_box(&matrix))))
    });
}

fn bench_matrix_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_multiply");
    for n in [3usize, 5, 10].iter() {
        let a: Vec<Vec<f64>> = (0..*n).map(|i| (0..*n).map(|j| (i * n + j) as f64).collect()).collect();
        let b = a.clone();
        group.bench_with_input(BenchmarkId::new("NxN", n), n, |bench, _| {
            bench.iter(|| black_box(MatrixOps::matrix_multiply(black_box(&a), black_box(&b))))
        });
    }
    group.finish();
}

fn bench_loan_calculator(c: &mut Criterion) {
    let params = LoanParams {
        principal: 300_000.0,
        annual_rate_pct: 6.5,
        term_months: 360,
    };

    c.bench_function("loan_30yr_mortgage", |b| {
        b.iter(|| black_box(calculate_loan(black_box(&params))))
    });
}

criterion_group!(benches, bench_account_balance, bench_batch_processor, bench_eigenvalue, bench_matrix_ops, bench_loan_calculator);
criterion_main!(benches);
