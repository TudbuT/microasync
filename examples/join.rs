use microasync::*;

fn main() {
    println!("{:?}", sync(join!(fn1(), fn2())));
}

async fn fn1() -> i32 {
    join!(async { 100 }, async { 5 }).await.into_iter().sum()
}

async fn fn2() -> i32 {
    fn1().await + 1
}
