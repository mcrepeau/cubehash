use std::io::{Read};
use criterion::{criterion_group, criterion_main, Criterion, Throughput, BatchSize, black_box};

use cubehash::cubehash;

struct ZeroReader {
    remaining: u64,
}

impl Read for ZeroReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining == 0 {
            return Ok(0);
        }
        let n = (buf.len() as u64).min(self.remaining) as usize;
        buf[..n].fill(0);
        self.remaining -= n as u64;
        Ok(n)
    }
}

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("cubehash_sizes");

    let sizes = [
        100 * 1024,                 // 100 kB
        1 * 1024 * 1024,            // 1 MB
        10 * 1024 * 1024,           // 10 MB
        25 * 1024 * 1024,           // 25 MB
    ];
    let revisions = [2, 3];
    let hashlen = 256i32;

    for &size in &sizes {
        group.throughput(Throughput::Bytes(size));

        for &rev in &revisions {
            group.bench_function(format!("size_{}_rev_{}", size, rev), |b| {
                b.iter_batched(
                    || ZeroReader { remaining: black_box(size) },
                    |mut reader| {
                        let _ = cubehash(&mut reader, rev, hashlen);
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

