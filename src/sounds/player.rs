use soloud::{audio, AudioExt, LoadExt, Soloud};

pub fn play_sound_test() {
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

pub fn play_sound(path_to_sound: &str) {
    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new(path_to_sound)).unwrap();
    wav.set_looping(false);
    let h = sl.play(&wav);
    std::thread::sleep(std::time::Duration::from_millis(2000));
    sl.stop(h);
}

pub fn play_sound_once(soloud: &Soloud, path_to_sound: &str) {
    let mut wav = audio::Wav::default();
    wav.load(&std::path::Path::new(path_to_sound)).unwrap();
    wav.set_looping(false);
    let handle = soloud.play(&wav);
}
