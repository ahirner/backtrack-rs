window.BENCHMARK_DATA = {
  "lastUpdate": 1658059461772,
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
      },
      {
        "commit": {
          "author": {
            "email": "6055037+ahirner@users.noreply.github.com",
            "name": "Alexander Hirner",
            "username": "ahirner"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32c8ea073696dcda5712816835429f7a7d0fe69a",
          "message": "Merge pull request #12 from ahirner/chore/hygiene\n\nChore/hygiene",
          "timestamp": "2022-07-17T13:59:04+02:00",
          "tree_id": "e5dfefc834c4b7002368e0f654fd6a7efdea3f84",
          "url": "https://github.com/ahirner/backtrack-rs/commit/32c8ea073696dcda5712816835429f7a7d0fe69a"
        },
        "date": 1658059461410,
        "tool": "cargo",
        "benches": [
          {
            "name": "n-queens 5",
            "value": 9770,
            "range": "± 629",
            "unit": "ns/iter"
          },
          {
            "name": "n-queens 9",
            "value": 3924491,
            "range": "± 174819",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/n-queens 10",
            "value": 19906902,
            "range": "± 635803",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/n-queens 11",
            "value": 103095231,
            "range": "± 1983377",
            "unit": "ns/iter"
          },
          {
            "name": "bit-queens 9",
            "value": 3524613,
            "range": "± 165063",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/bit-queens 10",
            "value": 19101065,
            "range": "± 1103987",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/bit-queens 11",
            "value": 95000383,
            "range": "± 3073354",
            "unit": "ns/iter"
          },
          {
            "name": "total-sum 3^3",
            "value": 1732,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "smaller-sample-size/total-sum 3^42",
            "value": 183813379,
            "range": "± 11043368",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}