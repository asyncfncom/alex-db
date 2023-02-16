use alex_db_lib::{
    config::Config,
    db::Db,
    value_record::{Value, ValuePopBack, ValuePost},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::VecDeque, sync::Arc};

fn pop_back(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::Array(VecDeque::from([
                Value::String("test_value1".to_string()),
                Value::String("test_value2".to_string()),
            ])),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_pop_back = ValuePopBack { pop_back: None };

        db.try_pop_back(&key, value_pop_back).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("pop_back", |b| b.iter(|| pop_back(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
