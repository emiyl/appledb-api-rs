# AppleDB Rust API Script

```js
OsEntry {
  osStr: String,
  version: String,
  safariVersion: Array<String>,
  build: String,
  key: String,
  embeddedOSBuild: String,
  bridgeOSBuild: String,
  buildTrain: String,
  released: String,
  rc: Boolean,
  beta: Boolean,
  rsr: Boolean,
  internal: Boolean,
  preinstalled: Array<String>,
  notes: String,
  releaseNotes: String,
  securityNotes: String,
  ipd: Object<String, String>,
  appledb: {
    webImage: {
          id: String,
          align: String,
      },
      webUrl: String,
      apiUrl: String,
      hideFromLatestVersions: Boolean
  },
  deviceMap: Array<String>,
  osMap: Array<String>,
  sources: Vec< {
      type: String,
      prerequisiteBuild: Array<String>,
      deviceMap: Array<String>,
      osMap: Array<String>,
      windowsUpdateDetails: {
          updateId: String,
          revisionId: String
      },
      links: Array<{
          url: String,
          active: Boolean
      }>,
      hashes: Object<String, String>,
      skipUpdateLinks: Boolean,
      size: Number
  }>
}

DeviceEntry {
    name: String,
    key: String,
    type: String,
    identifier: Array<String>,
    model: Array<String>,
    board: Array<String>,
    released: Array<String>,
    soc: Array<String>,
    arch: String,
    internal: Boolean,
    alias: Array<String>,
    info: Value,
    iBridge: String,
    group: Boolean,
    windowsStoreId: String,
}
```
