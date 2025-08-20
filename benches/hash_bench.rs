use criterion::{criterion_group, criterion_main, Criterion, Throughput, BatchSize};
use cubehash::{CubeHashBest, CubeHashParams};

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("cubehash_sizes");

    let sizes = [
        100 * 1024,        // 100 kB
        1 * 1024 * 1024,   // 1 MB
        10 * 1024 * 1024,  // 10 MB
        25 * 1024 * 1024,  // 25 MB
    ];
    let revisions = [2, 3];
    let hashlen = 256i32;

    for &size in &sizes {
        group.throughput(Throughput::Bytes(size as u64));

        for &rev in &revisions {
            let params = CubeHashParams { revision: rev, hash_len_bits: hashlen };

            group.bench_function(format!("size_{}_rev_{}", size, rev), |b| {
                b.iter_batched(
                    || {
                        // Preallocate a buffer filled with zeros
                        let buf = vec![0u8; size];
                        buf
                    },
                    |buf| {
                        // Hash the buffer
                        let mut hasher = CubeHashBest::new(params);
                        hasher.update(&buf);
                        let _ = hasher.finalize();
                    },
                    BatchSize::LargeInput, // single large input per iteration
                );
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_sizes);
criterion_main!(benches);
