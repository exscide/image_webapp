#[macro_use] extern crate rocket;

use std::fs::File;

use rocket::http::ContentType;


#[get("/")]
fn index<'r>() -> std::io::Result<(ContentType, File)> {
	Ok((ContentType::JPEG, File::open("image.jpg")?))
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
