#!/usr/bin/env bash
docker ps | tail -n 1 | awk '{print $1}' | xargs docker stop
docker run -d --rm --network=host glotitude_hq