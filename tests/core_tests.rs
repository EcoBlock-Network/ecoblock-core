use ecoblock_core::{SensorData, NodeId, TangleBlockData, Signature};
use serde_json;


#[test]
fn sensor_data_can_be_created() {
    let data = SensorData {
        pm25: 12.3,
        co2: 415.0,
        temperature: 22.5,
        humidity: 45.0,
        timestamp: 1_694_888_000,
    };

    assert_eq!(data.pm25, 12.3);
    assert!(data.co2 > 400.0);
}


