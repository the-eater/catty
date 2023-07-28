#[macro_use]
extern crate criterion;

use criterion::{black_box, Bencher, Criterion};
use futures::channel::oneshot as futures_oneshot;

use criterion::async_executor::FuturesExecutor;

fn create_futures(b: &mut Bencher) {
    b.iter(|| futures_oneshot::channel::<u32>());
}

fn create_tokio(b: &mut Bencher) {
    b.iter(|| tokio::sync::oneshot::channel::<u32>());
}

fn create_catty(b: &mut Bencher) {
    b.iter(|| catty::oneshot::<u32>());
}

fn oneshot_futures(b: &mut Bencher) {
    b.to_async(FuturesExecutor).iter(|| async {
        let (tx, rx) = futures_oneshot::channel();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn oneshot_tokio(b: &mut Bencher) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    b.to_async(rt).iter(|| async {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn oneshot_tokio_mt(b: &mut Bencher) {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    b.to_async(rt).iter(|| async {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn oneshot_pollster_catty(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = catty::oneshot();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(pollster::block_on(rx), Ok(10));
    });
}

fn oneshot_futures_catty(b: &mut Bencher) {
    b.to_async(FuturesExecutor).iter(|| async {
        let (tx, rx) = catty::oneshot();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn oneshot_tokio_mt_catty(b: &mut Bencher) {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    b.to_async(rt).iter(|| async {
        let (tx, rx) = catty::oneshot();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn oneshot_tokio_catty(b: &mut Bencher) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    b.to_async(rt).iter(|| async {
        let (tx, rx) = catty::oneshot();
        tx.send(black_box(10u32)).unwrap();
        assert_eq!(rx.await, Ok(10));
    });
}

fn send_futures(b: &mut Bencher) {
    b.iter(|| {
        let (tx, _rx) = futures_oneshot::channel();
        tx.send(black_box(10u32)).unwrap();
    });
}

fn send_tokio(b: &mut Bencher) {
    b.iter(|| {
        let (tx, _rx) = tokio::sync::oneshot::channel();
        tx.send(black_box(10u32)).unwrap();
    });
}

fn send_catty(b: &mut Bencher) {
    b.iter(|| {
        let (tx, _rx) = catty::oneshot();
        tx.send(black_box(10u32)).unwrap();
    });
}

fn create(c: &mut Criterion) {
    let mut g = c.benchmark_group("create");
    g.bench_function("create-futures", |b| create_futures(b));
    g.bench_function("create-tokio", |b| create_tokio(b));
    g.bench_function("create-catty", |b| create_catty(b));
    g.finish();
}

fn oneshot(c: &mut Criterion) {
    let mut g = c.benchmark_group("oneshot");
    g.bench_function("oneshot-futures", |b| oneshot_futures(b));
    g.bench_function("oneshot-tokio", |b| oneshot_tokio(b));
    g.bench_function("oneshot-tokio-mt", |b| oneshot_tokio_mt(b));
    g.bench_function("oneshot-pollster-catty", |b| oneshot_pollster_catty(b));
    g.bench_function("oneshot-futures-catty", |b| oneshot_futures_catty(b));
    g.bench_function("oneshot-tokio-catty", |b| oneshot_tokio_catty(b));
    g.bench_function("oneshot-tokio-mt-catty", |b| oneshot_tokio_mt_catty(b));
    g.finish();
}

fn send_only(c: &mut Criterion) {
    let mut g = c.benchmark_group("send");
    g.bench_function("send-futures", |b| send_futures(b));
    g.bench_function("send-tokio", |b| send_tokio(b));
    g.bench_function("send-catty", |b| send_catty(b));
    g.finish();
}

criterion_group!(compare, create, oneshot, send_only);
criterion_main!(compare);
