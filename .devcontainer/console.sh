#!/bin/bash

CONTAINER_ID=`docker container ls | grep vsc-lets_expect | awk '{print $1}'`

if [ -z "$CONTAINER_ID" ]
then
      echo "Devcontainer not found"
      exit 1
fi

echo "Opening console on $CONTAINER_ID..."

docker exec -it $CONTAINER_ID /bin/bash
