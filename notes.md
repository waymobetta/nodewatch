# nodewatch

## todo:
&nbsp;  
## 10/11

&nbsp;  
[x] compare response from nodewatch API with static `country.json`

[x] store data in static `nodes.json`

[x] render data in world map

[ ] store data in local db to avoid redundant calls

[ ] read db data state on render while fetching new data in background

&nbsp;  
_example nodes vector:_
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

&nbsp;  
## 10/15

&nbsp;  
[x] add vim support (use without arrows)

[x] store clients data in static `clients.json`

[ ] render data in bar chart


