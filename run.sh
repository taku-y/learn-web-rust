#!/bin/bash
docker run -it -p 8000:8000 -p 9001:9001 --rm \
    -v $(pwd)/workspace:/root/workspace \
    --name learn-web-rust learn-web-rust
