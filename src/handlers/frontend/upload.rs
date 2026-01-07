use crate::env::frontend::ENV;
use axum_extra::extract::Multipart;
use std::path::Path;
use std::pin::Pin;
use tokio::fs::{File, copy, create_dir_all, metadata, read_dir, remove_dir_all, remove_file};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, Result};

#[derive(serde::Serialize)]
pub struct ResultStruct {
    message: String,
    success: bool,
}

pub async fn handler(mut multipart: Multipart) -> Result<ResultStruct> {
    let mut message = String::new();
    create_dir_all(&ENV.rust_app_frontend_upload_path).await?;
    create_dir_all(&ENV.rust_app_frontend_static_path).await?;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unnamed").to_string();
        let file_name = field.file_name().unwrap_or("noname").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        if name != "dist" || content_type != "application/zip" {
            continue;
        }
        let data = match field.bytes().await.ok() {
            Some(b) => b,
            None => continue,
        };

        let now = chrono::Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S_UTC");
        let new_file_name = format!("{}_{}.zip", timestamp, name);

        let filepath = format!("{}/{new_file_name}", ENV.rust_app_frontend_upload_path);
        let mut file = File::create(&filepath).await?;
        file.write_all(&data).await?;

        {
            // Giải nén file zip vào thư mục tmp
            let tmp_dir = format!("{}/tmp", ENV.rust_app_frontend_upload_path);
            create_dir_all(&tmp_dir).await?;

            let zip_file = File::open(&filepath).await?;
            let mut zip_reader = BufReader::new(zip_file);
            let mut buffer = Vec::new();
            zip_reader.read_to_end(&mut buffer).await?;

            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(buffer))?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = Path::new(&tmp_dir).join(file.mangled_name());

                if file.name().ends_with('/') {
                    create_dir_all(&outpath).await?;
                } else {
                    if let Some(p) = outpath.parent() {
                        create_dir_all(p).await?;
                    }
                    let mut outfile = File::create(&outpath).await?;
                    let mut file_buf = Vec::new();
                    std::io::copy(&mut file, &mut file_buf)?;
                    outfile.write_all(&file_buf).await?;
                }
            }

            // Xoá tất cả file và thư mục trong static_path
            let static_path = &ENV.rust_app_frontend_static_path;
            if metadata(static_path).await.is_ok() {
                let mut entries = read_dir(static_path).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_dir() {
                        remove_dir_all(&path).await?;
                    } else {
                        remove_file(&path).await?;
                    }
                }
            }

            copy_dir_all(Path::new(&tmp_dir), Path::new(static_path)).await?;

            // Xoá tmp
            remove_dir_all(&tmp_dir).await?;
        }

        message.push_str(&format!(
            "Saved: {file_name} to {} ({} bytes)",
            new_file_name,
            data.len()
        ));
    }

    Ok(ResultStruct {
        message,
        success: true,
    })
}

// Copy toàn bộ file và thư mục từ tmp sang static_path
fn copy_dir_all<'a>(
    src: &'a Path,
    dst: &'a Path,
) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        create_dir_all(dst).await?;
        let mut entries = read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_type = entry.file_type().await?;
            let dest_path = dst.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_all(&path, &dest_path).await?;
            } else {
                copy(&path, &dest_path).await?;
            }
        }
        Ok(())
    })
}
