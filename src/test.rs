use number_to_attribute;
use parse_http_response_content;
use check_if_key_in_json;
use Node;

#[test]
fn test_number_to_attribute() {
    assert_eq!(number_to_attribute(1), "market_cap".to_string());
    assert_eq!(number_to_attribute(2), "price".to_string());
    assert_eq!(number_to_attribute(3), "volume_24h".to_string());
}

#[test]
#[should_panic]
fn test_number_to_attribute_invalid_parameter() {
    number_to_attribute(0);
}


#[test]
fn test_parse_http_response_content() {
    let mock_http_response: String = String::from("HTTP/1.1 200 OK\r\n\r\n{content: 'example'}");
    let content: String = parse_http_response_content(mock_http_response);
    assert_eq!(content, "{content: 'example'}".to_string())
}

#[test]
fn test_check_if_key_in_json() {
    let value = json!({
    "data": {
        "key1": 200,
        "key2": "foo"
    }});

    assert!(check_if_key_in_json(&value, &"key1".to_string()));
    assert!(!check_if_key_in_json(&value, &"key3".to_string()));
}

#[test]
fn test_tree_structure() {
    let value = json!({
        "data": {
            "1": {
                "name": "Bitcoin",
                "quotes": {
                    "USD": {
                        "price": 8614.73,
                        "volume_24h": 8561930000.0,
                        "market_cap": 146690002871.0
                    }
                }
            },
            "2": {
                "name": "Ethereum",
                "quotes": {
                    "USD": {
                        "price": 356.44,
                        "volume_24h": 2561930000.0,
                        "market_cap": 86690002871.0
                    }
                }
            },
            "3": {
                "name": "Ripple",
                "quotes": {
                    "USD": {
                        "price": 0.89,
                        "volume_24h": 761930000.0,
                        "market_cap": 16690002871.0
                    }
                }
            },
            "4": {
                "name": "Bitcoin Cash",
                "quotes": {
                    "USD": {
                        "price": 2849.22,
                        "volume_24h": 1930000.0,
                        "market_cap": 690002871.0
                    }
                }
            }
        }
    });

    let sort_attribute: String = String::from("price");
    let mut tree = Node { left_node: None, right_node: None, value: &value["data"]["2"] };
    tree.insert(&value["data"]["1"], &sort_attribute);
    tree.insert(&value["data"]["3"], &sort_attribute);
    tree.insert(&value["data"]["4"], &sort_attribute);
    let mut recursion_depth: u32 = 4;
    tree.inorder(&sort_attribute, &mut recursion_depth);

    assert!(tree == Node {
        value: &value["data"]["2"],
        left_node: Some(Box::new(Node {
            value: &value["data"]["3"],
            left_node: None,
            right_node: None,
        })),
        right_node: Some(Box::new(Node {
            value: &value["data"]["1"],
            left_node:
            Some(Box::new(Node { value: &value["data"]["4"], left_node: None, right_node: None })),
            right_node: None,
        })),
    });
}
