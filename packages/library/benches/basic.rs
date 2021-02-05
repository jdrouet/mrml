use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mrml;

fn render_default_options(input: &str) {
    mrml::to_html(input, mrml::Options::default()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_str!("../resources/compare/success/mj-button.mjml");
    c.bench_function("render button", |b| {
        b.iter(|| render_default_options(black_box(data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
