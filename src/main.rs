use chrono::{DateTime, FixedOffset};
use influxdb2::models::Query;
use influxdb2::{Client, FromDataPoint};
use std::env;

#[derive(Debug, FromDataPoint)]
struct Surroundings {
    time: DateTime<FixedOffset>,
    rssi: f64,
    tempreture: f64,
    moisuture: f64,
    airPressure: f64,
}

impl Default for Surroundings {
    fn default() -> Self {
        Surroundings {
            time: chrono::MIN_DATETIME.with_timezone(&chrono::FixedOffset::east(7 * 3600)),
            rssi: 0.0,
            tempreture: 0.0,
            moisuture: 0.0,
            airPressure: 0.0,
        }
    }
}
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("INFLUXDB_HOST").unwrap();
    let org = env::var("INFLUXDB_ORG").unwrap();
    let token = env::var("INFLUXDB_TOKEN").unwrap();
    let client = Client::new(host, org, token);

    let qs = r#"
        from(bucket: "some_data")
        |> range(start: -5m)
        |> filter(fn: (r) => r["_measurement"] == "mqtt_consumer")
        |> yield(name: "mean")
    "#;
    let query = Query::new(qs.to_string());
    let res: Vec<Surroundings> = client.query::<Surroundings>(Some(query)).await?;
    println!("{:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = example().await {
        eprintln!("Error occurred: {}", e);
    }
}
