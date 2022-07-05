use criterion::criterion_main;

mod benchmarks;

criterion_main! {
//    benchmarks::compare_functions::fibonaccis,
    benchmarks::r#match::match_functions,
}
