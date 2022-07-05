use criterion::{black_box, criterion_group, criterion_main, Criterion};
use popflash_parser::r#match;
use popflash_parser::*;

use tokio::runtime::{Handle, Runtime};
pub async fn criterion_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let body = Runtime::block_on(&rt, utility::get_body_from_url(EXAMPLE_MATCH_URL)).unwrap();

    c.bench_function("match::Match::get_teams", |b| b.iter(|| (&body)));
}

criterion_group!(match_functions, criterion_benchmark);
