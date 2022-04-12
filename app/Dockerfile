FROM alpine:3.15.4
RUN apk --no-cache add openjdk11
COPY . .
RUN ./gradlew bootJar --no-daemon
RUN mv build/libs/*SNAPSHOT.jar app.jar
ENTRYPOINT ["java","-jar", "/app.jar"]
EXPOSE 8080