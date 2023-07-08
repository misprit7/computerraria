use std::process::Command;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let output = Command::new("python3")
        .arg("build.py")
        .output()
        .expect("Python script failed!");

    p!("status: {}", output.status);
}

