window.BENCHMARK_DATA = {
  "lastUpdate": 1658056672808,
  "repoUrl": "https://github.com/ahirner/backtrack-rs",
  "entries": {
    "backtrack-rs criterion": [
      {
        "commit": {
          "author": {
            "name": "ahirner",
            "username": "ahirner",
            "email": "a.hirner@gmail.com"
          },
          "committer": {
            "name": "ahirner",
            "username": "ahirner",
            "email": "a.hirner@gmail.com"
          },
          "id": "8c879820be37384ff41afe4b82d89e0ab880bc79",
          "message": "try bencher output from cargo-criterion",
          "timestamp": "2022-07-17T10:49:34Z",
          "url": "https://github.com/ahirner/backtrack-rs/commit/8c879820be37384ff41afe4b82d89e0ab880bc79"
        },
        "date": 1658056671577,
        "tool": "cargo",
        "benches": [
          {
            "name": "n-queens 5",
            "value": 7605,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "n-queens 9",
            "value": 3996035,
            "range": "± 6084",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/n-queens 10",
            "value": 19688636,
            "range": "± 15262",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/n-queens 11",
            "value": 104469893,
            "range": "± 89863",
            "unit": "ns/iter"
          },
          {
            "name": "bit-queens 9",
            "value": 3406045,
            "range": "± 14872",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/bit-queens 10",
            "value": 16484578,
            "range": "± 8088",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/bit-queens 11",
            "value": 88551879,
            "range": "± 52033",
            "unit": "ns/iter"
          },
          {
            "name": "total-sum 3^3",
            "value": 1822,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/total-sum 3^42",
            "value": 173307092,
            "range": "± 169432",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}