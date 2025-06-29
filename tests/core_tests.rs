use ecoblock_core::{SensorData, NodeId, TangleBlock, Signature};
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

#[test]
fn tangle_block_construction() {
    let node_id = NodeId("abc123".into());
    let signature = Signature("signed".into());
    let data = SensorData {
        pm25: 10.0,
        co2: 400.0,
        temperature: 20.0,
        humidity: 50.0,
        timestamp: 123456789,
    };

    let block = TangleBlock {
        id: "block1".into(),
        parents: vec!["parent1".into(), "parent2".into()],
        data,
        signature,
    };

    assert_eq!(block.parents.len(), 2);
}

#[test]
fn tangle_block_serialization_to_json() {
    let block = TangleBlock {
        id: "abc".into(),
        parents: vec![],
        data: SensorData {
            pm25: 1.0, co2: 400.0, temperature: 21.0, humidity: 45.0,
            timestamp: 123,
        },
        signature: Signature("sig".into()),
    };

    let json = serde_json::to_string(&block).unwrap();
    assert!(json.contains("abc"));
    assert!(json.contains("sig"));
}

