// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ringbuf::{DynRingBuf};

fn main() {
    let mut buf: DynRingBuf<_> = DynRingBuf::new();
    for i in 0..64 {
        if i % 2 == 0 {
            buf.push_left(i);
        } else {
            buf.push_right(i);
        }
    }
    println!("{:?}", buf);
    for el in &buf {
        print!("{:?}, ", el);
    }
}
