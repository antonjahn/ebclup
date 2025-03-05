use std::process::Command;

fn main() {
    // Run the pack.sh script
    let status = Command::new("sh").current_dir("resources")
        .arg("pack.sh")
        .status()
        .expect("Failed to execute pack.sh");

    if !status.success() {
        panic!("pack.sh script failed");
    }
}
