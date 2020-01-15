# Fulcrum
Distributed Ultra Scalable Database 

# plans

- Grpc for remoting
- Protobuf .proto files for stored data
- Flatbufs for stored data
- Async Rust server
- Native clients for Rust/.Net/Java
- sled, as embedded storage (millions/sec reads/writes for short data)
- Special treatment for long keys
- Special treatment for long values
- Long values partitioning 
- CDN-like lru cache-through servers for value parts
- Server replication via Raft and CRDT
- Merkle-based data consistency
- "soft-delete" 
- "key TTL/expiration"
- Orleans inspired distributed transactions (AKA external ACID) 
- ACL-based security
- Regex indices
- Native sharding
- Automatic query optimisation
- SQL
