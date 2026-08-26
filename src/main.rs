mod implant;
mod pipe_common;

fn main() {
    println!("[+] Starting Pipe Agent (Implant)...");
    if let Err(e) = implant::run_implant() {
        eprintln!("Agent error: {}", e);
    }
}