mod cka;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Result<(), std::io::Error> {
    cka::write_encryp_file("music.mp3", "music_encriptada")?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

/*fn main() {
    //cka::write_encryp_file("music.mp3", "music_encriptada")?;
    //Ok(())
}*/
