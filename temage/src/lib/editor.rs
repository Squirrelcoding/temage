// Import dependencies
use std::env;
use std::io::Write;
use std::path::Path;

use colored::Colorize;

// Function to start the editor
pub async fn launch_editor() {
    // Checks if the directory containing the code for the editor exists, if not then it downloads the eeditor
    if !check_if_directory_exists() {
        download_editor().await;
        return;
    }

    //Fetches the username from $USER, if no such variable is found then panic.
    let username = match env::var("USER") {
        Ok(username) => username,
        Err(error) => {
            panic!("{error}");
        }
    };

    // Get the code for the editor in a directory
    let static_assets = warp::fs::dir(format!("/home/{}/.temage-editor", username));

    //Get the process id of the running program and cast it into u16
    let id = std::process::id() as u16;

    // Opens the tab for the editor automatically
    open::that(format!("http://localhost:{}", id)).unwrap();

    // Give message to the user, the server isnt going to shut itself down!
    println!(
        "{}",
        format!(
            "Running editor on port {}. Close the web server with ctrl+c",
            id
        )
        .bold()
        .yellow()
    );

    //Start the web server
    warp::serve(static_assets).run(([127, 0, 0, 1], id)).await;
}

// This function runs if the directory for the application isnt found
pub async fn download_editor() {
    // Fetch $USER, panic if not found.
    let username = match env::var("USER") {
        Ok(username) => username,
        Err(error) => {
            panic!("{error}");
        }
    };

    // Exit early if directory is found, this check is probably useless though.
    if check_if_directory_exists() {
        return;
    }

    // Fancy and colorful message
    println!(
        "{}",
        "You do not have the editor installed yet. Downloading now..."
            .bold()
            .yellow()
    );

    // Create root directory for application
    std::fs::create_dir(format!("/home/{}/.temage-editor", username)).unwrap();

    // Message user of progress
    println!(
        "{}",
        format!("[1/5] Created directory /home/{}/.temage-editor", username)
            .bold()
            .green()
    );

    // Create assets folder for app
    std::fs::create_dir(format!("/home/{}/.temage-editor/assets", username)).unwrap();

    // Message user of progress
    println!(
        "{}",
        format!(
            "[2/5] Created directory /home/{}/.temage-editor/assets",
            username
        )
        .bold()
        .green()
    );

    // This function takes a url of a file (first arg) and writes the contents of that file to a path (second arg) asynchronously.
    // In this case its downloading index.html
    download_file(
        "https://raw.githubusercontent.com/Squirrelcoding/Temage-editor/main/index.html",
        &format!("/home/{}/.temage-editor/index.html", username),
    )
    .await;

    // Message user of progress
    println!("{}", "[3/5] Downloaded index.html".bold().green());

    // Download css file and write it to assets/index.43aa000d.css
    download_file("https://raw.githubusercontent.com/Squirrelcoding/Temage-editor/main/assets/index.43aa000d.css", &format!("/home/{}/.temage-editor/assets/index.43aa000d.css", username)).await;

    // Message user of progress
    println!(
        "{}",
        "[4/5] Downloaded assets/index.43aa000d.css".bold().green()
    );

    // Download js file and write it to assets/index.f7e602da.js
    download_file("https://raw.githubusercontent.com/Squirrelcoding/Temage-editor/main/assets/index.f7e602da.js", &format!("/home/{}/.temage-editor/assets/index.f7e602da.js", username)).await;

    // Message user of progress
    println!(
        "{}",
        "[5/5] Downloaded assets/index.f7e602da.js".bold().green()
    );

    // yay!
    println!(
        "{}",
        "Finished downloading editor, launch the editor to get started!"
            .bold()
            .green()
    );
}

// This function returns a boolean indicating whether the directory for the app already exists
pub fn check_if_directory_exists() -> bool {
    //Get $USER, panic if not found
    let username = match env::var("USER") {
        Ok(username) => username,
        Err(error) => {
            panic!("{error}");
        }
    };

    //Creates the directory
    Path::new(&format!("/home/{}/.temage-editor", username)).exists()
}

// Fetch the files, turn it into a buffer, and write the buffer to the path
pub async fn download_file(url: &str, file: &str) {
    // Get contents
    let mut body = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    // Create file
    let mut file = std::fs::File::create(file).unwrap();

    // Write contents to file
    file.write_all(&mut body).unwrap();
}
