use criterion::{black_box, criterion_group, criterion_main, Criterion};
use popflash_parser::r#match;
use popflash_parser::*;

use select::document::Document;
use tokio::runtime::{Handle, Runtime};

macro_rules! bench_setup {
    () => {
        Runtime::new().unwrap().block_on(async {
            popflash_parser::utility::get_body_from_url("https://popflash.site/match/1281644")
                .await
                .unwrap()
        })
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let document = bench_setup!();
    c.bench_function("match::Match::get_teams", |b| {
        b.iter(|| Match::get_teams(&document))
    });
}

criterion_group!(match_functions, criterion_benchmark);
