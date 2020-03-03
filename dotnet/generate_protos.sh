#!/bin/bash

OS=""
if [ "$(uname)" == "Darwin" ]; then
   OS="macosx"
elif [ "$(uname)" == "Linux" ]; then
   OS="linux"
else
   OS="windows"
fi

MACHINE=""
if [ "$(uname -m)" == "x86_64" ]; then
   MACHINE="x64"
else
   MACHINE="x86"
fi

PLATFORM=${OS}_${MACHINE}
PROTOC=$HOME/.nuget/packages/google.protobuf.tools/3.7.0/tools/${PLATFORM}/protoc
PLUGIN=$HOME/.nuget/packages/grpc.tools/1.19.0/tools/${PLATFORM}/grpc_csharp_plugin

mkdir -p generated
$PROTOC -I../src/proto/ -I$HOME/.nuget/packages/google.protobuf.tools/3.7.0/tools/ --csharp_out generated  ../src/proto/fulcrum.proto --grpc_out generated --plugin=protoc-gen-grpc=$PLUGIN 

echo "done"