use alex_db_lib::{
    config::Config,
    db::Db,
    value_record::{Value, ValueIncrement, ValuePost},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

fn increment(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::Integer(i as i64),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_increment = ValueIncrement {
            increment: Some(i as i64),
        };

        db.try_increment(&key, value_increment).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("increment", |b| b.iter(|| increment(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
