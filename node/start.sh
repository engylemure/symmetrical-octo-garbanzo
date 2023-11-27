#!/bin/bash

if [ "$NODE_ENV" == "dev" ]
then npm run start:dev
else npm run build && npm run start
fi