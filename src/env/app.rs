use dotenvy::from_filename;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rust_app_host: String,
    pub rust_app_port: u16,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    from_filename("env/app.env").expect("Cannot load env/app.env");
    envy::from_env::<Config>().expect("ENV format error")
});
