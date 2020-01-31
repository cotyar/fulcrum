# Fulcrum
Distributed Ultra Scalable Database 

Staying on shoulders of Giants:
- Best in Breed storage
- Best in Breed netwoking
- Fit for purpose modern languages
- Proven big blocks, no reinvented bycicles

# Plans

- [x] [*Grpc*](https://grpc.io/) for remoting
- [x] Client-side *load balancing*
- [x] *TLS* enabled communication
- [ ] Authentication
  - [ ] *OAuth*
  - [ ] *NTLM*
  - [ ] *Kerberos*
- [x] [*Protobuf*](https://developers.google.com/protocol-buffers) .proto files for stored data
- [ ] [*Flatbufs*](https://google.github.io/flatbuffers/) for stored data (as per [*Apache Arrow*](https://arrow.apache.org/))
- [x] *Async [Rust](https://www.rust-lang.org/)* server
- [ ] Native clients for 
  - [x] *Rust*
  - [ ] *.Net*
  - [ ] *Java/Kotlin*
  - [ ] *Javascript*
- [x] [*sled*](http://sled.rs/), as embedded storage (millions/sec reads/writes for short data)
- [ ] Special treatment for long keys
- [x] Special treatment for long value 
  - [x] Zero-cost value cloning
  - [x] Long values partitioning 
- [x] [*CDN*](https://en.wikipedia.org/wiki/Content_delivery_network)-like [*LRU*](https://medium.com/@vienchitang/what-is-an-lru-cache-3e8ad1853584) cache-through servers for value parts
- [ ] Server replication via [*Raft*](https://raft.github.io/) and *Reliable Casual Broadcast* (with [*CRDT*](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type))
  - [ ] Reimplement [RCB](https://engineering.purdue.edu/FTC/handouts/References/Chapt%204%20-%20Broadcast%20-%20jalote.pdf) from https://github.com/cotyar/braindump in Rust 
  - [ ] Sync-free Primitives and partitioning resistence similar to [AntidotDB](https://www.antidotedb.eu)
- [ ] [*Merkle-tree*](https://en.wikipedia.org/wiki/Merkle_tree)-based data consistency
- [ ] "soft-delete" 
- [ ] keys [*TTL*](https://en.wikipedia.org/wiki/Time_to_live) (expirations)
- [ ] [*Orleans*](https://dotnet.github.io/orleans/) inspired *distributed transactions* (aka external *ACID*) 
- [ ] [*RavenDB*](https://ravendb.net/)-inspired automated index management
- [ ] [*ACL*](https://en.wikipedia.org/wiki/Access-control_list)-based authorizations
  - [ ] *LDAP3* integration ([*Active Directory*](https://en.wikipedia.org/wiki/Active_Directory), etc)
- [ ] [*Column-oriented*](https://en.wikipedia.org/wiki/Column-oriented_DBMS) indices
- [ ] [*Regex*](https://en.wikipedia.org/wiki/Regular_expression) indices
- [ ] [*Full-text search*](https://en.wikipedia.org/wiki/Full-text_search)
- [ ] [*ActorDB*](https://www.actordb.com/) inspired multi-island [*sharding*](https://searchoracle.techtarget.com/definition/sharding)
- [ ] Automatic *query optimisation*
- [x] Remote logging and instrumentation 
  - [x] *tokio-tracing*
  - [ ] *Opentracing* 
- [ ] Write auto batching
- [ ] Persistent Subscriptions
- [ ] Value compression
- [ ] Value encryption
- [ ] Externaly consistent [*SQL*](https://en.wikipedia.org/wiki/SQL)
- [ ] [*WebAssembly*](https://webassembly.org/)-based stored procedures (somewhat inspired by *EOS*)
