use dotenvy::from_filename;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub rust_app_frontend_upload_path: String,
    pub rust_app_frontend_static_path: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    from_filename("env/frontend.env").expect("Cannot load env/frontend.env");
    envy::from_env::<Config>().expect("ENV format error")
});
