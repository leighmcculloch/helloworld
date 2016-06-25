run:
	cargo run

build:
	cargo build --release

docker-build:
	docker run --rm --privileged -it -v $$PWD:$$PWD -w $$PWD scorpil/rust cargo build --release
	docker build -t leighmcculloch/helloworld-rust .

docker-run:
	docker run -it -p 8080:80 leighmcculloch/helloworld-rust

docker-push:
	docker push leighmcculloch/helloworld-rust
