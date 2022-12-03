set shell := ["powershell.exe", "-c"]
run:
    tailwindcss -c tailwind.config.js -o ./tailwindcss.css
    cargo run --bin usuarios