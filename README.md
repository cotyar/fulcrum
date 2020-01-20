# Fulcrum
Distributed Ultra Scalable Database 

Staying on shoulders of Giants:
- Best in Breed storage
- Best in Breed netwoking
- Fit for purpose modern languages
- Proven big blocks, no reinvented bycicles

# plans

- [x] *Grpc* for remoting
- [x] Client-side *load balancing*
- [x] *TLS* enabled communication
- [ ] Authentication
  - [ ] *OAuth*
  - [ ] *NTLM*
  - [ ] *Kerberos*
- [x] *Protobuf* .proto files for stored data
- [ ] *Flatbufs* for stored data (as per *Apache Arrow*)
- [x] *Async Rust* server
- [ ] Native clients for 
  - [x] *Rust*
  - [ ] *.Net*
  - [ ] *Java/Kotlin*
  - [ ] *Javascript*
- [x] *sled*, as embedded storage (millions/sec reads/writes for short data)
- [ ] Special treatment for long keys
- [x] Special treatment for long value 
  - [x] Zero-cost value cloning
  - [x] Long values partitioning 
- [x] *CDN*-like *RLU* cache-through servers for value parts
- [ ] Server replication via *Raft* and _Reliable Casual Broadcast_ (with *CRDT*)
- [ ] *Merkle*-based data consistency
- [ ] "soft-delete" 
- [ ] keys *TTL* (expirations)
- [ ] *Orleans* inspired *distributed transactions* (aka external *ACID*) 
- [ ] *ACL*-based authorizations
  - [ ] *LDAP3* integration (*Active Directory*, etc)
- [ ] *Regex* indices
- [ ] Native *sharding*
- [ ] Automatic *query optimisation*
- [x] Remote logging and instrumentation 
  - [ ] *Opentracing* 
- [ ] Write auto batching
- [ ] Persistent Subscriptions
- [ ] Value compression
- [ ] Value encryption
- [ ] *SQL*
