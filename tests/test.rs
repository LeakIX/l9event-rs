use l9event::L9Event;
use std::fs;

#[test]
fn test_deserialize_l9event_from_file() {
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
