use std::fs;
use std::path::PathBuf;
use axum::extract::{Multipart, Path, State};
use axum::response::{IntoResponse, Response};
use axum::body::Body;
use axum::http::StatusCode;
use axum::Json;
use tokio::fs::{File, OpenOptions};
use tokio_util::io::ReaderStream;
use futures::{StreamExt, TryFutureExt};
use sqlx::PgPool;
use tokio::io::AsyncWriteExt;
use serde_json::json;
use tokio_util::bytes::{Bytes, BytesMut};
use tracing::log::info;
use tracing::warn;

const BUFFER_SIZE: usize = 64 * 1024; // 64KB буфер

pub struct VideoStreamer;

impl VideoStreamer {
    async fn stream_file(
         file_path: PathBuf,
    ) -> Result<Response, std::io::Error> {
        let file = File::open(file_path).await?;
        let metadata = file.metadata().await?;
        let file_size = metadata.len();

        // Create a stream from the file
        let stream = ReaderStream::new(file);

        // Convert the stream to bytes and handle errors
        let body_stream = stream.map(|result| {
            result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });

        let body = Body::from_stream(body_stream);

        Ok(Response::builder()
            .header("Content-Type", "video/mp4")
            .header("Content-Length", file_size.to_string())
            .header("Accept-Ranges", "bytes")
            .header("Cache-Control", "no-cache")
            .body(body)
            .unwrap())
    }
}


pub async fn stream_video(
    Path(filename): Path<String>,
) -> Result<Response, (axum::http::StatusCode, String)> {
    let mut file_path = PathBuf::from("./videos").join(filename);
    file_path.set_extension(String::from("mp4"));
    if !file_path.exists() {
        return Err((axum::http::StatusCode::NOT_FOUND, "Video not found".to_string()));
    }

    VideoStreamer::stream_file(file_path).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}


// Хз надо ли
// pub async fn upload_video(
//     //State(pool): State<PgPool>,
//     mut multipart: Multipart,
// ) -> Result<Response, (axum::http::StatusCode, String)> {
//     while let Some(mut field) = multipart.next_field().await.unwrap() {
//         let key = field.name().unwrap_or("unknown").to_string();
//
//         match key.as_str() {
//             "video" => {
//                 // Обработка видео файла
//                 let data = field.bytes().await.map_err(|e| {
//                     eprintln!("Error reading video data: {}", e);
//                 }).unwrap();
//
//                 // Сохраняем файл
//                 tokio::fs::write("./uploaded_video.mp4", &data).await.expect("TODO: panic message"); //TODO: name fix
//                 println!("Video saved: {} bytes", data.len());
//             }
//             _ => {}
//         }
//     }
//     Err((StatusCode::BAD_REQUEST, "No video file provided".to_string()))
// }

pub async fn upload_video_chunked(
    mut multipart: Multipart,
) -> Result<Response, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        if field.name() == Some("video") {
            let unique_name = format!("{}.mp4", uuid::Uuid::new_v4());
            let file_path = format!("./uploads/{}", unique_name);

            let mut file = tokio::fs::File::create(file_path).await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let data = field.bytes().await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

            let total_size = data.len();

            const CHUNK_SIZE: usize = 1 * 1024 * 1024; // 1mb чанки
            let mut written = 0;

            while written < total_size {
                let end = std::cmp::min(written + CHUNK_SIZE, total_size);
                let chunk = &data[written..end];

                file.write_all(chunk).await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                written = end;

                //huenya
                if written % (10 * 1024 * 1024) == 0 {
                    info!("Progress: {}/{} mb", written/1024/1024, total_size/1024/1024);
                }
            }

            let response = json!({
                "status": "success",
                "file_name": unique_name,
                "size": total_size,
                "chunks_processed": (total_size + CHUNK_SIZE - 1) / CHUNK_SIZE
            });

            return Ok(Json(response).into_response());
        }
    }

    Err((StatusCode::BAD_REQUEST, "No video file provided".to_string()))
}