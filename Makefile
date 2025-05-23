


release:
	cargo update --verbose
	git diff
	-git commit -am "updated dependencies [ci skip]";
	-zuper-rs-build-utils tag
	-git push --tags
	-git push