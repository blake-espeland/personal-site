serve: dump
	@echo "building with Trunk..."
	@trunk build --release
	@echo "copying build files to alt directory..."
	@cp -r dist ~/Desktop/blake-espeland.github.io
	@mv ~/Desktop/blake-espeland.github.io/dist/* ~/Desktop/blake-espeland.github.io
	@rm -rf ~/Desktop/blake-espeland.github.io/dist
	@echo "files ready to commit"
	@echo "go to ~/Desktop/blake-espeland.github.io to check changes"


dump:
	@echo "preparing to begin..."
	@rm -f ~/Desktop/blake-espeland.github.io/*.html
	@rm -f ~/Desktop/blake-espeland.github.io/*.css
	@rm -f ~/Desktop/blake-espeland.github.io/*.wasm
	@rm -f ~/Desktop/blake-espeland.github.io/*.js
	@rm -rf ~/Desktop/blake-espeland.github.io/resources
	@rm -rf ~/Desktop/blake-espeland.github.io/dist