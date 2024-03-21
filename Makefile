build:
	zed-extension --source-dir ./ --output-dir ./target/ --scratch-dir ./target
	mkdir -p target/navi
	tar -xzf target/archive.tar.gz -C target/navi
	cp -Rf target/navi ~/Library/Application\ Support/Zed/extensions/installed/
	tree ~/Library/Application\ Support/Zed/extensions/installed/navi
