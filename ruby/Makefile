run:
	ruby main.rb

setup:
	bundle install --local

vendor:
	bundle pack --all

docker-build:
	docker build -t leighmcculloch/helloworld-ruby .

docker-run:
	docker run -it -p 8080:8080 leighmcculloch/helloworld-ruby

docker-push:
	docker push leighmcculloch/helloworld-ruby
