set shell := ["powershell.exe", "-c"]
run:
    tailwindcss -c tailwind.config.js -o ./static/tailwindcss.css
    cargo run --bin usuarios