# AppleDB Rust API Script

```js
OsEntry {
  os_str: String,
  version: String,
  safari_version: Array<String>,
  build: String,
  key: String,
  embeddedOS_build: String,
  bridgeOS_build: String,
  build_train: String,
  released: String,
  rc: Boolean,
  beta: Boolean,
  rsr: Boolean,
  internal: Boolean,
  preinstalled: Array<String>,
  notes: String,
  release_notes: String,
  security_notes: String,
  ipd: Object<String, String>,
  appledb_web: {
    web_image: {
          id: String,
          align: String,
      },
      web_url: String,
      api_url: String,
      hide_from_latest_versions: Boolean
  },
  device_map: Array<String>,
  os_map: Array<String>,
  sources: Array< {
      type: String,
      prerequisite_build: Array<String>,
      device_map: Array<String>,
      os_map: Array<String>,
      windows_update_details: {
          updateId: String,
          revisionId: String
      },
      links: Array<{
          url: String,
          active: Boolean
      }>,
      hashes: Object<String, String>,
      skip_update_links: Boolean,
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
    windows_store_id: String,
}

JailbreakEntry {
    name: String,
    key: String,
    alias: Array<String>,
    priority: Value,
    hide_from_guide: Boolean,
    info: {
        website: {
            name: String,
            url: String
        },
        wiki: {
            name: String,
            url: String
        },
        guide: Array<{
            name: String,
            url: String,
            pkgman: String,
            update_link: Array<{
                name: String,
                url: String
            }>
        }>,
        latest_ver: String,
        color: String,
        icon: String,
        notes: String,
        jailbreaksmeapp: Boolean,
        type: String,
        firmwares: Array<String>,
        soc: String
    },
    compatibility: Array<{
        firmwares: Array<String>,
        devices: Array<String>
    }>
}

BypassEntry {
    name: String,
    bundle_id: String,
    uri: String,
    icon: String,
    notes: String,
    bypasses: Array<{
        name: String,
        notes: String,
        version: String,
        guide: String,
        repository: {
            uri: String
        },
    }>
}
```
