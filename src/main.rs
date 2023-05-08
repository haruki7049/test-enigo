use enigo::{Enigo, Key, KeyboardControllable};
use std::{thread::*, time::Duration};

fn main() {
    println!("standby...");
    sleep(Duration::from_secs(1));

    let mut enigo = Enigo::new();

    enigo.key_down(Key::Layout('a'));
    sleep(Duration::from_secs(1));
    enigo.key_up(Key::Layout('a'));
}
