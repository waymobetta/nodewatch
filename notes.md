### nodewatch

### todo:

10/11

compare response from nodewatch API with static `country.json`
- potentially store this data in local db to avoid redundant calls
- read db data state on render while fetching new data in background


nodes vector:
```rust
vec![
    Node {
        name: "United States".to_string(),
        country_code: "US".to_string(),
        capital: "Washington, DC".to_string(),
        count: 950,
        coordinates: vec![38.0, -97.0],
    },
    Node {
        name: "United Kingdom".to_string(),
        country_code: "GB".to_string(),
        capital: "London".to_string(),
        count: 500,
        coordinates: vec![54.0, -2.0],
    },
]
```

10/15

create chart for clients
- clients.json
- bar



