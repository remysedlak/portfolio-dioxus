@echo off
start cmd /k "npx @tailwindcss/cli -i ../input.css -o ../assets/tailwind.css --watch"
start cmd /k "dx serve"
