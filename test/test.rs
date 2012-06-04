use leveldb;
use std;

import leveldb::*;
import core::io;

#[test]
fn hello() {
    alt open([create_if_missing], "/tmp/testdb") {
      result::ok(r) {
        let secret = "leveldb";
        r.put([], "hello", secret);
        alt r.get([], "hello") {
          result::ok(v) {
            assert v == secret;
            io::println("ok");
            r.close();
          }
          result::err(e) {
            io::println(#fmt("error: %s", e));
          }
        }
      }
      result::err(e) {
        io::println(#fmt("error: %s", e));
      }
    }
}
