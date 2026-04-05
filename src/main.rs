mod client;

use crate::client::prometheus::{
    run_prometheus,
    PrometheusQueryParams,
    PrometheusConfig
};

fn main() {
    let query: String = format!("histogram_quantile(0.99, sum by (le) (rate(bob_http_request_duration_seconds_bucket[10m])))");
    let res = run_prometheus(query, PrometheusQueryParams::default() , PrometheusConfig::default());
    println!("Hello, world! {:?}", res);
}
