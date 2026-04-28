use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

/// Donde se guarda el cache de las portadas de album.
pub fn cache_dir() -> PathBuf {
    let base = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join("music-db").join("art");
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    dir
}

/// Generar un nombre unico para cada portada usando un hash del path.
fn path_art(mp3_path: &str) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    mp3_path.hash(&mut hasher);
    let hash = hasher.finish();
    cache_dir().join(format!("{:016x}.jpg", hash))
}

/// Sacar la portada del mp3 y guardarla en cache.
pub fn extraer_y_guardar(mp3_path: &str) -> Option<String> {
    let out = path_art(mp3_path);

    if out.exists() {
        return Some(out.to_string_lossy().into_owned());
    }

    let tag = id3::Tag::read_from_path(mp3_path).ok()?;
    let pic = tag.pictures().next()?;
    std::fs::write(&out, &pic.data).ok()?;
    Some(out.to_string_lossy().into_owned())
}
