all:
	go build -o bin/main

css:
	pnpm styles

dev:
	air & pnpm dev
