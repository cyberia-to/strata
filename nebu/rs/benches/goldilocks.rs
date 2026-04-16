use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nebu::field::Goldilocks;
use nebu::ntt;
use nebu::sqrt;
use strata_core::Field;
use strata_ext::Batch;
use strata_proof::Dot;

fn bench_add(c: &mut Criterion) {
    let a = Goldilocks::new(0xDEAD_BEEF_CAFE_BABE);
    let b = Goldilocks::new(0x1234_5678_9ABC_DEF0);
    c.bench_function("Goldilocks::add", |bench| {
        bench.iter(|| black_box(a) + black_box(b))
    });
}

fn bench_mul(c: &mut Criterion) {
    let a = Goldilocks::new(0xDEAD_BEEF_CAFE_BABE);
    let b = Goldilocks::new(0x1234_5678_9ABC_DEF0);
    c.bench_function("Goldilocks::mul", |bench| {
        bench.iter(|| black_box(a) * black_box(b))
    });
}

fn bench_square(c: &mut Criterion) {
    let a = Goldilocks::new(0xDEAD_BEEF_CAFE_BABE);
    c.bench_function("Goldilocks::square", |bench| {
        bench.iter(|| black_box(a).square())
    });
}

fn bench_inv(c: &mut Criterion) {
    let a = Goldilocks::new(0xDEAD_BEEF_CAFE_BABE);
    c.bench_function("Goldilocks::inv", |bench| bench.iter(|| black_box(a).inv()));
}

fn bench_pow7(c: &mut Criterion) {
    let a = Goldilocks::new(0xDEAD_BEEF_CAFE_BABE);
    c.bench_function("Goldilocks::pow7", |bench| {
        bench.iter(|| black_box(a).pow7())
    });
}

fn bench_sqrt(c: &mut Criterion) {
    let a = Goldilocks::new(4); // known square
    c.bench_function("Goldilocks::sqrt", |bench| {
        bench.iter(|| sqrt::sqrt(black_box(a)))
    });
}

fn bench_ntt_1024(c: &mut Criterion) {
    let mut data: Vec<Goldilocks> = (0..1024).map(|i| Goldilocks::new(i)).collect();
    c.bench_function("NTT forward 1024", |bench| {
        bench.iter(|| {
            let mut d = data.clone();
            ntt::ntt(black_box(&mut d));
        })
    });
}

fn bench_ntt_4096(c: &mut Criterion) {
    let data: Vec<Goldilocks> = (0..4096).map(|i| Goldilocks::new(i)).collect();
    c.bench_function("NTT forward 4096", |bench| {
        bench.iter(|| {
            let mut d = data.clone();
            ntt::ntt(black_box(&mut d));
        })
    });
}

fn bench_ntt_4096_twiddles(c: &mut Criterion) {
    let data: Vec<Goldilocks> = (0..4096).map(|i| Goldilocks::new(i)).collect();
    let twiddles = ntt::precompute_twiddles_vec(4096);
    c.bench_function("NTT forward 4096 (precomputed)", |bench| {
        bench.iter(|| {
            let mut d = data.clone();
            ntt::ntt_with_twiddles(black_box(&mut d), &twiddles);
        })
    });
}

fn bench_batch_inv_1000(c: &mut Criterion) {
    let mut elems: Vec<Goldilocks> = (1..=1000).map(|i| Goldilocks::new(i)).collect();
    c.bench_function("batch_inv 1000", |bench| {
        bench.iter(|| {
            let mut e = elems.clone();
            Goldilocks::batch_inv(black_box(&mut e));
        })
    });
}

fn bench_dot_1000(c: &mut Criterion) {
    let a: Vec<Goldilocks> = (0..1000).map(|i| Goldilocks::new(i * 7 + 3)).collect();
    let b: Vec<Goldilocks> = (0..1000).map(|i| Goldilocks::new(i * 11 + 5)).collect();
    c.bench_function("Dot::dot 1000", |bench| {
        bench.iter(|| Goldilocks::dot(black_box(&a), black_box(&b)))
    });
}

criterion_group!(
    benches,
    bench_add,
    bench_mul,
    bench_square,
    bench_inv,
    bench_pow7,
    bench_sqrt,
    bench_ntt_1024,
    bench_ntt_4096,
    bench_batch_inv_1000,
    bench_dot_1000,
    bench_ntt_4096_twiddles,
);
criterion_main!(benches);
