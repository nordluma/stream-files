use actix_multipart::form::{bytes::Bytes, MultipartForm};
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Server listening on: 127.0.0.1:8080");
    HttpServer::new(|| App::new().route("/", web::post().to(upload_file)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Debug, MultipartForm)]
struct UploadFileForm {
    file: Bytes,
}

async fn upload_file(
    MultipartForm(multipart): MultipartForm<UploadFileForm>,
) -> actix_web::Result<HttpResponse> {
    let file_name = multipart
        .file
        .file_name
        .expect("File should have a file name");
    let content = multipart.file.data;
    let content_type = multipart.file.content_type;

    eprintln!(
        "Received request -\nFilename: {}\nContent-Type: {:?}\nContent-Length: {}",
        file_name,
        content_type,
        content.len()
    );

    std::fs::write(file_name, content)?;

    Ok(HttpResponse::Ok().body("File has been uploaded"))
}
