extern crate prometheus;
use crate::typesense::models::{
    typesense_metrics_model::TypesenseMetrics, typesense_stats_model::TypesenseStats,
};
use prometheus::{register_gauge_vec_with_registry, Encoder, Registry, TextEncoder};
use regex::Regex;

#[derive(Debug, Clone)]
pub(crate) struct TargetScrape {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) metrics: Option<TypesenseMetrics>,
    pub(crate) stats: Option<TypesenseStats>,
}

pub(crate) fn generate_metrics(scrapes: Vec<TargetScrape>) -> String {
    let registry = Registry::new();

    let typesense_scrape_up = register_gauge_vec_with_registry!(
        "typesense_scrape_up",
        "1 if scraping a Typesense target succeeded (both /metrics.json and /stats.json); 0 otherwise",
        &["host", "port"],
        registry
    )
    .unwrap();

    let typesense_metrics = register_gauge_vec_with_registry!(
        "typesense_metrics",
        "Data received through the metrics api of typesense",
        &["host", "port", "key"],
        registry
    )
    .unwrap();

    let typesense_stats = register_gauge_vec_with_registry!(
        "typesense_stats",
        "Data received through the stats api of typesense",
        &["host", "port", "key"],
        registry
    )
    .unwrap();

    let typesense_stats_latency_ms = register_gauge_vec_with_registry!(
        "typesense_stats_latency_ms",
        "Each endpoint latency in ms from stats api",
        &["host", "port", "key"],
        registry
    )
    .unwrap();

    let typesense_stats_latency_ms_by_collection = register_gauge_vec_with_registry!(
        "typesense_stats_latency_ms_by_collection",
        "Each endpoint latency in ms from stats api",
        &["host", "port", "key", "method", "collection_name", "action"],
        registry
    )
    .unwrap();

    let typesense_stats_requests_per_second = register_gauge_vec_with_registry!(
        "typesense_stats_requests_per_second",
        "Each endpoint rps from stats api",
        &["host", "port", "key"],
        registry
    )
    .unwrap();

    let typesense_stats_requests_per_second_by_collection = register_gauge_vec_with_registry!(
        "typesense_stats_requests_per_second_by_collection",
        "Each endpoint rps from stats api",
        &["host", "port", "key", "method", "collection_name", "action"],
        registry
    )
    .unwrap();

    for target in scrapes.iter() {
        let port_str = target.port.to_string();
        let up = if target.metrics.is_some() && target.stats.is_some() {
            1.0
        } else {
            0.0
        };

        typesense_scrape_up
            .with_label_values(&[&target.host, &port_str])
            .set(up);

        if let Some(ts_metrics) = target.metrics.as_ref() {
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_cpu1_active_percentage"])
                .set(ts_metrics.system_cpu1_active_percentage.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_cpu3_active_percentage"])
                .set(ts_metrics.system_cpu3_active_percentage.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_cpu2_active_percentage"])
                .set(ts_metrics.system_cpu2_active_percentage.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_cpu4_active_percentage"])
                .set(ts_metrics.system_cpu4_active_percentage.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_cpu_active_percentage"])
                .set(ts_metrics.system_cpu_active_percentage.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_disk_total_bytes"])
                .set(ts_metrics.system_disk_total_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_disk_used_bytes"])
                .set(ts_metrics.system_disk_used_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_memory_total_bytes"])
                .set(ts_metrics.system_memory_total_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_memory_used_bytes"])
                .set(ts_metrics.system_memory_used_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_network_received_bytes"])
                .set(ts_metrics.system_network_received_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "system_network_sent_bytes"])
                .set(ts_metrics.system_network_sent_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_active_bytes"])
                .set(ts_metrics.typesense_memory_active_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_allocated_bytes"])
                .set(ts_metrics.typesense_memory_allocated_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[
                    &target.host,
                    &port_str,
                    "typesense_memory_fragmentation_ratio",
                ])
                .set(
                    ts_metrics
                        .typesense_memory_fragmentation_ratio
                        .parse()
                        .unwrap_or(0.0),
                );
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_mapped_bytes"])
                .set(ts_metrics.typesense_memory_mapped_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_metadata_bytes"])
                .set(ts_metrics.typesense_memory_metadata_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_resident_bytes"])
                .set(ts_metrics.typesense_memory_resident_bytes.parse().unwrap_or(0.0));
            typesense_metrics
                .with_label_values(&[&target.host, &port_str, "typesense_memory_retained_bytes"])
                .set(ts_metrics.typesense_memory_retained_bytes.parse().unwrap_or(0.0));
        }

        if let Some(ts_stats) = target.stats.as_ref() {
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "cache_hit_count"])
                .set(ts_stats.cache_hit_count);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "cache_hit_ratio"])
                .set(ts_stats.cache_hit_ratio);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "cache_miss_count"])
                .set(ts_stats.cache_miss_count);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "delete_latency_ms"])
                .set(ts_stats.delete_latency_ms);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "delete_requests_per_second"])
                .set(ts_stats.delete_requests_per_second);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "import_latency_ms"])
                .set(ts_stats.import_latency_ms);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "import_requests_per_second"])
                .set(ts_stats.import_requests_per_second);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "overloaded_requests_per_second"])
                .set(ts_stats.overloaded_requests_per_second);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "pending_write_batches"])
                .set(ts_stats.pending_write_batches);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "search_latency_ms"])
                .set(ts_stats.search_latency_ms);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "search_requests_per_second"])
                .set(ts_stats.search_requests_per_second);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "total_requests_per_second"])
                .set(ts_stats.total_requests_per_second);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "write_latency_ms"])
                .set(ts_stats.write_latency_ms);
            typesense_stats
                .with_label_values(&[&target.host, &port_str, "write_requests_per_second"])
                .set(ts_stats.write_requests_per_second);

            for (key, value) in ts_stats.latency_ms.iter() {
                typesense_stats_latency_ms
                    .with_label_values(&[&target.host, &port_str, key])
                    .set(value.to_string().parse::<f64>().unwrap_or(0.0));

                if let Some((method, collection_name, action)) = parse_collection_action_line(key) {
                    typesense_stats_latency_ms_by_collection
                        .with_label_values(&[
                            &target.host,
                            &port_str,
                            key,
                            method.as_str(),
                            collection_name.as_str(),
                            action.as_str(),
                        ])
                        .set(value.to_string().parse::<f64>().unwrap_or(0.0));
                }
            }

            for (key, value) in ts_stats.requests_per_second.iter() {
                typesense_stats_requests_per_second
                    .with_label_values(&[&target.host, &port_str, key])
                    .set(value.to_string().parse::<f64>().unwrap_or(0.0));

                if let Some((method, collection_name, action)) = parse_collection_action_line(key) {
                    typesense_stats_requests_per_second_by_collection
                        .with_label_values(&[
                            &target.host,
                            &port_str,
                            key,
                            method.as_str(),
                            collection_name.as_str(),
                            action.as_str(),
                        ])
                        .set(value.to_string().parse::<f64>().unwrap_or(0.0));
                }
            }
        }
    }

    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let metric_line = String::from_utf8(buffer).unwrap();

    metric_line
}

fn parse_collection_action_line(input: &str) -> Option<(String, String, String)> {
    let pattern = r"(\w+)\s+/collections/([^/]+)/([^/]+/[^/]+)";
    let re = Regex::new(pattern).unwrap();

    re.captures(input).map(|caps| {
        let method = caps.get(1).unwrap().as_str().to_string();
        let collection_name = caps.get(2).unwrap().as_str().to_string();
        let action = caps.get(3).unwrap().as_str().to_string();
        (method, collection_name, action)
    })
}
