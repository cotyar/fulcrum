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

import fulcrum.AddRequest
import fulcrum.DataTreeGrpcKt
import fulcrum.Key
import fulcrum.ValueEntry
import io.grpc.ManagedChannel
import io.grpc.ManagedChannelBuilder
import kotlinx.coroutines.*
import java.io.Closeable
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlin.random.Random
import kotlin.random.nextLong
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.flow

class FulcrumClient constructor(private val channel: ManagedChannel) : Closeable {
    private val stub = DataTreeGrpcKt.DataTreeCoroutineStub(channel)

    suspend fun addToCache(value: String) = coroutineScope {
        val request = AddRequest.newBuilder().
            setKey(Key.newBuilder().setKey("test_key")).
            setValue(ValueEntry.newBuilder().setStr(value)).
            build()
        val response = async { stub.add(request) }
        println("Received: ${response.await().respCase.name}")
    }

    override fun close() {
        channel.shutdown().awaitTermination(5, TimeUnit.SECONDS)
    }
}

/**
 * Greeter, uses first argument as name to greet if present;
 * greets "world" otherwise.
 */
fun main(args: Array<String>) = runBlocking {
    val port = 50055

    val client = FulcrumClient(
            ManagedChannelBuilder.forAddress("localhost", port)
                    .usePlaintext()
                    .executor(Dispatchers.Default.asExecutor())
                    .build())

    val user = args.singleOrNull() ?: "Hello World"
    client.addToCache(user)
}