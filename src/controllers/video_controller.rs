use std::path::PathBuf;
use axum::extract::Path;
use axum::response::Response;
use axum::body::Body;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use futures::StreamExt;

pub struct VideoStreamer;

impl VideoStreamer {
    async fn stream_file(
        mut file_path: PathBuf,
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