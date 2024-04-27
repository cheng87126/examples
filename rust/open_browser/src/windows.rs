use std::process::Command;

pub fn open(){
    let href = "https://github.com";
    Command::new("cmd.exe").arg("/C").arg("start").arg(href).spawn().unwrap();
}