mod client;

use crate::client::prometheus::{
    run_prometheus,
    PrometheusQueryParams,
    PrometheusConfig
};

fn main() {
    run_prometheus("some query".to_string(), PrometheusQueryParams::default() , PrometheusConfig::default());
    println!("Hello, world!");
}
