use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_leb128_unsigned(c: &mut Criterion) {
    let mut group = c.benchmark_group("leb128 decoding unsigned");

    let edge_cases = [0, 1, u64::MAX, u64::MAX / 2, 1 << 63, (1 << 63) - 1];

    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:leb128::read::u64", format!("{}{}", value, i)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let mut writable = &mut buffer[..];
                leb128::write::unsigned(&mut writable, v).expect("Should write number");

                let mut writable = &writable[..];
                b.iter(|| leb128::read::unsigned(&mut writable));
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1u64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:leb128::read::u64", format!("{}{}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let mut writable = &mut buffer[..];
                leb128::write::unsigned(&mut writable, v).expect("Should write number");
                let mut writable = &writable[..];
                b.iter(|| leb128::read::unsigned(&mut writable));
            },
        );
    }

    group.finish();
}
fn bench_leb128_signed(c: &mut Criterion) {
    let mut group = c.benchmark_group("leb128 decoding signed");

    let edge_cases: [i64; 6] = [0, 1, i64::MAX, i64::MAX / 2, 1 << 63, (-1 << 63) + 1];
    // Bench edge cases
    for (i, &value) in edge_cases.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("edge:leb128::read::i64", format!("{}{}", value, i)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let mut writable = &mut buffer[..];
                leb128::write::signed(&mut writable, v).expect("Should write number");
                let mut writable = &writable[..];
                b.iter(|| leb128::read::signed(&mut writable));
            },
        );
    }

    // Logarithmic sampling over the u64 range
    for power in 0..=63 {
        let value = 1i64 << power;
        group.bench_with_input(
            BenchmarkId::new("log:128::read::i64", format!("{} {}", value, power)),
            &value,
            |b, &v| {
                let mut buffer = std::hint::black_box([0u8; 10]);
                let mut writable = &mut buffer[..];
                leb128::write::signed(&mut writable, v).expect("Should write number");
                let mut writable = &writable[..];
                b.iter(|| leb128::read::signed(&mut writable));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_leb128_signed, bench_leb128_unsigned);
criterion_main!(benches);
