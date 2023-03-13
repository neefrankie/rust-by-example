use futures::{executor::block_on, future};

fn main() {
    let future = hello_world();
    println!("Called hello_world");
    block_on(future);
    println!("Done");

    block_on(async_main());
}

async fn hello_world() {
    println!("Hello, world!");
}

struct Song {
    title: String,
}

async fn learn_song() -> Song {
    println!("Learn a song");
    return Song {
        title: String::from("Hotel California"),
    };
}

async fn sing_song(song: Song) {
    println!("Sing song {}", song.title);
}

async fn dance() {
    println!("Dancing");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}
