use chrono::{DateTime, FixedOffset};
use futures::stream;
use influxdb2::models::{DataPoint, Query};
use influxdb2::{Client, FromDataPoint};
use std::env;
use tokio::time::{self, Duration}; // Streamを使用するために追加

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
    let location = env::var("NODE_LOCATION").unwrap_or("some_node".to_string());
    let destination = env::var("CENTRAL_DB_SVC").unwrap();

    let client = Client::new(format!("http://{}:8086", host), org.clone(), token.clone());
    let destination_client = Client::new(
        format!("http://{}:8086", destination),
        org.clone(),
        token.clone(),
    );
    let qs = r#"
        from(bucket: "some_data")
        |> range(start: -10s)
        |> filter(fn: (r) => r["_measurement"] == "mqtt_consumer")
        |> yield(name: "mean")
    "#;
    let query = Query::new(qs.to_string());
    let res: Vec<Surroundings> = client.query::<Surroundings>(Some(query)).await?;
    let count = res.len() as f64;
    let sum_rssi = res.iter().map(|x| x.rssi).sum::<f64>() / count;
    let sum_temperature = res.iter().map(|x| x.tempreture).sum::<f64>() / count;
    let sum_moisture = res.iter().map(|x| x.moisuture).sum::<f64>() / count;
    let sum_air_pressure = res.iter().map(|x| x.airPressure).sum::<f64>() / count;

    // 新しいデータポイントの作成
    let new_data = DataPoint::builder("average_data")
        .tag("location", location)
        .field("avg_rssi", sum_rssi)
        .field("avg_temperature", sum_temperature)
        .field("avg_moisture", sum_moisture)
        .field("avg_air_pressure", sum_air_pressure)
        .build()?;

    // 別のデータベース（あるいはバケット）にデータを挿入
    let data_points = vec![new_data];

    // Streamを生成
    let data_stream = stream::iter(data_points);

    // データストリームをバケットに書き込む
    let res = destination_client
        .write("surroundings_avg", data_stream)
        .await?;
    println!("Write result: {:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() {
    let interval_str = env::var("BATCH_INTERVAL").unwrap_or_else(|_| "10".to_string());

    let interval: u64 = interval_str.parse().unwrap_or(10);

    let mut interval = time::interval(Duration::from_secs(interval)); // 5分間隔

    loop {
        interval.tick().await; // 次のタイマーイベントまで待機

        if let Err(e) = example().await {
            eprintln!("Error occurred: {}", e);
        }
    }
}
