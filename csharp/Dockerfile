FROM microsoft/aspnet:1.0.0-beta5
ENV PORT 80
EXPOSE $PORT
COPY . /app
WORKDIR /app
RUN ["dnu", "restore"]
ENTRYPOINT dnx . kestrel
