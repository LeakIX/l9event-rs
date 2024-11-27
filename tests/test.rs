use l9format::L9Event;
use std::fs;

#[test]
fn test_deserialize_l9format_sample_event() {
    // Path to the JSON file
    let file_path = "tests/data/sample_event.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "leak");
    assert_eq!(event.host, "site1.example.com");
    assert_eq!(event.ip, "127.0.0.1");
    assert_eq!(event.http.status, 200);
}

#[test]
fn test_deserialize_l9format_file_0() {
    // Path to the JSON file
    let file_path = "tests/data/l9event_ip4scout_20211219_0.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "synack");
    assert_eq!(event.host, "");
    assert_eq!(event.ip, "205.93.200.123");
    assert_eq!(event.http.status, 0);
}

#[test]
fn test_deserialize_l9format_file_1() {
    // Path to the JSON file
    let file_path = "tests/data/l9event_ip4scout_20211219_1.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "synack");
    assert_eq!(event.host, "");
    assert_eq!(event.port, "2064");
    assert_eq!(event.http.status, 0);
}

#[test]
fn test_deserialize_l9format_file_2() {
    // Path to the JSON file
    let file_path = "tests/data/l9event_ip4scout_20211219_2.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "synack");
    assert_eq!(event.host, "");
    assert_eq!(event.port, "9112");
    assert_eq!(event.http.status, 0);
}

#[test]
fn test_deserialize_l9format_file_3() {
    // Path to the JSON file
    let file_path = "tests/data/l9event_ip4scout_20211219_3.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "synack");
    assert_eq!(event.host, "");
    assert_eq!(event.port, "7651");
    assert_eq!(event.http.status, 0);
}

#[test]
fn test_deserialize_l9format_file_4() {
    // Path to the JSON file
    let file_path = "tests/data/l9event_ip4scout_20211219_4.json";

    // Read the JSON file
    let json_content = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    // Deserialize into L9Event
    let event: L9Event =
        serde_json::from_str(&json_content).expect("Failed to deserialize the JSON content");

    // Verify some fields (adjust based on the actual file contents)
    assert_eq!(event.event_type, "synack");
    assert_eq!(event.host, "");
    assert_eq!(event.port, "4442");
    assert_eq!(event.http.status, 0);
}
