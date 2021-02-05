use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mrml;

fn render_default_options(input: &str) {
    mrml::to_html(input, mrml::Options::default()).unwrap();
}

fn render_without_comments(input: &str) {
    let opts = mrml::Options {
        keep_comments: false,
        ..mrml::Options::default()
    };
    mrml::to_html(input, opts).unwrap();
}

fn amario_benchmark(c: &mut Criterion) {
    let data = include_str!("../resources/template/amario.mjml");
    c.bench_function("amario", |b| {
        b.iter(|| render_default_options(black_box(data)))
    });
    c.bench_function("amario without comments", |b| {
        b.iter(|| render_without_comments(black_box(data)))
    });
}

criterion_group!(benches, amario_benchmark);
criterion_main!(benches);
