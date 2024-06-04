extern crate mpd;

use mpd::Client;

const HOST: &str = "romeo:6600";

fn main() {
    let mut conn = Client::connect(HOST).unwrap();
    let mut duration: u64 = 0; // u64 because that's what duration unwraps to
    let s = conn.status().unwrap();
    let lok: bool;
    let _state = match s.state {
        mpd::State::Stop => lok = false,
        mpd::State::Play => lok = true,
        mpd::State::Pause => lok = true,
    };
    // only get the rest of the information if there's information to fetch!
    if lok {
        let pos: u32 = s.song.unwrap().pos;
        let p = conn.queue().unwrap();
        let mut i: u32 = 0;
        let mut elapsed: u64 = s.time.unwrap().0.as_secs();
        for item in p {
            let time = item.duration.unwrap().as_secs();
            duration += time;
            if i < pos {
                elapsed += time;
            }
            i += 1;
        }
        let pc = ((elapsed as f32/duration as f32)*100.0) as f32;
        let pcs = format!("{:.2}", pc);
        println!("{pcs}");
    }
}
