run:
	mvn package
	java -jar target/helloworld-jar-with-dependencies.jar

build:
	mvn package

docker-build:
	mvn package
	docker build -t leighmcculloch/helloworld-java .

docker-run:
	docker run -it -p 8080:80 leighmcculloch/helloworld-java

docker-push:
	docker push leighmcculloch/helloworld-java
