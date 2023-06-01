mod json_daisychain;
mod json_nom;

use std::collections::HashMap;

use criterion::{black_box, criterion_group, Criterion};
use json_daisychain::daisychain_parser;
use json_nom::nom_parser;
use log::{trace};

const JSON: &str = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  } ";

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub fn bench_nom(c: &mut Criterion) {
    c.bench_function("json_nom", |b| b.iter(|| nom_parser(black_box(JSON))));
}

pub fn bench_daisychain(c: &mut Criterion) {
    c.bench_function("json_daisychain", |b| {
        b.iter(|| daisychain_parser(black_box(JSON)))
    });
}

criterion_group!(benches, bench_nom, bench_daisychain);
// criterion_main!(benches);

fn main() {
    env_logger::init();
    trace!(target:"dc" ,"Logging enabled");
    benches();
    Criterion::default().configure_from_args().final_summary();
}
