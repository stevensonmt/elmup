extern crate serde_json;

use std::{ env, fs, io, io::prelude::*, process::{ Command, Stdio } };



fn main() -> Result<(), &'static str> {
    println!("\n");
    let path = env::var("PATH").expect("$PATH not set in your ENV variables.");//.unwrap_or(panic!("Elm is not installed in your $PATH. Please see https://guide.elm-lang.org/install.html for installation instructions.")); 
    let mut paths = path.split(":");
    let elm_path = paths.find(|p| {println!("Checking path {:?} for Elm installation ...\n", p); fs::metadata(&(format!("{}/elm", p))).is_ok()}).map(|q| {let elmpath = format!("{}/elm", q); println!("Found Elm installed at {}.\n", elmpath); elmpath }).unwrap_or_else(|| panic!("Elm is not installed in your $PATH. Please see https://guide.elm-lang.org/install.html for installation instructions."));
        
    
    let pwd = env::current_dir().expect("Hmm, I can't read the current dir for some reason.");

    let file_not_found = | dir: &std::path::PathBuf, file: &str | format!("Could not open {}. Either {:?} does not contain an 'elm.json' file or an error occurred when I tried to back it up.", file, dir);

    fs::rename("elm.json", "elm.json.bak").unwrap_or_else(|_e| println!("When I tried to back up your elm.json, I got an error. This probably means the backup has already happened or elm.json does not exist. I'll check for the elm.json.bak file next."));
    
    let file = fs::File::open("elm.json.bak").expect(&file_not_found(&pwd, "elm.json.bak"));

    println!("\nelm.json.bak successfully found or created\n");

    let mut reader = io::BufReader::new(file);

    let mut json = String::new();

    reader.read_to_string(&mut json).expect("Can't read file to string.");

    let mut parsed_json: serde_json::Value = serde_json::from_str(&json).expect("Can't deserialize the json. Is it formatted correctly?");

    let old_deps = &parsed_json["dependencies"]["direct"];

    //let deps = deps.as_object().unwrap().iter().map(|(pkg, v)| (pkg.to_owned(), v.as_str().unwrap_or("").to_string())).collect::<Vec<(String, String)>>();
    
    Command::new(format!("{}", elm_path)).arg("init").stdout(Stdio::inherit()).spawn().expect("Elm init failed.").wait().expect("you didn't wait!");

    let mut tmp: Vec<usize> = vec!();

    old_deps.as_object().unwrap().iter().for_each(|(dep, _v)| { Command::new(format!("{}", elm_path)).args(&["install", dep]).stdout(Stdio::inherit()).spawn().expect("Elm install failed.").wait().expect("y u no wait?"); println!("Finished installing dependencies.")});

    /*let deps = old_deps.as_object()*/
        //.unwrap()
        //.iter()
        //.enumerate();

    //for (i,(package, version)) in deps {
        //println!("{}. {} is installed as version {}. Do you want to upgrade this package? (y/N)", i+1, package, version);
        //let mut confirm = String::new();
        //io::stdin().read_line(&mut confirm).expect("Sorry, failed to read that input.");
        //match confirm.trim().to_lowercase().as_str() {
        
            //"y" => { tmp.push(i)
            //},
            //"n" | "" => continue,
            //_ => println!("Please enter 'y' or 'n' or hit 'enter' to accept the default 'n'.")
        //}
    //}

    //let unchanged_deps: serde_json::Value = old_deps.as_object().unwrap().iter().enumerate().filter(|(i, (_p,_v))| !tmp.contains(i)).map(|(_i, (p,v))| format!("{}: {},", p, v)).collect();

    //println!("{}", unchanged_deps);

    //let mut list = String::new();
    //list = tmp.iter().map(|i| format!("{}, {}", list, i+1)).collect::<String>();
    //list = list.trim_start_matches(',').trim().to_string();
    //println!("Okay, I will try to only update package(s) {}.\nNow I'm going to run 'elm init' to create a new elm.json.\n", list);
    //println!("Now I'm going to add the packages you didn't want to update to the elm.json.");
    //let new_elm_json = fs::File::open("elm.json").expect(&file_not_found(&pwd, "elm.json"));
    //let mut new_json = String::new();
    //reader = io::BufReader::new(new_elm_json);
    //reader.read_to_string(&mut new_json).expect("Can't read file to string.");
    /*parsed_json = serde_json::from_str(&new_json).expect("Can't deserialize the json. Is it formatted correctly?");*/
    //Command::new(format!("{}", elm_path)).args(format!("install {}", pkg)).stdout(Stdio::inherit()).spawn().expect("'elm install' failed.").wait().expect("you didn't wait!");
    
    Ok(())

}
