fn main() {
    let image = image::open("./icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    println!("{width},{height},{:?}",rgba);
}
