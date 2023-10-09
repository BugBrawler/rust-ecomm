db:
	surreal start

css:
	tailwindcss -i ./styles/app.css -o ./assets/main.css --minify --watch
