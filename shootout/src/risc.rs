use std::path::Path;
use criterion::BenchmarkGroup;
use criterion::measurement::WallTime;

pub fn bench_sudoku(c: &mut BenchmarkGroup<WallTime>) {
    use sudoku_risc::*;
    let receipt = prove(setup());
    c.bench_function("RISC0: sudoku-setup", |b| b.iter(|| setup()));
    c.bench_function("RISC0: sudoku-prove", |b| b.iter(|| prove(setup())));
    c.bench_function("RISC0: sudoku-digest", |b| b.iter(|| digest(&receipt)));
    c.bench_function("RISC0: sudoku-verify", |b| b.iter(|| verify(&receipt)));
    c.bench_function("RISC0: sudoku", |b| b.iter(|| prove_and_verify()));
}

pub fn bench_fib(c: &mut BenchmarkGroup<WallTime>) {
    use risc::*;
    let receipt = prove(setup(93));
    c.bench_function("RISC0: fib-setup", |b| b.iter(|| setup(93)));
    c.bench_function("RISC0: fib-prove", |b| b.iter(|| prove(setup(93))));
    c.bench_function("RISC0: fib-digest", |b| b.iter(|| digest(&receipt)));
    c.bench_function("RISC0: fib-verify", |b| b.iter(|| verify(FIB_ID, &receipt)));
    c.bench_function("RISC0: fib", |b| b.iter(|| prove_and_verify(93)));
    bench_fib_fixed(c, FIB_FIFTY_ID, &FIB_FIFTY_PATH, "50");
    bench_fib_fixed(c, FIB_NINTY_TWO_ID, &FIB_NINTY_TWO_PATH, "92");
}

pub fn bench_fib_fixed(
    c: &mut BenchmarkGroup<WallTime>,
    method_id: &[u8],
    method_path: &dyn AsRef<Path>,
    fib_number: &str,
) {
    use risc::*;
    let setup = || setup_fix(method_id, method_path);
    let receipt = prove(setup());
    let name = format!("fib{}", fib_number);
    c.bench_function(&format!("RISC0: {}-setup", name), |b| b.iter(|| setup()));
    c.bench_function(&format!("RISC0: {}-prove", name), |b| {
        b.iter(|| prove(setup()))
    });
    c.bench_function(&format!("RISC0: {}-digest", name), |b| {
        b.iter(|| digest(&receipt))
    });
    c.bench_function(&format!("RISC0: {}-verify", name), |b| {
        b.iter(|| verify(method_id, &receipt))
    });
    c.bench_function(&format!("RISC0: {}", name), |b| {
        b.iter(|| prove_and_verify_fix(method_id, method_path))
    });
}
