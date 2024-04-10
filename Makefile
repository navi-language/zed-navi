build:
	mkdir -p target/navi
	zed-extension --source-dir ./ --output-dir ./target/ --scratch-dir ./target
	tar -xzf target/archive.tar.gz -C target/navi
	cp -Rf target/navi ~/Library/Application\ Support/Zed/extensions/installed/
	tree ~/Library/Application\ Support/Zed/extensions/installed/navi
