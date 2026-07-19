# Setup for developers

## Linux
Run project
```bash cargo run ```

Build project for prod
```bash cargo build --release ```

Live compile css to tailwindcss
```bash sudo npx @tailwindcss/cli -c tailwind.config.js -i tailwind.css -o assets/css/tailwind-output.css --watch ```

Prod compile css to tailwindcss
```bash sudo npx @tailwindcss/cli -c tailwind.config.js -i tailwind.css -o assets/css/tailwind-output.css --minify ```
