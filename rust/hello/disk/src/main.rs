use std::{fs,io};
fn main() {
    let mut path = "/".to_string();
    loop {
        let root = fs::read_dir(&path);
        if let Ok(dir) = root {
            let files = dir.map(|file| file.unwrap()).collect::<Vec<_>>();
            files
                .into_iter()
                .for_each(|f| {
                    let name = f.file_name().into_string().unwrap();
                    let metadata = f.metadata().unwrap();
                    let is_dir = metadata.is_dir();
                    let mut size = "文件夹".to_string();
                    if !is_dir {
                        size = format!("{}",metadata.len());
                    }
                    println!("{:30}{}",name,size);
                });
        }
        let mut tmp = String::from("");
        io::stdin().read_line(&mut tmp).unwrap();
        let path_arr = tmp.split_whitespace().collect::<Vec<_>>();
        println!("{:?}",path_arr);
        if path_arr.len() == 1 && path_arr[0] == "q"{
            break;
        }else if path_arr.len() == 2 && path_arr[0] == "cd" {
            if path_arr[1].starts_with("..") {
                
            }else {
                path += &format!("{}/",path_arr[1]);
            }
        }
        println!("{}",path);
    }
}
