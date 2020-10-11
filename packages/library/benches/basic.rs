use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mrml;

fn render(input: &str) {
    mrml::to_html(input, mrml::Options::default()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_str!("../test/mj-button.mjml");
    c.bench_function("render button", |b| b.iter(|| render(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
