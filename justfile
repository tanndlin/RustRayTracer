set shell := ["powershell.exe", "-c"]

profile:
    flamegraph -o graph.svg -- .\target\debug\rs-raytracer.exe