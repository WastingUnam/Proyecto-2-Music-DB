use id3::{Tag, TagLike};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tag = Tag::read_from_path("./wetdream/stop calling.mp3")?;

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
    Ok(())
}
