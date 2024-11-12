use config::Config;

pub fn handle_sound_mappings(file_path: String) {
    let app_settings = Config::builder()
        .add_source(config::File::with_name(file_path.as_str()))
        .build()
        .expect("Unable to load sound mapping file");

    println!("{}", app_settings.get_string("test.key").unwrap());
}
