set shell := ["powershell.exe", "-c"]

profile:
    cargo build -r --no-default-features --target-dir target/profile-st
    samply record .\target\profile-st\debug\rs-raytracer.exe