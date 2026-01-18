use crate::api::error::AppError;
use crate::api::state::AppState;
use crate::banner::{apply_pattern, Banner, Pattern};
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::Response;
use image::RgbaImage;
use image::imageops::resize;
use serde::Deserialize;
use std::io::Cursor;

const CACHE_TIME: i32 = 30 * 24 * 60 * 60;

#[derive(Deserialize, Debug)]
pub struct GenerateQuery {
    pub width: Option<usize>,
}

pub async fn generate(
    State(app_state): State<AppState>,
    Path(banner_string): Path<String>,
    Query(params): Query<GenerateQuery>,
) -> Result<Response, AppError> {
    let banner_without_extension = banner_string
        .chars()
        .take_while(|&ch| ch != '.')
        .collect::<String>();

    let banner = Banner::parse_banner(&banner_without_extension)?;

    let mut image = RgbaImage::new(20, 40);
    let base_pattern_image = &app_state.patterns[Pattern::Base];

    let color = banner.color.to_rgb();

    apply_pattern(&mut image, base_pattern_image, color);

    for pattern in banner.patterns {
        let pattern_image = &app_state.patterns[pattern.pattern];

        apply_pattern(&mut image, pattern_image, pattern.color.to_rgb());
    }

    let image_width = params.width
        .unwrap_or(20)
        .clamp(20, 320) as u32;

    if image_width != 20 {
        image = resize(&image, image_width, image_width * 2, image::imageops::FilterType::Nearest);
    }

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, format!("public, max-age={}", CACHE_TIME))
        .body(buf.into())?)
}
