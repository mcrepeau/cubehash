use std::io::Cursor;
use criterion::{criterion_group, criterion_main, Criterion, Throughput, BatchSize, black_box};

use cubehash::cubehash;

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("cubehash_sizes");

    let sizes = [0usize, 32, 1024, 64 * 1024, 1 * 1024 * 1024];
    let revisions = [2, 3];
    let hashlen = 256i32;

    for &size in &sizes {
        group.throughput(Throughput::Bytes(size as u64));
        let data = vec![0u8; size.max(1)]; // avoid zero-length in BatchInput

        for &rev in &revisions {
            group.bench_function(format!("size_{}_rev_{}", size, rev), |b| {
                b.iter_batched(
                    || Cursor::new(black_box(&data)),
                    |mut cursor| {
                        let _ = cubehash(&mut cursor, rev, hashlen);
                    },
                    BatchSize::SmallInput,
                );
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_sizes);
criterion_main!(benches);

