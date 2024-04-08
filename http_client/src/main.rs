use bytes::Bytes;
use futures_util::Stream;
use reqwest::{Body, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let file = request_file(&client, "https://wallpapercave.com/wp/wp5111087.jpg").await?;
    let response = upload_file(&client, file).await?;

    println!("{}", response);

    Ok(())
}

struct FileStream<S> {
    filename: String,
    content_length: u64,
    stream: S,
}

async fn request_file(
    client: &Client,
    url: &str,
) -> reqwest::Result<FileStream<impl Stream<Item = reqwest::Result<Bytes>>>> {
    let response = client.get(url).send().await?;

    Ok(FileStream {
        filename: response
            .url()
            .path_segments()
            .map(|segment| segment.last().unwrap_or("tmp.bin").to_string())
            .unwrap(),
        content_length: response
            .content_length()
            .expect("response should have a content_length"),
        stream: response.bytes_stream(),
    })
}

async fn upload_file<S>(client: &Client, file_stream: FileStream<S>) -> reqwest::Result<String>
where
    S: Stream<Item = reqwest::Result<Bytes>> + Send + Sync + 'static,
{
    let body_stream = Body::wrap_stream(file_stream.stream);
    let file_part =
        reqwest::multipart::Part::stream_with_length(body_stream, file_stream.content_length)
            .file_name(file_stream.filename);
    let multipart_form = reqwest::multipart::Form::new().part("file", file_part);

    client
        .post("http://127.0.0.1:8080/")
        .multipart(multipart_form)
        .send()
        .await?
        .text()
        .await
}
