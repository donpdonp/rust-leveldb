#[link(name = "leveldb",
       vers = "0.1",
       uuid = "122bed0b-c19b-4b82-b0b7-7ae8aead7297",
       url = "http://github.com/lht/rust-leveldb")];

#[comment = "Rust binding for LevelDB"];
#[license = "BSD"];
// #[crate_type = "lib"];

use std;

import core::result;
import core::ptr::{addr_of};
import core::libc::{c_int, c_uint, size_t};

export open;
export db;
export option, read_option, write_option;

iface db {
    fn get(ropts: read_options, key: str) -> result::result<str, str>;
    fn put(opts: write_options, key: str, val: str);
    fn delete(opts: write_options, key: str);
    fn write(opts: write_options, wb: write_batch);
    fn close();
}

fn open(opts: options, name: str) -> result::result<db, str> unsafe {
    let copts = to_c_options(opts);
    let err : *u8 = ptr::null();
    ret str::as_buf(name) {|cname|
        let r = leveldb::leveldb_open(copts, cname, ptr::addr_of(err));
        if r == ptr::null() {
            result::err(str::unsafe::from_buf(err))
        } else {
            result::ok(r as db)
        }
    };
}

enum leveldb_t {}
enum leveldb_cache_t {}
enum leveldb_comparator_t {}
enum leveldb_env_t {}
enum leveldb_filelock_t {}
enum leveldb_iterator_t {}
enum leveldb_logger_t {}
enum leveldb_options_t {}
enum leveldb_randomfile_t {}
enum leveldb_readoptions_t {}
enum leveldb_seqfile_t {}
enum leveldb_snapshot_t {}
enum leveldb_writablefile_t {}
enum leveldb_writebatch_t {}
enum leveldb_writeoptions_t {}

#[link_args="-lpthread -lstdc++ -Wl,--no-as-needed -lsnappy"]
native mod leveldb {
    // DB Operations

    fn leveldb_open(
        opts: *leveldb_options_t,
        name: *u8,
        errptr: **u8
    ) -> *leveldb_t;

    fn leveldb_close(db: *leveldb_t);

    fn leveldb_get(
        db: *leveldb_t,
        opts: *leveldb_readoptions_t,
        key: *u8,
        klen: size_t,
        vlen: *size_t,
        errptr: **u8
    ) -> *u8;

    fn leveldb_put(
        db: *leveldb_t,
        opts: *leveldb_writeoptions_t,
        key: *u8,
        klen: size_t,
        val: *u8,
        vlen: size_t,
        errptr: **u8
    );

    fn leveldb_delete(
        db: *leveldb_t,
        opts: *leveldb_writeoptions_t,
        key: *u8,
        klen: size_t,
        errptr: **u8
    );

    fn leveldb_write(
        db: *leveldb_t,
        opts: *leveldb_writeoptions_t,
        wb: *leveldb_writebatch_t,
        errptr: **u8
    );

    fn leveldb_create_iterator(
        db: *leveldb_t,
        opts: *leveldb_readoptions_t
    ) -> *leveldb_iterator_t;

    fn leveldb_create_snapshot(
        db: *leveldb_t
    ) -> *leveldb_snapshot_t;

    fn leveldb_release_snapshot(
        db: *leveldb_t,
        snapshot: *leveldb_snapshot_t
    );

    fn leveldb_property_value(
        db: *leveldb_t,
        propname: *u8
    );

    fn leveldb_approximate_sizes(
        db: *leveldb_t,
        num_ranges: c_int,
        range_start_key: **u8,
        range_start_key_len: *size_t,
        range_limit_key: **u8,
        range_limit_key_len: *size_t,
        sizes: *u64
    );

    // Management operations
    fn leveldb_destroy_db(
        opts: *leveldb_options_t,
        name: *u8,
        errptr: **u8);

    fn leveldb_repair_db(
        opts: *leveldb_options_t,
        name: *u8,
        errptr: **u8);

    // Iterator

    fn leveldb_iter_destroy(it: *leveldb_iterator_t);
    fn leveldb_iter_valid(it: *leveldb_iterator_t) -> u8;
    fn leveldb_iter_seek_to_first(it: *leveldb_iterator_t);
    fn leveldb_iter_seek_to_last(it: *leveldb_iterator_t);
    fn leveldb_iter_seek(it: *leveldb_iterator_t,
                         key: *u8, klen: size_t);
    fn leveldb_iter_next(it: *leveldb_iterator_t);
    fn leveldb_iter_prev(it: *leveldb_iterator_t);
    fn leveldb_iter_key(it: *leveldb_iterator_t,
                        klen: size_t) -> *u8;
    fn leveldb_iter_value(it: *leveldb_iterator_t,
                          vlen: *size_t) -> *u8;
    fn leveldb_iter_get_error(it: *leveldb_iterator_t, errptr: **u8);

    // Write batch

    fn leveldb_writebatch_create() -> *leveldb_writebatch_t;
    fn leveldb_writebatch_destroy(wb: *leveldb_writebatch_t);
    fn leveldb_writebatch_clear(wb: *leveldb_writebatch_t);
    fn leveldb_writebatch_put(
        wb: *leveldb_writebatch_t,
        key: *u8, klen: size_t,
        val: *u8, klen: size_t);
    fn leveldb_writebatch_delete(
        wb: *leveldb_writebatch_t,
        key: *u8, klen: size_t);

    /* FIXME: how to passing function pointers?
    fn leveldb_writebatch_iterate(
        wb: *leveldb_writebatch_t, state: *u8,
        void (*put)(void*, const char* k, size_t klen,
        const char* v, size_t vlen),
        void (*deleted)(void*, const char* k, size_t klen));
        put: *u8, delete: *u8);
    */

    // options
    fn leveldb_options_create() -> *leveldb_options_t;
    fn leveldb_options_destroy(opts: *leveldb_options_t);
    fn leveldb_options_set_comparator(
        opts: *leveldb_options_t, c: *leveldb_comparator_t);
    fn leveldb_options_set_create_if_missing(
        opts: *leveldb_options_t, x: u8);
    fn leveldb_options_set_error_if_exists(
        opts: *leveldb_options_t, x: u8);
    fn leveldb_options_set_paranoid_checks(
        opts: *leveldb_options_t, x: u8);
    fn leveldb_options_set_env(
        opts: *leveldb_options_t, env: *leveldb_env_t);
    fn leveldb_options_set_info_log(
        opts: *leveldb_options_t, g: *leveldb_logger_t);
    fn leveldb_options_set_write_buffer_size(
        opts: *leveldb_options_t, x: size_t);
    fn leveldb_options_set_max_open_files(
        opts: *leveldb_options_t, x: c_int);
    fn leveldb_options_set_block_size(
        opts: *leveldb_options_t, x: size_t);
    fn leveldb_options_set_block_restart_interval(
        opts: *leveldb_options_t, x: c_int);
    fn leveldb_options_set_compression(
        opts: *leveldb_options_t, z: c_int);

    // read options
    fn leveldb_readoptions_create() -> *leveldb_readoptions_t;
    fn leveldb_readoptions_destroy(ropts: *leveldb_readoptions_t);
    fn leveldb_readoptions_set_verify_checksums(
        ropts: *leveldb_readoptions_t, v: u8);
    fn leveldb_readoptions_set_fill_cache(
        ropts: *leveldb_readoptions_t, v: u8);
    fn leveldb_readoptions_set_snapshot(
        ropts: *leveldb_readoptions_t,
        snapshot: *leveldb_snapshot_t);

    // write options
    fn leveldb_writeoptions_create() -> *leveldb_writeoptions_t;
    fn leveldb_writeoptions_destroy(opts: *leveldb_writeoptions_t);
    fn leveldb_writeoptions_set_sync( opts: *leveldb_writeoptions_t, v: u8);

    /* Cache */

    fn leveldb_cache_create_lru(capacity: size_t) -> *leveldb_cache_t;
    fn leveldb_cache_destroy(cache: *leveldb_cache_t);

    /* Env */

    fn leveldb_create_default_env() -> *leveldb_env_t;
    fn leveldb_env_destroy(env: *leveldb_env_t);
}

type db_ = *leveldb_t;
type opts = leveldb_options_t;
type read_optioin = *leveldb_readoptions_t;
type wopts = leveldb_writeoptions_t;
type write_batch = *leveldb_writebatch_t;

// type compression_type = int;

const no_compression :c_int = 0i32;
const snappy_compression :c_int = 1i32;

enum option {
    create_if_missing,
    error_if_exists,
    paranoid_checks,
    // env,
    // log,
    write_buffer_size(size_t),
    max_open_files(c_int),
    // block_cache(),
    block_size(size_t),
    block_restart_interval(c_int),
    compression(c_int),
}

type options = [option];

type snapshot = *leveldb_snapshot_t;

enum read_option {
    verify_checksum,
    full_cache,
    use_snapshot(snapshot),
}
type read_options = [read_option];

enum write_option {
    sync,
}
type write_options = [write_option];

fn to_c_options(opts: options) -> *leveldb_options_t {
    let copts = leveldb::leveldb_options_create();
    for vec::each(opts) { |o|
        alt o {
          create_if_missing {
            leveldb::leveldb_options_set_create_if_missing(copts, 1u8);
          }
          error_if_exists {
            leveldb::leveldb_options_set_error_if_exists(copts, 1u8);
          }
          paranoid_checks {
            leveldb::leveldb_options_set_paranoid_checks(copts, 1u8);
          }
          // env;
          // log;
          write_buffer_size(sz) {
            leveldb::leveldb_options_set_write_buffer_size(copts, sz);
          }
          max_open_files(num) {
            leveldb::leveldb_options_set_max_open_files(copts, num);
          }
          // block_cache();
          block_size(sz) {
            leveldb::leveldb_options_set_block_size(copts, sz);
          }
          block_restart_interval(int) {
            leveldb::leveldb_options_set_block_restart_interval(copts, int);
          }
          compression(ct) {
            leveldb::leveldb_options_set_compression(copts, ct);
          }
        }
    }
    ret copts;
}

fn to_c_readoptions(opts: read_options)
    -> *leveldb_readoptions_t {
    let copts = leveldb::leveldb_readoptions_create();
    for vec::each(opts) { |o|
        alt o {
          verify_checksum {
            leveldb::leveldb_readoptions_set_verify_checksums(copts, 1u8);
          }
          full_cache {
            leveldb::leveldb_readoptions_set_fill_cache(copts, 1u8);
          }
          use_snapshot(snapshot) {
            leveldb::leveldb_readoptions_set_snapshot(copts, snapshot);
          }
        }
    }
    ret copts;
}

fn to_c_writeoptions(opts: write_options)
    -> *leveldb_writeoptions_t {
    let copts = leveldb::leveldb_writeoptions_create();
    for vec::each(opts) { |o|
        alt o {
          sync {
            leveldb::leveldb_writeoptions_set_sync(copts, 1u8);
          }
        }
    }
    ret copts;
}

impl of db for db_ {
    fn get(ropts: read_options, key: str)
        -> result::result<str, str> unsafe {
        let vlen: size_t = 0 as size_t;
        let err: *u8 = ptr::null();
        let copts = to_c_readoptions(ropts);
        ret str::as_buf(key) {|kb|
            let r = leveldb::leveldb_get(
                self, copts, kb, str::len(key) as size_t,
                ptr::addr_of(vlen), ptr::addr_of(err));
            if r == ptr::null() {
                result::err(str::unsafe::from_buf(err))
            } else {
                result::ok(str::unsafe::from_buf(r))
            }
        };
    }

    fn put(opts: write_options, key: str, val: str) unsafe {
        let copts = to_c_writeoptions(opts);
        let err: *u8 = ptr::null();
        str::as_buf(key) {|bk|
            str::as_buf(val) {|bv|
                leveldb::leveldb_put(
                    self, copts,
                    bk, str::len(key) as size_t,
                    bv, str::len(val) as size_t,
                    ptr::addr_of(err));
            }
        }
        if err != ptr::null() {
            fail str::unsafe::from_buf(err);
        }
    }

    fn delete(opts: write_options, key: str) unsafe {
        let copts = to_c_writeoptions(opts);
        let err: *u8 = ptr::null();
        str::as_buf(key) {|bk|
            leveldb::leveldb_delete(
                self, copts,
                bk, str::len(key) as size_t,
                ptr::addr_of(err));
        }
        if err != ptr::null() {
            fail str::unsafe::from_buf(err);
        }
    }

    fn write(opts: write_options, wb: write_batch) unsafe {
        let copts = to_c_writeoptions(opts);
        let err: *u8 = ptr::null();
        leveldb::leveldb_write(self, copts, wb, ptr::addr_of(err));
        if err != ptr::null() {
            fail str::unsafe::from_buf(err);
        }
    }

    fn close() {
        leveldb::leveldb_close(self);
    }
}
