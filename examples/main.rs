use microasync::sync;

// Let's say you have a sync function here:
fn main() {
    // And there's an async function you want to run:
    //println!("{}", do_sth_async(1000).await);
    // but oh no! this doesn't work, because do_sth_async is async and main is sync!

    // Here's where microasync::sync comes into play:
    println!("{}", sync(do_sth_async(1000)).unwrap());
    // It synchronizes a single async function, returning its result as if it was a normal
    // function. For this, a *tiny*, single-threaded async "runtime" is created, that runs this one
    // task, and then stops.
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

async fn do_sth_async(i: i32) -> i32 {
    add_async(i, i * 4).await
}
