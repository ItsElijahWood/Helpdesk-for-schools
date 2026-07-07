use std::{fs::File, io::BufReader};

use actix_web::{HttpResponse, HttpResponseBuilder, Result};
use awc::http::StatusCode;
use rodio::{Decoder, DeviceSinkBuilder, Player};

use crate::app::ErrorTypes;

pub async fn play() -> Result<HttpResponse, ErrorTypes> {
    let notify_name =
        std::env::var("FS_NOTIFY_SOUND").expect("failed to find FS_NOTIFY_SOUND in .env file.");

    let mut sink_handle = DeviceSinkBuilder::open_default_sink()
        .expect("failed to open os sink for default audio output stream.");
    sink_handle.log_on_drop(false);

    let player = Player::connect_new(&sink_handle.mixer());

    let file_name = format!("assets/media/{}", notify_name);
    let file =
        BufReader::new(File::open(file_name).expect("failed to load sound file into buffer."));
    let source = Decoder::new(file).expect("failed to decode audio.");

    player.append(source);
    player.sleep_until_end();

    Ok(HttpResponseBuilder::new(StatusCode::OK).finish())
}
