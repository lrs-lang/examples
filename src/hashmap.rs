// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};
use std::hashmap::{CompactMap, HashMap, Entry};
use std::hash::{Hasher};
use std::hash::xx_hash::{XxHash32, XxHash64};

const MAX_VAL: u32 = 10000000;
const MASK: u64 = 131072;

fn main() {
    let mut map: CompactMap<_, _> = CompactMap::new().unwrap();
    for i in 0xE000u32 .. 0x110000 {
        let c = char::from_u32(i).unwrap();
        map.set(c, c);
    }
}
