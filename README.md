# microasync

MicroAsync is a tiny async "runtime" for rust, created when I was very bothered by a
library that was fully async, but my code was all written synchronously.

## Use-case

Let's say you have a sync function here, and there's an async function you want to run,
but oh no! It doesn't work, because the function is async and your function isn't!

**Here's where microasync::sync comes into play.**

It synchronizes a single async function, returning its result as if it was a normal
function. For this, a *tiny*, single-threaded async "runtime" is created, that runs this
one task, and then stops.

## Example

```rs
use microasync::sync;

fn main() {
    //println!("{}", do_sth_async(1000).await);

    println!("{}", sync(do_sth_async(1000)));
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

async fn do_sth_async(i: i32) -> i32 {
    add_async(i, i * 4).await
}
```
