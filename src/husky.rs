use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

pub fn install(directory: &str) {
    let hook = Assets::get("hook").unwrap();

    println!("Add script to {:?} to {directory}", std::str::from_utf8(hook.data.as_ref()).unwrap());
    todo!("create the function to initialize github");
}
