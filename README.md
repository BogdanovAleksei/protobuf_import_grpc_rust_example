# protobuf_import_grpc_rust_example

Example import common types protobuf in rust

#### proto content

base.proto - basic types
``` protobuf
syntax = "proto3";

package base;

message Name{
  string name = 1;
}

message Identity{
  Name name = 1;
  string address = 2;
  int32 age = 3;
  Job job = 4;
}

message Job{
  repeated Name name = 1;
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
  repeated base.Job job = 2;
  repeated base.Identity manager = 3;
  repeated base.Name name = 4;
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
  repeated base.Job job = 2;
  repeated base.Identity manager = 3;
  repeated base.Name name = 4;
  string description = 5;
}
```

### output struct

base.rs
``` rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Name {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub age: i32,
    #[prost(message, optional, tag="4")]
    pub job: ::core::option::Option<Job>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    #[prost(message, repeated, tag="1")]
    pub name: ::prost::alloc::vec::Vec<Name>,
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
    #[prost(message, repeated, tag="2")]
    pub job: ::prost::alloc::vec::Vec<super::base::Job>,
    #[prost(message, repeated, tag="3")]
    pub manager: ::prost::alloc::vec::Vec<super::base::Identity>,
    #[prost(message, repeated, tag="4")]
    pub name: ::prost::alloc::vec::Vec<super::base::Name>,
}
```

message_b.rs
``` rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageB {
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="2")]
    pub job: ::prost::alloc::vec::Vec<super::base::Job>,
    #[prost(message, repeated, tag="3")]
    pub manager: ::prost::alloc::vec::Vec<super::base::Identity>,
    #[prost(message, repeated, tag="4")]
    pub name: ::prost::alloc::vec::Vec<super::base::Name>,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
}
```

### impl for based type Name

```rust
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(String);

impl From<base::Name> for Name {
    fn from(name: base::Name) -> Self {
        Self(name.name)
    }
}
```


### usage

todo