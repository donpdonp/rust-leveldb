Rust binding for LevelDB
http://code.google.com/p/leveldb/

```rust
use leveldb;

import leveldb::*;

fn main() {
  alt leveldb::open([create_if_missing], "sample.leveldb") {
    result::ok(db) {
      db.put([], "key", "value");
      let value = db.get([], "key");
      io::println(#fmt("value for key is \"%s\"", result::get(value)));
    }
    result::err(e) {
      io::println(#fmt("open error: %s", e));
    }
  }
}
```
