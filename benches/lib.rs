// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md

#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences)]

#![feature(test)]

extern crate rand;
extern crate test;
extern crate rusqlite;
// extern crate maidsafe_utilities;

use rusqlite::Connection;

use rand::Rng;
use test::Bencher;
// use maidsafe_utilities::serialisation;

fn generate_random_bytes(size: u64) -> Vec<u8> {
    rand::thread_rng()
        .gen_iter()
        .take(size as usize)
        .collect()
}

// #[bench]
// fn bench_write(b: &mut Bencher) {
//     let one_mb = 1024 * 1024;
//     let data = generate_random_bytes(one_mb);

//     let conn = Connection::open("./mydb.db").unwrap();

//     let _ = conn.execute("CREATE TABLE chunk_store (
//                   id              INTEGER PRIMARY KEY,
//                   chunk           TEXT NOT NULL
//                   )", &[]).unwrap();

//     b.iter(|| {
//         let key = rand::thread_rng().gen_range(0, 10000);
//         let _ = conn.execute("INSERT INTO chunk_store (id, chunk)
//                       VALUES (?1, ?2)",
//                      &[&key, &data]);
//     });
// }

// #[bench]
// fn bench_write(b: &mut Bencher) {
//     let one_mb = 1024 * 1024;
//     let data = generate_random_bytes(one_mb);

//     let conn = Connection::open_in_memory().unwrap();

//     let _ = conn.execute("CREATE TABLE chunk_store (
//                   id              INTEGER PRIMARY KEY,
//                   chunk           TEXT NOT NULL
//                   )", &[]).unwrap();

//     b.iter(|| {
//         let key = rand::thread_rng().gen_range(0, 10000);
//         let serialised_value = match serialisation::serialise(&data) {
//           Ok(result) => result,
//           Err(_) => return,
//         };
//         let _ = conn.execute("INSERT INTO chunk_store (id, chunk)
//                       VALUES (?1, ?2)",
//                      &[&key, &serialised_value]);
//     });
// }

#[bench]
fn bench_read(b: &mut Bencher) {
    let one_mb = 1024 * 1024;
    let data = generate_random_bytes(one_mb);

    // let conn = Connection::open_in_memory().unwrap();
    let conn = Connection::open("./mydb.db").unwrap();

    let _ = conn.execute("CREATE TABLE chunk_store (
                  id              INTEGER PRIMARY KEY,
                  chunk           TEXT NOT NULL
                  )", &[]).unwrap();
    for i in 0..300 {
      let _ = conn.execute("INSERT INTO chunk_store (id, chunk)
                    VALUES (?1, ?2)",
                   &[&i, &data]);
    }

    b.iter(|| {
        let key = rand::thread_rng().gen_range(0, 300);
        let _: Vec<u8> = conn.query_row("SELECT chunk FROM chunk_store WHERE id == ?1", &[&key], |row| row.get(0)).unwrap();
        // assert_eq!(data, result);
    });
}

// #[bench]
// fn bench_read(b: &mut Bencher) {
//     let one_mb = 1024 * 1024;
//     let data = generate_random_bytes(one_mb);
//     let serialised_value = match serialisation::serialise(&data) {
//       Ok(result) => result,
//       Err(_) => return,
//     };

//     // let conn = Connection::open_in_memory().unwrap();
//     let conn = Connection::open("./mydb.db").unwrap();

//     let _ = conn.execute("CREATE TABLE chunk_store (
//                   id              INTEGER PRIMARY KEY,
//                   chunk           TEXT NOT NULL
//                   )", &[]).unwrap();
//     for i in 0..300 {
//       let _ = conn.execute("INSERT INTO chunk_store (id, chunk)
//                     VALUES (?1, ?2)",
//                    &[&i, &serialised_value]);
//     }

//     b.iter(|| {
//         let key = rand::thread_rng().gen_range(0, 300);
//         let result: Vec<u8> = conn.query_row("SELECT chunk FROM chunk_store WHERE id == ?1", &[&key], |row| row.get(0)).unwrap();
//         let _ = serialisation::deserialise::<Vec<u8>>(&result);
//         // assert_eq!(data, result);
//     });
// }
