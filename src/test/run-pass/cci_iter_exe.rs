// aux-build:cci_iter_lib.rs

use std;
use cci_iter_lib;

import std::io;

fn main() {
    //let bt0 = sys::rusti::frame_address(1u32);
    //#debug["%?", bt0];
    cci_iter_lib::iter([1, 2, 3]) {|i|
        io::print(#fmt["%d", i]);
        //assert bt0 == sys::rusti::frame_address(2u32);
    }
}
