docker-build:
	docker run --rm --privileged -it -v $$PWD:$$PWD -w $$PWD leighmcculloch/swift swift build
	docker build -t leighmcculloch/helloworld-swift .

docker-run:
	docker run -it -p 8080:80 leighmcculloch/helloworld-swift

docker-push:
	docker push leighmcculloch/helloworld-swift
