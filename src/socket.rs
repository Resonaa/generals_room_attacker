use crate::{BotData, CONFIG};
use rust_socketio::ClientBuilder;
use std::iter::repeat_with;

macro_rules! strings {
    ($first: expr, $( $x: expr ),*) => {
        {
            let mut ans = String::from($first);
            $(
                ans.push_str(&format!("\",\"{}", $x));
            )*
            ans
        }
    };
}

pub fn new_bot(config: &'static BotData) {
    let user_id: String = repeat_with(fastrand::alphanumeric).take(8).collect();

    ClientBuilder::new(CONFIG.base_url)
        .on("open", move |_, socket| {
            info!("connected");
            socket
                .emit("join_private", strings!(config.room, user_id, CONFIG.nbk))
                .unwrap();

            if config.team != 0 {
                socket
                    .emit("set_custom_team", strings!(config.room, config.team))
                    .unwrap();
            }
        })
        .on("close", move |_, _| error!("disconnected"))
        .connect()
        .unwrap();
}
