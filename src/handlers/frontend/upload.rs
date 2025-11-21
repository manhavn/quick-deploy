use crate::env::frontend::ENV;
use axum_extra::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[derive(serde::Serialize)]
pub struct ResultStruct {
    message: String,
    success: bool,
}

pub async fn handler(mut multipart: Multipart) -> ResultStruct {
    let mut message = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unnamed").to_string();
        let file_name = field.file_name().unwrap_or("noname").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        let data = field.bytes().await.unwrap();

        if name != "dist" || content_type != "application/zip" {
            continue;
        }

        let now = chrono::Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S_UTC");
        let new_file_name = format!("{}_{}.zip", timestamp, name);

        let filepath = format!("{}/{new_file_name}", ENV.rust_app_frontend_upload_path);
        let mut file = File::create(&filepath).await.unwrap();
        file.write_all(&data).await.unwrap();

        {
            // Giải nén file zip vào thư mục tmp
            let tmp_dir = format!("{}/tmp", ENV.rust_app_frontend_upload_path);
            tokio::fs::create_dir_all(&tmp_dir).await.unwrap();

            let zip_file = File::open(&filepath).await.unwrap();
            let mut zip_reader = tokio::io::BufReader::new(zip_file);
            let mut buffer = Vec::new();
            zip_reader.read_to_end(&mut buffer).await.unwrap();

            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(buffer)).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath = std::path::Path::new(&tmp_dir).join(file.mangled_name());

                if file.name().ends_with('/') {
                    tokio::fs::create_dir_all(&outpath).await.unwrap();
                } else {
                    if let Some(p) = outpath.parent() {
                        tokio::fs::create_dir_all(p).await.unwrap();
                    }
                    let mut outfile = File::create(&outpath).await.unwrap();
                    let mut file_buf = Vec::new();
                    std::io::copy(&mut file, &mut file_buf).unwrap();
                    outfile.write_all(&file_buf).await.unwrap();
                }
            }

            // Xoá tất cả file và thư mục trong static_path
            let static_path = &ENV.rust_app_frontend_static_path;
            if tokio::fs::metadata(static_path).await.is_ok() {
                let mut entries = tokio::fs::read_dir(static_path).await.unwrap();
                while let Some(entry) = entries.next_entry().await.unwrap() {
                    let path = entry.path();
                    if path.is_dir() {
                        tokio::fs::remove_dir_all(&path).await.unwrap();
                    } else {
                        tokio::fs::remove_file(&path).await.unwrap();
                    }
                }
            }

            copy_dir_all(
                std::path::Path::new(&tmp_dir),
                std::path::Path::new(static_path),
            )
            .await
            .unwrap();

            // Xoá tmp
            tokio::fs::remove_dir_all(&tmp_dir).await.unwrap();
        }

        message.push_str(&format!(
            "Saved: {file_name} to {} ({} bytes)",
            new_file_name,
            data.len()
        ));
    }

    ResultStruct {
        message: message,
        success: true,
    }
}

// Copy toàn bộ file và thư mục từ tmp sang static_path
fn copy_dir_all<'a>(
    src: &'a std::path::Path,
    dst: &'a std::path::Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        tokio::fs::create_dir_all(dst).await?;
        let mut entries = tokio::fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;
            let dest_path = dst.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_all(&path, &dest_path).await?;
            } else {
                tokio::fs::copy(&path, &dest_path).await?;
            }
        }
        Ok(())
    })
}
