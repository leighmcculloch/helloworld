FROM leighmcculloch/swift
ENV PORT 80
EXPOSE $PORT
ADD .build/debug/helloworld /
ENTRYPOINT /helloworld --bind 0.0.0.0:$PORT
