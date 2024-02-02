use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {

    // args
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();
    if len < 2 { panic!("invalid args"); }
    if fs::metadata(&args[1]).is_err() { panic!("invalid input file path: {}", &args[1]); }
    let m = len - 2;
    if m == 0 { panic!("nothing to modify"); }
    
    
    let mut delete_keys: Vec<String> = vec![];
    for i in 0 .. m {
        let arg = args[2 + i].to_string();
        match arg.chars().next().unwrap() {
            '-' => {
                let keys: Vec<String> = arg.get(1..arg.len()).unwrap().split(",").map(String::from).collect();
                delete_keys.extend(keys);
            },
            _ => {}
        }
    }

    // files
    let path = std::path::Path::new(&args[1]);
    let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
    let file_extension = path.extension().unwrap().to_string_lossy().into_owned();
    let output_file = format!("./{}-edited.{}", file_stem, file_extension);
    println!("input = {}, output = {}", &args[1], &output_file);

    // read input
    let content = fs::read_to_string(&args[1]).unwrap();
    let mut records: Vec<Value> = serde_json::from_str(&content).unwrap();
    
    // write output
    let mut output = std::fs::File::create(output_file).unwrap();
    writeln!(output, "[").unwrap();
    let m = records.len();
    for (index, record) in records.iter_mut().enumerate(){
        for key in delete_keys.iter() {
            let or = record.as_object_mut().unwrap();
            if or.get(key).is_some() {
                or.remove(key).unwrap();
            }
        }
        if index == m - 1 { writeln!(output, "{}", record.to_string()).unwrap(); }
        else { writeln!(output, "{},", record.to_string()).unwrap(); } 
    }
    writeln!(output, "]").unwrap();
    drop(output);

    Ok(())
}
