curl_commands

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetClientCounts {\n  aggregateByAgentName {\n    name\n    count\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"aggregateByAgentName":[{"name":"lighthouse","count":371},{"name":"lodestar","count":26},{"name":"prysm","count":3083},{"name":"others","count":16},{"name":"teku","count":318},{"name":"nimbus","count":167}]}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetNodeStats {\n  getNodeStats {\n    totalNodes\n    nodeSyncedPercentage\n    nodeUnsyncedPercentage\n  }\n}\n","variables":{"percentage":15}}' \
  --compressed
```

response:
```json
{"data":{"getNodeStats":{"totalNodes":3981,"nodeSyncedPercentage":85.50615423260487,"nodeUnsyncedPercentage":14.493845767395127}}}
```

request:
```bash
  curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetRegionalStats {\n  getRegionalStats {\n    totalParticipatingCountries\n    hostedNodePercentage\n    nonhostedNodePercentage\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"getRegionalStats":{"totalParticipatingCountries":60,"hostedNodePercentage":41.296156744536546,"nonhostedNodePercentage":58.703843255463454}}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw $'{"query":"query GetNodeStatsOverTime($start: Float\u0021, $end: Float\u0021) {\\n  getNodeStatsOverTime(start: $start, end: $end) {\\n    time\\n    totalNodes\\n    syncedNodes\\n    unsyncedNodes\\n  }\\n}\\n","variables":{"start":1633219180,"end":1633823980}}' \
  --compressed
  ```

  response:
  ```json
{"data":{"getNodeStatsOverTime":[{"time":1.6332192e+09,"totalNodes":3819,"syncedNodes":3304,"unsyncedNodes":515},{"time":1.6333056e+09,"totalNodes":3763,"syncedNodes":3260,"unsyncedNodes":503},{"time":1.633392e+09,"totalNodes":3832,"syncedNodes":3273,"unsyncedNodes":559},{"time":1.6334784e+09,"totalNodes":4035,"syncedNodes":3388,"unsyncedNodes":647},{"time":1.6335648e+09,"totalNodes":4144,"syncedNodes":3473,"unsyncedNodes":671},{"time":1.6336512e+09,"totalNodes":4180,"syncedNodes":3492,"unsyncedNodes":688},{"time":1.6337376e+09,"totalNodes":4044,"syncedNodes":3416,"unsyncedNodes":628}]}}
  ```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetHeatmap {\n  getHeatmapData {\n    networkType\n    clientType\n    syncStatus\n    latitude\n    longitude\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"getHeatmapData":[{"networkType":"residential","clientType":"prysm","syncStatus":"synced","latitude":43.1727,"longitude":5.6087},{"networkType":"hosting","clientType":"prysm","syncStatus":"synced","latitude":49.405,"longitude":11.1617},{"networkType":"","clientType":"prysm","syncStatus":"synced","latitude":51.2993,"longitude":9.491}]}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetOperatingSystems {\n  aggregateByOperatingSystem {\n    name\n    count\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"aggregateByOperatingSystem":[{"name":"linux","count":680},{"name":"unknown","count":3294},{"name":"mac","count":3},{"name":"windows","count":4}]}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetNetworks {\n  aggregateByNetwork {\n    name\n    count\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"aggregateByNetwork":[{"name":"","count":1003},{"name":"hosting","count":1644},{"name":"business","count":55},{"name":"residential","count":1264},{"name":"government","count":3},{"name":"education","count":12}]}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetNodesByCountries {\n  aggregateByCountry {\n    name\n    count\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"aggregateByCountry":[{"name":"Canada","count":102},{"name":"Austria","count":39},{"name":"Taiwan","count":16},{"name":"Thailand","count":3},{"name":"South Korea","count":43},{"name":"Argentina","count":1},{"name":"Ireland","count":154},{"name":"Belgium","count":28},{"name":"Italy","count":23},{"name":"Greece","count":3},{"name":"Spain","count":30},{"name":"Serbia","count":4},{"name":"Malaysia","count":1},{"name":"Lithuania","count":10},{"name":"Mexico","count":3},{"name":"Turkey","count":3},{"name":"Russia","count":40},{"name":"Peru","count":1},{"name":"Myanmar","count":1},{"name":"Hungary","count":3},{"name":"India","count":149},{"name":"Slovakia","count":5},{"name":"Slovenia","count":8},{"name":"China","count":27},{"name":"Singapore","count":67},{"name":"Indonesia","count":6},{"name":"New Zealand","count":4},{"name":"Australia","count":83},{"name":"Ukraine","count":6},{"name":"United States","count":950},{"name":"Norway","count":8},{"name":"United Kingdom","count":88},{"name":"Réunion","count":3},{"name":"Poland","count":22},{"name":"Finland","count":256},{"name":"Malta","count":4},{"name":"Japan","count":34},{"name":"Estonia","count":4},{"name":"Denmark","count":4},{"name":"Paraguay","count":1},{"name":"Bosnia and Herzegovina","count":1},{"name":"Oman","count":1},{"name":"Switzerland","count":44},{"name":"Portugal","count":12},{"name":"Germany","count":821},{"name":"Sweden","count":16},{"name":"South Africa","count":7},{"name":"Colombia","count":1},{"name":"Netherlands","count":101},{"name":"Israel","count":3},{"name":"Czechia","count":10},{"name":"Romania","count":5},{"name":"France","count":561},{"name":"Jersey","count":2},{"name":"Montenegro","count":1},{"name":"Vietnam","count":1},{"name":"Hong Kong","count":142},{"name":"Brazil","count":6},{"name":"Croatia","count":1},{"name":"Bulgaria","count":8}]}}
```

request:
```bash
curl 'https://api.nodewatch.io/query' \
  -H 'authority: api.nodewatch.io' \
  -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'accept: */*' \
  -H 'sec-gpc: 1' \
  -H 'origin: https://www.nodewatch.io' \
  -H 'sec-fetch-site: same-site' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-dest: empty' \
  -H 'referer: https://www.nodewatch.io/' \
  -H 'accept-language: en-US,en;q=0.9' \
  --data-raw '{"query":"query GetClientVersions {\n  aggregateByClientVersion {\n    client\n    count\n    versions {\n      name\n      count\n    }\n  }\n}\n"}' \
  --compressed
```

response:
```json
{"data":{"aggregateByClientVersion":[{"client":"teku","count":318,"versions":[{"name":"v21.5.0","count":6},{"name":"v21.6.0","count":2},{"name":"v21.4.1","count":1},{"name":"v21.6.1","count":5},{"name":"v21.9.2","count":129},{"name":"v21.7.0","count":8},{"name":"v21.8.1","count":11},{"name":"v21.8.2","count":86},{"name":"v21.8.0","count":4},{"name":"v21.1.0","count":1},{"name":"v21.9.0","count":3},{"name":"v21.2.0","count":1},{"name":"v21.9.1","count":56},{"name":"v21.4.0","count":2},{"name":"v21.1.1","count":2},{"name":"v20.11.0","count":1}]},{"client":"nimbus","count":167,"versions":[{"name":"unknown","count":167}]},{"client":"lodestar","count":26,"versions":[{"name":"0.32.5","count":23},{"name":"0.32.4","count":3}]},{"client":"lighthouse","count":371,"versions":[{"name":"v1.0.6","count":6},{"name":"v1.1.2","count":1},{"name":"v1.1.3","count":1},{"name":"v1.1.0","count":6},{"name":"v1.5.0","count":9},{"name":"v1.5.1","count":70},{"name":"v1.0.2","count":7},{"name":"v1.2.0","count":1},{"name":"v1.4.0","count":35},{"name":"v1.0.0","count":2},{"name":"v2.0.0","count":125},{"name":"v1.3.0","count":29},{"name":"v1.2.2","count":1},{"name":"v1.0.1","count":1},{"name":"v1.5.2","count":64},{"name":"v1.0.3","count":8},{"name":"v1.0.5","count":1},{"name":"v1.0.4","count":4}]},{"client":"others","count":16,"versions":[{"name":"0.31.0","count":13},{"name":"unknown","count":3}]},{"client":"prysm","count":3083,"versions":[{"name":"v1.3.11","count":33},{"name":"v1.4.1","count":12},{"name":"v1.3.2","count":3},{"name":"Unknown","count":9},{"name":"v1.3.8","count":24},{"name":"v1.1.0","count":9},{"name":"v1.3.9","count":15},{"name":"v1.4.2","count":1147},{"name":"v1.3.5","count":2},{"name":"v1.3.7","count":6},{"name":"v1.4.3","count":393},{"name":"v1.0.1","count":1},{"name":"v1.2.1","count":3},{"name":"v1.3.3","count":3},{"name":"v1.0.4","count":1},{"name":"v1.3.10","count":13},{"name":"v2.0.0","count":406},{"name":"v1.2.2","count":1},{"name":"v0.1.0","count":14},{"name":"v2.0.1","count":509},{"name":"v1.2.0","count":2},{"name":"v1.4.4","count":477}]}]}}
```
