use bytes::Bytes;
use futures_util::Stream;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let _file = request_file(client, "https://wallpapercave.com/wp/wp5111087.jpg").await?;

    Ok(())
}

struct FileStream<S> {
    filename: String,
    content_length: u64,
    stream: S,
}

async fn request_file(
    client: Client,
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
