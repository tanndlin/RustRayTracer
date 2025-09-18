set shell := ["powershell.exe", "-c"]

profile:
    cargo build --no-default-features
    flamegraph -o graph.svg -- .\target\debug\rs-raytracer.exe