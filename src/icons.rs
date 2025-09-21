// Big shoutout to simpleicons.org and Google's Material Icons

pub fn get_file_icon(filename: &str) -> egui::Image<'static> {
    let path = std::path::Path::new(filename);
    if let Some(extension) = path.extension() {
        match extension.to_string_lossy().to_lowercase().as_str() {
            "txt" => egui::Image::new(egui::include_image!("icons/text.png")),
            "rs" => egui::Image::new(egui::include_image!("icons/rust.png")),
            "py" => egui::Image::new(egui::include_image!("icons/python.png")),
            "js" => egui::Image::new(egui::include_image!("icons/javascript.png")),
            "html" => egui::Image::new(egui::include_image!("icons/html.png")),
            "css" => egui::Image::new(egui::include_image!("icons/css.png")),
            "json" => egui::Image::new(egui::include_image!("icons/json.png")),
            "xml" => egui::Image::new(egui::include_image!("icons/xml.png")),
            "md" => egui::Image::new(egui::include_image!("icons/markdown.png")),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "ico" => egui::Image::new(egui::include_image!("icons/image.png")),
            "mp4" | "avi" | "mkv" | "mov" => egui::Image::new(egui::include_image!("icons/video.png")),
            "mp3" | "wav" | "flac" | "ogg" => egui::Image::new(egui::include_image!("icons/audio.png")),
            "doc" | "docx" | "odt" | "gdoc" => egui::Image::new(egui::include_image!("icons/text.png")),
            "xls" | "xlsx" => egui::Image::new(egui::include_image!("icons/text.png")),
            "ppt" | "pptx" => egui::Image::new(egui::include_image!("icons/text.png")),
            "zip" | "rar" | "7z" | "tar" | "gz" => egui::Image::new(egui::include_image!("icons/archive.png")),
            "exe" | "msi" => egui::Image::new(egui::include_image!("icons/executable.png")),
            _ => egui::Image::new(egui::include_image!("icons/file.png")),
        }
    } else {
        egui::Image::new(egui::include_image!("icons/file.png"))
    }
}