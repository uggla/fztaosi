use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    prelude::*,
};

pub async fn music(music_ready: &mut bool) {
    if !*music_ready {
        clear_background(BLACK);
        draw_text("Loading... Please wait...", 0.0, 35.0, 50.0, YELLOW);
        next_frame().await;
        let music = include_bytes!("../musics/HoliznaCC0 - Red Skies.ogg");
        let music = load_sound_from_bytes(music).await.unwrap();

        play_sound(
            music,
            PlaySoundParams {
                looped: true,
                volume: 1.0,
            },
        );
        *music_ready = true;
    }
}
