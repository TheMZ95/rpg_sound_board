use soloud::{audio, AudioExt, LoadExt, Soloud};

pub fn play_sound() {
    let mut sl = Soloud::default().unwrap();

    let mut wav = audio::Wav::default();

    wav.load(&std::path::Path::new("Feuer 01.mp3")).unwrap();

    let h = sl.play(&wav);
    std::thread::sleep(std::time::Duration::from_millis(2000));
    sl.stop(h);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
