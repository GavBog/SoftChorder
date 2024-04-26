use chorder::{config::parse_chords, utils::manipulate_text_buffer};
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        listen(move |event| {
            let Event {
                event_type, name, ..
            } = event;
            if let EventType::KeyPress(_) = event_type {
                let name = name.unwrap().parse();
                if let Ok(name) = name {
                    tx.send(name)
                        .unwrap_or_else(|e| eprintln!("Failed to send event {:?}", e));
                }
            }
        })
        .expect("Failed to attach listener");
    });

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let chrods = parse_chords("chords.json").await;

    let mut text_buffer = String::new();
    let time = tokio::time::Duration::from_millis(20);
    let timeout = tokio::time::sleep(time);
    tokio::pin!(timeout);

    loop {
        let key = rx.recv().await.unwrap();
        if timeout.is_elapsed() {
            text_buffer.clear();
        }
        timeout.as_mut().reset(tokio::time::Instant::now() + time);
        manipulate_text_buffer(key, &mut text_buffer);

        if chrods.contains_key(&text_buffer) {
            tokio::time::sleep(time).await;
        }
        while let Ok(key) = rx.try_recv() {
            manipulate_text_buffer(key, &mut text_buffer);
            if chrods.contains_key(&text_buffer) {
                tokio::time::sleep(time).await;
            }
        }

        if let Some(phrase) = chrods.get(&text_buffer) {
            // TODO: Change this Later
            let phrase = phrase.replace("KEY_", "").to_lowercase();
            println!("Wrote: {}", phrase);
            for _ in text_buffer.chars() {
                enigo.key(Key::Backspace, Click).unwrap();
            }
            enigo.text(&format!("{} ", phrase)).unwrap();
            text_buffer.clear();
        }
    }
}
