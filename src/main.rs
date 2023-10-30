use std::fs::{self, DirEntry};

mod io_utils;
mod loaders_utils;
mod path_utils;
mod file_utils;
mod text_utils;

fn main() {
    let response = io_utils::text("Percorso");
    if response.is_err()
    {
        let err = response.err().unwrap();
        println!("{:#?}", err);
        return;
    }

    let input_path = response.unwrap();
    let response = path_utils::is_path_valid(&input_path);

    let loader = loaders_utils::get_spinner("Searching files to convert...");
    
    if response.is_err()
    {
        let err = response.err().unwrap();
        println!("{:#?}", err);
        return;
    }

    let path = response.unwrap();

    let response = fs::read_dir(path);
    if response.is_err()
    {
        let err = response.err().unwrap();
        println!("{:#?}", err);
        return;
    }

    let dir = response.unwrap();
    let all_files: Vec<DirEntry> = dir.map(|x| x.unwrap()).collect();

    loader.finish_with_message(format!("Found {} files to convert", all_files.len()));

    let loader = loaders_utils::get_spinner("Starting conversion...");

    for entry in all_files {
        let file_name = entry.file_name();
        if !file_name.to_str().unwrap().to_lowercase().ends_with(".cs") {
            continue;
        }

        loader.set_message(format!("Converting {:?}", file_name));
        let mut final_lines: Vec<String> = vec![];
        let mut opened_braces = 0;

        let entry_path = entry.path();
        let response = fs::read_to_string(&entry_path);
        if response.is_err()
        {
            let err = response.err().unwrap();
            println!("{:#?}", err);
            continue;
            // return;
        }

        let content = response.unwrap();
        let lines = content.lines();
        for line in lines {
            if text_utils::is_comment(line) {
                continue;
            }

            let line = line.trim();

            if text_utils::is_using(line)
            {
                final_lines.push("\n".to_string());
                final_lines.push(line.to_string());
                continue;
            }

            if text_utils::is_namespace(line) {
                final_lines.push("\nusing System.Text.Json.Serialization;\n".to_string());
                final_lines.push("\n".to_string());
                final_lines.push(format!("{};", line.to_string()));
                continue;
            }

            if text_utils::is_class(line) {
                text_utils::add_class(line, &mut final_lines);
                opened_braces += 1;
                continue;
            }

            if text_utils::is_property_decorator(line) {
                text_utils::add_property_decorator(line, &mut final_lines);
                continue;
            }

            if text_utils::is_property(line) {
                text_utils::add_property(line, &mut final_lines);
                continue;
            }
        }

        text_utils::close_braces(opened_braces, &mut final_lines);

        let result: String = final_lines.iter().map(|x| x.to_string()).collect();

        let original_path = entry.path();

        let response = file_utils::delete_file(original_path.to_str().unwrap());
        if response.is_err()
        {
            let err = response.err().unwrap();
            println!("{:#?}", err);
            continue;
            // return;
        }

        let response = fs::write(original_path, result);
        if response.is_err()
        {
            let err = response.err().unwrap();
            println!("{:#?}", err);
            continue;
            // return;
        }
    }

    loader.finish_with_message("All files converted");
}
