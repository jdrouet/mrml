use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mrml::prelude::render::Options;

fn render(input: &str) {
    let opts = Options::default();
    let root = mrml::mjml::MJML::parse(input).unwrap();
    root.render(&opts).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = include_str!("../resources/template/amario.mjml");
    c.bench_function("amario", |b| b.iter(|| render(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
