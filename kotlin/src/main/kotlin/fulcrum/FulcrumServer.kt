/*
 * Copyright 2020 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.grpc.examples.fulcrum

import com.google.common.base.Stopwatch
import com.google.common.base.Ticker
import com.google.common.io.ByteSource
import com.google.protobuf.util.Durations
import fulcrum.AddRequest
import fulcrum.AddResponse
import fulcrum.DataTreeGrpcKt
import fulcrum.Key
import io.grpc.Server
import io.grpc.ServerBuilder
import java.util.Collections
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.filter
import kotlinx.coroutines.flow.flow
//import maryk.rocksdb

/**
 * Kotlin adaptation of RouteGuideServer from the Java gRPC example.
 */

import io.grpc.examples.helloworld.GreeterGrpcKt
import io.grpc.examples.helloworld.HelloReply
import io.grpc.examples.helloworld.HelloRequest

class FulcrumServer constructor(private val port: Int) {
    val server: Server = ServerBuilder
            .forPort(port)
            .addService(FulcrumServerService())
            .build()

    fun start() {
        server.start()
        println("Server started, listening on $port")
        Runtime.getRuntime().addShutdownHook(
                Thread {
                    println("*** shutting down gRPC server since JVM is shutting down")
                    this@FulcrumServer.stop()
                    println("*** server shut down")
                }
        )
    }

    private fun stop() {
        server.shutdown()
    }

    fun blockUntilShutdown() {
        server.awaitTermination()
    }

    private class FulcrumServerService : DataTreeGrpcKt.DataTreeCoroutineImplBase() {
        override suspend fun add(request: AddRequest): AddResponse =
                AddResponse.newBuilder().setSuccess(request.key).build()
    }
}

fun main() {
    val port = 50055
    val server = FulcrumServer(port)
    server.start()
    server.blockUntilShutdown()
}