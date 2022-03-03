use discord_rpc_client::Client;
use fork::{daemon, Fork};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, process, thread, time};

const ID: u64 = 948690008406102026;
const MESSAGE: &str = "Either AFK or working";

fn main() {
    if let Ok(Fork::Child) = daemon(false, false) {
        let pid = process::id();
        fs::write("/tmp/discord-idle-status.pid", pid.to_string()).unwrap();

        let mut client = Client::new(ID);

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        client.start();

        client
            .set_activity(|act| {
                act.state(MESSAGE)
                    .timestamps(|timestamps| timestamps.start(current_time))
            })
            .unwrap();

        loop {
            // keep activity running
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}
