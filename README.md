# protobuf_import_grpc_rust_example

Example import common types protobuf in rust

#### proto content

base.proto - basic types
``` protobuf
syntax = "proto3";

package base;

message Identity{
  string name = 1;
  string address = 2;
  int32 age = 3;
  Job job = 4;
}

message Job{
  string name = 1;
  string description = 2;
}
```

message_a.proto - types for service A
``` protobuf
syntax = "proto3";
import "google/protobuf/timestamp.proto";
import "base.proto";

package message_a;

message MessageA{
  google.protobuf.Timestamp date = 1;
  base.Job job = 2;
  base.Identity manager = 3;
}
```

message_b.proto - types for service B
``` protobuf
syntax = "proto3";

import "google/protobuf/timestamp.proto";
import "base.proto";

package message_b;

message MessageB{
  google.protobuf.Timestamp date = 1;
  base.Job job = 2;
  base.Identity manager = 3;
  string description = 4;
}
```

### output struct

base.rs
``` rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub age: i32,
    #[prost(message, optional, tag="4")]
    pub job: ::core::option::Option<Job>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
```

message_a.rs
``` rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageA {
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub job: ::core::option::Option<super::base::Job>,
    #[prost(message, optional, tag="3")]
    pub manager: ::core::option::Option<super::base::Identity>,
}
```

message_b.rs
``` rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageB {
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub job: ::core::option::Option<super::base::Job>,
    #[prost(message, optional, tag="3")]
    pub manager: ::core::option::Option<super::base::Identity>,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,

```

### usage

todo