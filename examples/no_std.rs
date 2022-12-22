// we don't actually use #![no_std] to be able to compile this. 
// make sure to add --features no_std.

use microasync::sync;

fn main() {
    for _ in 0..10000000 {
        sync(test());
    }
}

async fn test() {}
