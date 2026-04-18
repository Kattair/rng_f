use std::hint::black_box;
use std::io;

use criterion::{Criterion, criterion_group, criterion_main};
use rng_f::{AsciiGenerator, Generator, NumberGenerator, write_matrix};

const ASCII_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const ROWS: u128 = 1_000;
const COLS: u128 = 1_000;

fn bench_write_matrix(c: &mut Criterion, name: &str, mut generator: impl Generator) {
    c.bench_function(name, |b| {
        b.iter(|| write_matrix(black_box(&mut generator), &mut io::sink(), ROWS, COLS).unwrap());
    });
}

fn bench_ascii_generator(c: &mut Criterion) {
    bench_write_matrix(
        c,
        "write_matrix/AsciiGenerator",
        AsciiGenerator::new(ASCII_CHARSET.to_owned(), ",").unwrap(),
    );
}

fn bench_number_generator(c: &mut Criterion) {
    bench_write_matrix(
        c,
        "write_matrix/NumberGenerator",
        NumberGenerator::new(i64::MIN..i64::MAX, ",").unwrap(),
    );
}

criterion_group!(benches, bench_ascii_generator, bench_number_generator);
criterion_main!(benches);
