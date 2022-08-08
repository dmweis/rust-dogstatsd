use std::path::PathBuf;

// Import the client object.
use datadog_statsd::client::{AlertType, Client, ServiceCheckStatus};

fn main() {
    let client = Client::with_uds_socket(
        &PathBuf::from(r"YOUR_SOCKET"),
        "myapp",
        Some(vec!["common1", "common2:test"]),
    )
    .unwrap();
    let tags = &Some(vec!["tag1", "tag2:test"]);

    client.incr("some.counter", tags);
    println!("Sent a counter!");

    client.gauge("some.gauge", 124.0, tags);
    println!("Set a gauge!");

    client.timer("timer.duration", 182.1, &None);
    println!("Set a timer!");

    client.time("closure.duration", tags, || {
        println!("Timing a closure");
    });

    client.histogram("some.histogram", 104.3, tags);
    println!("Set a histogram!");

    client.event("event title", "event text", AlertType::Warning, tags);
    println!("Sent a event!");

    client.service_check(
        "myapp.service.check.name",
        ServiceCheckStatus::Critical,
        tags,
    );
    println!("Sent a service_check!");
}
