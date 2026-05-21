set shell := ["powershell.exe", "-c"]

profile:
    cargo build --no-default-features --target-dir target/profile-st
    samply record .\target\profile-st\debug\rs-raytracer.exe