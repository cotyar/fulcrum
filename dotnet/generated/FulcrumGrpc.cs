// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: fulcrum.proto
// </auto-generated>
#pragma warning disable 0414, 1591
#region Designer generated code

using grpc = global::Grpc.Core;

namespace Fulcrum {
  public static partial class CdnQuery
  {
    static readonly string __ServiceName = "fulcrum.CdnQuery";

    static readonly grpc::Marshaller<global::Fulcrum.CdnGetRequest> __Marshaller_fulcrum_CdnGetRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnGetRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnGetResponse> __Marshaller_fulcrum_CdnGetResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnGetResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnContainsRequest> __Marshaller_fulcrum_CdnContainsRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnContainsRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnContainsResponse> __Marshaller_fulcrum_CdnContainsResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnContainsResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnStreamValueResponse> __Marshaller_fulcrum_CdnStreamValueResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnStreamValueResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnGetResponse> __Method_Get = new grpc::Method<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnGetResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Get",
        __Marshaller_fulcrum_CdnGetRequest,
        __Marshaller_fulcrum_CdnGetResponse);

    static readonly grpc::Method<global::Fulcrum.CdnContainsRequest, global::Fulcrum.CdnContainsResponse> __Method_Contains = new grpc::Method<global::Fulcrum.CdnContainsRequest, global::Fulcrum.CdnContainsResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Contains",
        __Marshaller_fulcrum_CdnContainsRequest,
        __Marshaller_fulcrum_CdnContainsResponse);

    static readonly grpc::Method<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnStreamValueResponse> __Method_StreamValue = new grpc::Method<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnStreamValueResponse>(
        grpc::MethodType.ServerStreaming,
        __ServiceName,
        "StreamValue",
        __Marshaller_fulcrum_CdnGetRequest,
        __Marshaller_fulcrum_CdnStreamValueResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Fulcrum.FulcrumReflection.Descriptor.Services[0]; }
    }

    /// <summary>Base class for server-side implementations of CdnQuery</summary>
    public abstract partial class CdnQueryBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.CdnGetResponse> Get(global::Fulcrum.CdnGetRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.CdnContainsResponse> Contains(global::Fulcrum.CdnContainsRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task StreamValue(global::Fulcrum.CdnGetRequest request, grpc::IServerStreamWriter<global::Fulcrum.CdnStreamValueResponse> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for CdnQuery</summary>
    public partial class CdnQueryClient : grpc::ClientBase<CdnQueryClient>
    {
      /// <summary>Creates a new client for CdnQuery</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public CdnQueryClient(grpc::Channel channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for CdnQuery that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public CdnQueryClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected CdnQueryClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected CdnQueryClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Fulcrum.CdnGetResponse Get(global::Fulcrum.CdnGetRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Get(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.CdnGetResponse Get(global::Fulcrum.CdnGetRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Get, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnGetResponse> GetAsync(global::Fulcrum.CdnGetRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnGetResponse> GetAsync(global::Fulcrum.CdnGetRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Get, null, options, request);
      }
      public virtual global::Fulcrum.CdnContainsResponse Contains(global::Fulcrum.CdnContainsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Contains(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.CdnContainsResponse Contains(global::Fulcrum.CdnContainsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Contains, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnContainsResponse> ContainsAsync(global::Fulcrum.CdnContainsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return ContainsAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnContainsResponse> ContainsAsync(global::Fulcrum.CdnContainsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Contains, null, options, request);
      }
      public virtual grpc::AsyncServerStreamingCall<global::Fulcrum.CdnStreamValueResponse> StreamValue(global::Fulcrum.CdnGetRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return StreamValue(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncServerStreamingCall<global::Fulcrum.CdnStreamValueResponse> StreamValue(global::Fulcrum.CdnGetRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncServerStreamingCall(__Method_StreamValue, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override CdnQueryClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new CdnQueryClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(CdnQueryBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_Get, serviceImpl.Get)
          .AddMethod(__Method_Contains, serviceImpl.Contains)
          .AddMethod(__Method_StreamValue, serviceImpl.StreamValue).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, CdnQueryBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_Get, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnGetResponse>(serviceImpl.Get));
      serviceBinder.AddMethod(__Method_Contains, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.CdnContainsRequest, global::Fulcrum.CdnContainsResponse>(serviceImpl.Contains));
      serviceBinder.AddMethod(__Method_StreamValue, serviceImpl == null ? null : new grpc::ServerStreamingServerMethod<global::Fulcrum.CdnGetRequest, global::Fulcrum.CdnStreamValueResponse>(serviceImpl.StreamValue));
    }

  }
  public static partial class CdnControl
  {
    static readonly string __ServiceName = "fulcrum.CdnControl";

    static readonly grpc::Marshaller<global::Fulcrum.CdnAddRequest> __Marshaller_fulcrum_CdnAddRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnAddRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnAddResponse> __Marshaller_fulcrum_CdnAddResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnAddResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnDeleteRequest> __Marshaller_fulcrum_CdnDeleteRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnDeleteRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CdnDeleteResponse> __Marshaller_fulcrum_CdnDeleteResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CdnDeleteResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Fulcrum.CdnAddRequest, global::Fulcrum.CdnAddResponse> __Method_Add = new grpc::Method<global::Fulcrum.CdnAddRequest, global::Fulcrum.CdnAddResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Add",
        __Marshaller_fulcrum_CdnAddRequest,
        __Marshaller_fulcrum_CdnAddResponse);

    static readonly grpc::Method<global::Fulcrum.CdnDeleteRequest, global::Fulcrum.CdnDeleteResponse> __Method_Delete = new grpc::Method<global::Fulcrum.CdnDeleteRequest, global::Fulcrum.CdnDeleteResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Delete",
        __Marshaller_fulcrum_CdnDeleteRequest,
        __Marshaller_fulcrum_CdnDeleteResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Fulcrum.FulcrumReflection.Descriptor.Services[1]; }
    }

    /// <summary>Base class for server-side implementations of CdnControl</summary>
    public abstract partial class CdnControlBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.CdnAddResponse> Add(global::Fulcrum.CdnAddRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.CdnDeleteResponse> Delete(global::Fulcrum.CdnDeleteRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for CdnControl</summary>
    public partial class CdnControlClient : grpc::ClientBase<CdnControlClient>
    {
      /// <summary>Creates a new client for CdnControl</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public CdnControlClient(grpc::Channel channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for CdnControl that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public CdnControlClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected CdnControlClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected CdnControlClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Fulcrum.CdnAddResponse Add(global::Fulcrum.CdnAddRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Add(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.CdnAddResponse Add(global::Fulcrum.CdnAddRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Add, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnAddResponse> AddAsync(global::Fulcrum.CdnAddRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return AddAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnAddResponse> AddAsync(global::Fulcrum.CdnAddRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Add, null, options, request);
      }
      public virtual global::Fulcrum.CdnDeleteResponse Delete(global::Fulcrum.CdnDeleteRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Delete(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.CdnDeleteResponse Delete(global::Fulcrum.CdnDeleteRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Delete, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnDeleteResponse> DeleteAsync(global::Fulcrum.CdnDeleteRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return DeleteAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CdnDeleteResponse> DeleteAsync(global::Fulcrum.CdnDeleteRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Delete, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override CdnControlClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new CdnControlClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(CdnControlBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_Add, serviceImpl.Add)
          .AddMethod(__Method_Delete, serviceImpl.Delete).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, CdnControlBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_Add, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.CdnAddRequest, global::Fulcrum.CdnAddResponse>(serviceImpl.Add));
      serviceBinder.AddMethod(__Method_Delete, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.CdnDeleteRequest, global::Fulcrum.CdnDeleteResponse>(serviceImpl.Delete));
    }

  }
  /// <summary>
  /// rpc Transaction(stream AddRequest) returns (stream AddResponse) {}
  /// rpc ReadTransaction(stream AddRequest) returns (stream AddResponse) {}
  /// rpc Watch(AddRequest) returns (stream AddResponse) {}
  /// </summary>
  public static partial class DataTree
  {
    static readonly string __ServiceName = "fulcrum.DataTree";

    static readonly grpc::Marshaller<global::Fulcrum.AddRequest> __Marshaller_fulcrum_AddRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.AddRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.AddResponse> __Marshaller_fulcrum_AddResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.AddResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CopyRequest> __Marshaller_fulcrum_CopyRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CopyRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.CopyResponse> __Marshaller_fulcrum_CopyResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.CopyResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.DeleteRequest> __Marshaller_fulcrum_DeleteRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.DeleteRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.DeleteResponse> __Marshaller_fulcrum_DeleteResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.DeleteResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.GetRequest> __Marshaller_fulcrum_GetRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.GetRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.GetResponse> __Marshaller_fulcrum_GetResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.GetResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.ContainsRequest> __Marshaller_fulcrum_ContainsRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.ContainsRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.ContainsResponse> __Marshaller_fulcrum_ContainsResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.ContainsResponse.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.SearchRequest> __Marshaller_fulcrum_SearchRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.SearchRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Fulcrum.SearchResponse> __Marshaller_fulcrum_SearchResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Fulcrum.SearchResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Fulcrum.AddRequest, global::Fulcrum.AddResponse> __Method_Add = new grpc::Method<global::Fulcrum.AddRequest, global::Fulcrum.AddResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Add",
        __Marshaller_fulcrum_AddRequest,
        __Marshaller_fulcrum_AddResponse);

    static readonly grpc::Method<global::Fulcrum.CopyRequest, global::Fulcrum.CopyResponse> __Method_Copy = new grpc::Method<global::Fulcrum.CopyRequest, global::Fulcrum.CopyResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Copy",
        __Marshaller_fulcrum_CopyRequest,
        __Marshaller_fulcrum_CopyResponse);

    static readonly grpc::Method<global::Fulcrum.DeleteRequest, global::Fulcrum.DeleteResponse> __Method_Delete = new grpc::Method<global::Fulcrum.DeleteRequest, global::Fulcrum.DeleteResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Delete",
        __Marshaller_fulcrum_DeleteRequest,
        __Marshaller_fulcrum_DeleteResponse);

    static readonly grpc::Method<global::Fulcrum.GetRequest, global::Fulcrum.GetResponse> __Method_Get = new grpc::Method<global::Fulcrum.GetRequest, global::Fulcrum.GetResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Get",
        __Marshaller_fulcrum_GetRequest,
        __Marshaller_fulcrum_GetResponse);

    static readonly grpc::Method<global::Fulcrum.ContainsRequest, global::Fulcrum.ContainsResponse> __Method_Contains = new grpc::Method<global::Fulcrum.ContainsRequest, global::Fulcrum.ContainsResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "Contains",
        __Marshaller_fulcrum_ContainsRequest,
        __Marshaller_fulcrum_ContainsResponse);

    static readonly grpc::Method<global::Fulcrum.SearchRequest, global::Fulcrum.SearchResponse> __Method_Search = new grpc::Method<global::Fulcrum.SearchRequest, global::Fulcrum.SearchResponse>(
        grpc::MethodType.ServerStreaming,
        __ServiceName,
        "Search",
        __Marshaller_fulcrum_SearchRequest,
        __Marshaller_fulcrum_SearchResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Fulcrum.FulcrumReflection.Descriptor.Services[2]; }
    }

    /// <summary>Base class for server-side implementations of DataTree</summary>
    public abstract partial class DataTreeBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.AddResponse> Add(global::Fulcrum.AddRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.CopyResponse> Copy(global::Fulcrum.CopyRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.DeleteResponse> Delete(global::Fulcrum.DeleteRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.GetResponse> Get(global::Fulcrum.GetRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Fulcrum.ContainsResponse> Contains(global::Fulcrum.ContainsRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task Search(global::Fulcrum.SearchRequest request, grpc::IServerStreamWriter<global::Fulcrum.SearchResponse> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for DataTree</summary>
    public partial class DataTreeClient : grpc::ClientBase<DataTreeClient>
    {
      /// <summary>Creates a new client for DataTree</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public DataTreeClient(grpc::Channel channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for DataTree that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public DataTreeClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected DataTreeClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected DataTreeClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Fulcrum.AddResponse Add(global::Fulcrum.AddRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Add(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.AddResponse Add(global::Fulcrum.AddRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Add, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.AddResponse> AddAsync(global::Fulcrum.AddRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return AddAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.AddResponse> AddAsync(global::Fulcrum.AddRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Add, null, options, request);
      }
      public virtual global::Fulcrum.CopyResponse Copy(global::Fulcrum.CopyRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Copy(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.CopyResponse Copy(global::Fulcrum.CopyRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Copy, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CopyResponse> CopyAsync(global::Fulcrum.CopyRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return CopyAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.CopyResponse> CopyAsync(global::Fulcrum.CopyRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Copy, null, options, request);
      }
      public virtual global::Fulcrum.DeleteResponse Delete(global::Fulcrum.DeleteRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Delete(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.DeleteResponse Delete(global::Fulcrum.DeleteRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Delete, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.DeleteResponse> DeleteAsync(global::Fulcrum.DeleteRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return DeleteAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.DeleteResponse> DeleteAsync(global::Fulcrum.DeleteRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Delete, null, options, request);
      }
      public virtual global::Fulcrum.GetResponse Get(global::Fulcrum.GetRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Get(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.GetResponse Get(global::Fulcrum.GetRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Get, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.GetResponse> GetAsync(global::Fulcrum.GetRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.GetResponse> GetAsync(global::Fulcrum.GetRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Get, null, options, request);
      }
      public virtual global::Fulcrum.ContainsResponse Contains(global::Fulcrum.ContainsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Contains(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Fulcrum.ContainsResponse Contains(global::Fulcrum.ContainsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_Contains, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.ContainsResponse> ContainsAsync(global::Fulcrum.ContainsRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return ContainsAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Fulcrum.ContainsResponse> ContainsAsync(global::Fulcrum.ContainsRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_Contains, null, options, request);
      }
      public virtual grpc::AsyncServerStreamingCall<global::Fulcrum.SearchResponse> Search(global::Fulcrum.SearchRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return Search(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncServerStreamingCall<global::Fulcrum.SearchResponse> Search(global::Fulcrum.SearchRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncServerStreamingCall(__Method_Search, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override DataTreeClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new DataTreeClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(DataTreeBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_Add, serviceImpl.Add)
          .AddMethod(__Method_Copy, serviceImpl.Copy)
          .AddMethod(__Method_Delete, serviceImpl.Delete)
          .AddMethod(__Method_Get, serviceImpl.Get)
          .AddMethod(__Method_Contains, serviceImpl.Contains)
          .AddMethod(__Method_Search, serviceImpl.Search).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, DataTreeBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_Add, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.AddRequest, global::Fulcrum.AddResponse>(serviceImpl.Add));
      serviceBinder.AddMethod(__Method_Copy, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.CopyRequest, global::Fulcrum.CopyResponse>(serviceImpl.Copy));
      serviceBinder.AddMethod(__Method_Delete, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.DeleteRequest, global::Fulcrum.DeleteResponse>(serviceImpl.Delete));
      serviceBinder.AddMethod(__Method_Get, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.GetRequest, global::Fulcrum.GetResponse>(serviceImpl.Get));
      serviceBinder.AddMethod(__Method_Contains, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Fulcrum.ContainsRequest, global::Fulcrum.ContainsResponse>(serviceImpl.Contains));
      serviceBinder.AddMethod(__Method_Search, serviceImpl == null ? null : new grpc::ServerStreamingServerMethod<global::Fulcrum.SearchRequest, global::Fulcrum.SearchResponse>(serviceImpl.Search));
    }

  }
}
#endregion