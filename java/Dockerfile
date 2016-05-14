FROM java:alpine
ENV PORT 80
EXPOSE $PORT
ADD target/helloworld-jar-with-dependencies.jar /
CMD ["java", "-jar", "/helloworld-jar-with-dependencies.jar"]
