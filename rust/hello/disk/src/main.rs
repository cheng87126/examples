use std::fs;
fn main() {

    let root = fs::read_dir("/");
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
}
