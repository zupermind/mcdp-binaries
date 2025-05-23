


release:
	cargo update --verbose
	git diff
	-git commit -am "updated dependencies";
	-zuper-rs-build-utils tag
	-git push --tags
	-git push