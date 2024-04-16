fn main() {
    use std::process::{Command, Stdio};
 
    use std::str;

    let mut command = Command::new("bash");

    // Pass the script name as an argument
    

    let output = command.arg("bash_script.sh").stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // Convert the output bytes to a string
    let stdout = str::from_utf8(&output.stdout).expect("Not UTF-8");

    // Print the output
    println!("{}", stdout);

}
