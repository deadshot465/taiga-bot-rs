use crate::shared::constants::ASSET_DIRECTORY;
use crate::shared::structs::Context;
use image::DynamicImage;
use image::imageops::{FilterType, overlay};
use std::io::{BufWriter, Cursor};

const HEART_FILE_NAME: &str = "/png/heart2.png";
const DEFAULT_IMAGE_HEIGHT: u32 = 150;
const DEFAULT_IMAGE_WIDTH: u32 = DEFAULT_IMAGE_HEIGHT * 3;

pub fn calculate_ship_score(user_id_1: u64, user_id_2: u64) -> u64 {
    if user_id_1 == user_id_2 {
        100
    } else {
        ((user_id_1 + user_id_2) / 7) % 100
    }
}

pub async fn download_avatar(ctx: Context<'_>, avatar_url: &str) -> anyhow::Result<Vec<u8>> {
    Ok(ctx
        .data()
        .http_client
        .get(avatar_url)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec())
}

pub fn generate_ship_image(avatar_1: &[u8], avatar_2: &[u8]) -> anyhow::Result<Vec<u8>> {
    let heart_path = String::from(ASSET_DIRECTORY) + HEART_FILE_NAME;
    let image_1 = image::load_from_memory(avatar_1)?.resize_exact(
        DEFAULT_IMAGE_HEIGHT,
        DEFAULT_IMAGE_HEIGHT,
        FilterType::Nearest,
    );
    let image_2 = image::load_from_memory(avatar_2)?.resize_exact(
        DEFAULT_IMAGE_HEIGHT,
        DEFAULT_IMAGE_HEIGHT,
        FilterType::Nearest,
    );
    let image_heart = image::open(heart_path)?.resize_exact(
        DEFAULT_IMAGE_HEIGHT,
        DEFAULT_IMAGE_HEIGHT,
        FilterType::Nearest,
    );

    let mut buffer = DynamicImage::new_rgba8(DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT);
    overlay(&mut buffer, &image_1, 0, 0);
    overlay(&mut buffer, &image_heart, DEFAULT_IMAGE_HEIGHT as i64, 0);
    overlay(&mut buffer, &image_2, (DEFAULT_IMAGE_HEIGHT * 2) as i64, 0);
    let length = buffer.as_bytes().len();
    let image = Vec::with_capacity(length);
    let mut writer = BufWriter::new(Cursor::new(image));
    buffer.write_to(&mut writer, image::ImageFormat::Png)?;
    Ok(writer.into_inner().unwrap_or_default().into_inner())
}

pub fn get_ship_message(ctx: Context<'_>, score: u64) -> String {
    ctx.data()
        .ship_messages
        .iter()
        .find(|msg| msg.max_score as u64 >= score)
        .map(|msg| msg.message.as_str())
        .unwrap_or_default()
        .to_string()
}

pub fn monochrome_if_lower_score(score: u64, url: String) -> String {
    if score > 50 {
        url.replace(".webp", ".png")
    } else {
        url
    }
}
