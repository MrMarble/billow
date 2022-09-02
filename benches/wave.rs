use billow::Wave;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use image::DynamicImage;

fn load_assets(folder: &str) -> Vec<DynamicImage> {
    let path = format!("assets/{}", folder);

    let mut images: Vec<DynamicImage> = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "png" || ext == "jpg" {
            match image::open(path) {
                Ok(image) => images.push(image),
                Err(err) => {
                    panic!("Failed to load image: {}", err)
                }
            }
        }
    }
    images
}

fn initialize(c: &mut Criterion) {
    let mut group = c.benchmark_group("initilize");
    let assets = load_assets("basic");

    for size in [5, 10, 20, 50, 100, 200].iter() {
        group.throughput(criterion::Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("wave", size), size, |b, &size| {
            let mut wave = Wave::new(&assets, size, size);
            b.iter(|| wave.initialize());
        });
    }
}

fn least_entropy(c: &mut Criterion) {
    c.bench_function("collapse_least_entropy 20x20", |b| {
        let assets = load_assets("basic");
        let mut wave = Wave::new(&assets, black_box(20), black_box(20));
        wave.initialize();

        b.iter(|| wave.collapse_least_entropy())
    });
}

fn collapse(c: &mut Criterion) {
    c.bench_function("collapse 20x20", |b| {
        let assets = load_assets("basic");
        let mut wave = Wave::new(&assets, black_box(20), black_box(20));
        wave.initialize();

        b.iter(|| {
            wave.initialize();
            wave.collapse(100)
        })
    });
}

criterion_group!(benches, initialize, least_entropy, collapse);
criterion_main!(benches);
