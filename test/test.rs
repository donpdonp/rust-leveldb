use leveldb;
use std;

import leveldb::*;
import std::io;

#[test]
fn hello() {
    alt open([create_if_missing], "/tmp/testdb") {
      either::right(r) {
        let secret = "leveldb";
        r.put([], "hello", secret);
        alt r.get([], "hello") {
          either::right(v) {
            assert v == secret;
            io::println("ok");
            r.close();
          }
        }
      }
    }
}
