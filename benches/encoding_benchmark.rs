use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hamming_rust::{
    encoding::bitvec::BitVec,
    encoding::hamming::{Hamming, HammingCode},
};
use rand::Rng;

fn generate_random_bitvec(size: usize) -> BitVec {
    let mut rng = rand::rng();
    let mut bv = BitVec::with_capacity(size);
    for _ in 0..size {
        bv.push(rng.random_bool(0.5));
    }
    bv
}

fn benchmark_implementations(c: &mut Criterion) {
    let sizes = [128, 1024, 8192, 65536];

    for size in sizes {
        let data = generate_random_bitvec(size);

        let mut group = c.benchmark_group(format!("hamming_encode_{}_bits", size));

        // Single-threaded benchmark
        group.bench_function("standard", |b| {
            let hamming = Hamming;
            b.iter(|| hamming.encode(black_box(&data)))
        });

        group.finish();

        // Now benchmark decoding with an error
        let mut encoded = Hamming.encode(&data).unwrap();
        encoded.toggle(size / 2).unwrap(); // Introduce an error in the middle

        let mut group = c.benchmark_group(format!("hamming_decode_{}_bits", size));

        // Single-threaded benchmark
        group.bench_function("standard", |b| {
            let hamming = Hamming;
            b.iter(|| hamming.decode(black_box(&encoded)))
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_implementations);
criterion_main!(benches);
