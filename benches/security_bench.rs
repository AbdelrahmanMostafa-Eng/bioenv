use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bioenv::config::Config;
use bioenv::security::ExposeBioEnvSecret;
use secrecy::SecretString;

fn bench_namespace_hashing(c: &mut Criterion) {
    c.bench_function("project_namespace_hashing", |b| {
        b.iter(|| Config::project_namespace())
    });
}

fn bench_secret_exposure(c: &mut Criterion) {
    let secret = SecretString::from("a_very_long_secret_key_for_benchmarking_purposes");
    c.bench_function("secret_exposure_overhead", |b| {
        b.iter(|| black_box(secret.expose()))
    });
}

criterion_group!(benches, bench_namespace_hashing, bench_secret_exposure);
criterion_main!(benches);
