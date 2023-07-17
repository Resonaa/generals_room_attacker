use generals_room_attacker::{socket::new_bot, BOT_DATA};
use std::thread;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    for bot_data in BOT_DATA.iter() {
        thread::spawn(|| new_bot(bot_data));
    }

    loop {
        thread::park();
    }
}
