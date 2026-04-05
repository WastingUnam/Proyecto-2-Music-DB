use id3::{Tag, TagLike};
use walkdir::{WalkDir, DirEntry};

fn es_mp3(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|x| x.ends_with(".mp3")).unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directorio = WalkDir::new(".").into_iter().filter_map(|x| x.ok());
    for entry in directorio {
        if es_mp3(&entry){
            let path = entry.path();
            let tag = Tag::read_from_path(path).unwrap();

            if let Some(artist) = tag.artist() {
                println!("Artista: {}", artist);
            }
            if let Some(title) = tag.title() {
                println!("Titulo: {}", title);
            }
            if let Some(album) = tag.album() {
                println!("Album: {}", album);
            }
            if let Some(track) = tag.track(){
                println!("Numero de pista: {}", track);
            }
            println!("{}", path.display());
            print!("\n");
        }
    }
    Ok(())
}
