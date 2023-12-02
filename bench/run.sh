#!/bin/bash

# You should edit this path to the specific path of your 
# gatling bin folder
GATLING_BIN_DIR=$HOME/Documents/gatling/bin

WORKSPACE="$( cd "$( dirname "$0" )" &> /dev/null && pwd )"


sh $GATLING_BIN_DIR/gatling.sh -rm local -s NumbersSimulation \
    -rd "DESCRICAO" \
    -rf $WORKSPACE/results \
    -sf $WORKSPACE/simulation \