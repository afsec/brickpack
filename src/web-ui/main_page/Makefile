
help:
	@echo
	@echo "make - This message"
	@echo "make help - This message"

	@echo "make release - Build artifacts to folder: [./dist/]"
	@echo "make debug - Build artifacts to folder: [./dist/] with Debbuger"
	@echo "make clean - Clean artifacts"
	
	@echo


pre-build:
	@mkdir -p ./dist

## BUILDS
debug: pre-build
	@elm make --debug --output=main.js src/Main.elm
	$(MAKE) post-build
	@#$(MAKE) run

release: pre-build
	@elm make --optimize --output=main.js src/Main.elm
	@echo -n "Optimizing code ...  "
	@./optimize.sh
	@echo "Ok"
	$(MAKE) post-build

## BUILDS

post-build:
	@cp -v main.html ./dist/index.html
	@cp -v main.js ./dist/
	@cp -v elm-loader.js ./dist/
	@cp -v main.js ../artifacts/js/main.js

run:
	@python3 -m http.server --directory ./dist 9000


clean:
	@rm -rfv ./dist
	@rm -rfv ./main.js


