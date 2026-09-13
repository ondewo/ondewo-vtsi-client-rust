// @generated
/// Generated client implementations.
pub mod calls_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CallsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CallsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CallsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CallsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CallsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn start_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::StartCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCallerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::StartCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/ListCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "ListCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCallerRequest>,
        ) -> std::result::Result<tonic::Response<super::Caller>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/GetCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "GetCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/DeleteCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "DeleteCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/DeleteCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "DeleteCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::StopCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::StopCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_listener(
            &mut self,
            request: impl tonic::IntoRequest<super::StartListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartListenerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartListener",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartListener"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_listeners(
            &mut self,
            request: impl tonic::IntoRequest<super::StartListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartListenersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartListeners",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartListeners"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_listener(
            &mut self,
            request: impl tonic::IntoRequest<super::StopListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopListenerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopListener",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopListener"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_listeners(
            &mut self,
            request: impl tonic::IntoRequest<super::StopListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopListenersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopListeners",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopListeners"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_listeners(
            &mut self,
            request: impl tonic::IntoRequest<super::ListListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListListenersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/ListListeners",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "ListListeners"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_listener(
            &mut self,
            request: impl tonic::IntoRequest<super::GetListenerRequest>,
        ) -> std::result::Result<tonic::Response<super::Listener>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/GetListener",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "GetListener"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_listener(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteListenerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/DeleteListener",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "DeleteListener"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_listeners(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteListenersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/DeleteListeners",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "DeleteListeners"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_scheduled_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::StartScheduledCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartScheduledCallerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartScheduledCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartScheduledCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_scheduled_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::StartScheduledCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartScheduledCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StartScheduledCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StartScheduledCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_scheduled_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScheduledCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ScheduledCaller>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/GetScheduledCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "GetScheduledCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_scheduled_callers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScheduledCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListScheduledCallersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/ListScheduledCallers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "ListScheduledCallers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_scheduled_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelScheduledCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelScheduledCallerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/CancelScheduledCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "CancelScheduledCaller"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_call(
            &mut self,
            request: impl tonic::IntoRequest<super::StopCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopCall"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::StopCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopCalls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopCalls"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_all_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::StopAllCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/StopAllCalls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StopAllCalls"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_call(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferCallResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/TransferCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "TransferCall"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferCallsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/TransferCalls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "TransferCalls"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_call(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCallRequest>,
        ) -> std::result::Result<tonic::Response<super::Call>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/GetCall",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("ondewo.vtsi.Calls", "GetCall"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Calls/ListCalls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "ListCalls"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod calls_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CallsServer.
    #[async_trait]
    pub trait Calls: std::marker::Send + std::marker::Sync + 'static {
        async fn start_caller(
            &self,
            request: tonic::Request<super::StartCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCallerResponse>,
            tonic::Status,
        >;
        async fn start_callers(
            &self,
            request: tonic::Request<super::StartCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartCallersResponse>,
            tonic::Status,
        >;
        async fn list_callers(
            &self,
            request: tonic::Request<super::ListCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallersResponse>,
            tonic::Status,
        >;
        async fn get_caller(
            &self,
            request: tonic::Request<super::GetCallerRequest>,
        ) -> std::result::Result<tonic::Response<super::Caller>, tonic::Status>;
        async fn delete_caller(
            &self,
            request: tonic::Request<super::DeleteCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallerResponse>,
            tonic::Status,
        >;
        async fn delete_callers(
            &self,
            request: tonic::Request<super::DeleteCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallersResponse>,
            tonic::Status,
        >;
        async fn stop_caller(
            &self,
            request: tonic::Request<super::StopCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallerResponse>,
            tonic::Status,
        >;
        async fn stop_callers(
            &self,
            request: tonic::Request<super::StopCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallersResponse>,
            tonic::Status,
        >;
        async fn start_listener(
            &self,
            request: tonic::Request<super::StartListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartListenerResponse>,
            tonic::Status,
        >;
        async fn start_listeners(
            &self,
            request: tonic::Request<super::StartListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartListenersResponse>,
            tonic::Status,
        >;
        async fn stop_listener(
            &self,
            request: tonic::Request<super::StopListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopListenerResponse>,
            tonic::Status,
        >;
        async fn stop_listeners(
            &self,
            request: tonic::Request<super::StopListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopListenersResponse>,
            tonic::Status,
        >;
        async fn list_listeners(
            &self,
            request: tonic::Request<super::ListListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListListenersResponse>,
            tonic::Status,
        >;
        async fn get_listener(
            &self,
            request: tonic::Request<super::GetListenerRequest>,
        ) -> std::result::Result<tonic::Response<super::Listener>, tonic::Status>;
        async fn delete_listener(
            &self,
            request: tonic::Request<super::DeleteListenerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteListenerResponse>,
            tonic::Status,
        >;
        async fn delete_listeners(
            &self,
            request: tonic::Request<super::DeleteListenersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteListenersResponse>,
            tonic::Status,
        >;
        async fn start_scheduled_caller(
            &self,
            request: tonic::Request<super::StartScheduledCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartScheduledCallerResponse>,
            tonic::Status,
        >;
        async fn start_scheduled_callers(
            &self,
            request: tonic::Request<super::StartScheduledCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartScheduledCallersResponse>,
            tonic::Status,
        >;
        async fn get_scheduled_caller(
            &self,
            request: tonic::Request<super::GetScheduledCallerRequest>,
        ) -> std::result::Result<tonic::Response<super::ScheduledCaller>, tonic::Status>;
        async fn list_scheduled_callers(
            &self,
            request: tonic::Request<super::ListScheduledCallersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListScheduledCallersResponse>,
            tonic::Status,
        >;
        async fn cancel_scheduled_caller(
            &self,
            request: tonic::Request<super::CancelScheduledCallerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelScheduledCallerResponse>,
            tonic::Status,
        >;
        async fn stop_call(
            &self,
            request: tonic::Request<super::StopCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallResponse>,
            tonic::Status,
        >;
        async fn stop_calls(
            &self,
            request: tonic::Request<super::StopCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallsResponse>,
            tonic::Status,
        >;
        async fn stop_all_calls(
            &self,
            request: tonic::Request<super::StopAllCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopCallsResponse>,
            tonic::Status,
        >;
        async fn transfer_call(
            &self,
            request: tonic::Request<super::TransferCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferCallResponse>,
            tonic::Status,
        >;
        async fn transfer_calls(
            &self,
            request: tonic::Request<super::TransferCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TransferCallsResponse>,
            tonic::Status,
        >;
        async fn get_call(
            &self,
            request: tonic::Request<super::GetCallRequest>,
        ) -> std::result::Result<tonic::Response<super::Call>, tonic::Status>;
        async fn list_calls(
            &self,
            request: tonic::Request<super::ListCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CallsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CallsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CallsServer<T>
    where
        T: Calls,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/ondewo.vtsi.Calls/StartCaller" => {
                    #[allow(non_camel_case_types)]
                    struct StartCallerSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::StartCallerRequest>
                    for StartCallerSvc<T> {
                        type Response = super::StartCallerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StartCallers" => {
                    #[allow(non_camel_case_types)]
                    struct StartCallersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StartCallersRequest>
                    for StartCallersSvc<T> {
                        type Response = super::StartCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/ListCallers" => {
                    #[allow(non_camel_case_types)]
                    struct ListCallersSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::ListCallersRequest>
                    for ListCallersSvc<T> {
                        type Response = super::ListCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::list_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/GetCaller" => {
                    #[allow(non_camel_case_types)]
                    struct GetCallerSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::GetCallerRequest>
                    for GetCallerSvc<T> {
                        type Response = super::Caller;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::get_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/DeleteCaller" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCallerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::DeleteCallerRequest>
                    for DeleteCallerSvc<T> {
                        type Response = super::DeleteCallerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::delete_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/DeleteCallers" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCallersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::DeleteCallersRequest>
                    for DeleteCallersSvc<T> {
                        type Response = super::DeleteCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::delete_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopCaller" => {
                    #[allow(non_camel_case_types)]
                    struct StopCallerSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::StopCallerRequest>
                    for StopCallerSvc<T> {
                        type Response = super::StopCallerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopCallers" => {
                    #[allow(non_camel_case_types)]
                    struct StopCallersSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::StopCallersRequest>
                    for StopCallersSvc<T> {
                        type Response = super::StopCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StartListener" => {
                    #[allow(non_camel_case_types)]
                    struct StartListenerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StartListenerRequest>
                    for StartListenerSvc<T> {
                        type Response = super::StartListenerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartListenerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_listener(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartListenerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StartListeners" => {
                    #[allow(non_camel_case_types)]
                    struct StartListenersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StartListenersRequest>
                    for StartListenersSvc<T> {
                        type Response = super::StartListenersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartListenersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_listeners(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartListenersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopListener" => {
                    #[allow(non_camel_case_types)]
                    struct StopListenerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StopListenerRequest>
                    for StopListenerSvc<T> {
                        type Response = super::StopListenerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopListenerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_listener(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopListenerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopListeners" => {
                    #[allow(non_camel_case_types)]
                    struct StopListenersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StopListenersRequest>
                    for StopListenersSvc<T> {
                        type Response = super::StopListenersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopListenersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_listeners(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopListenersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/ListListeners" => {
                    #[allow(non_camel_case_types)]
                    struct ListListenersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::ListListenersRequest>
                    for ListListenersSvc<T> {
                        type Response = super::ListListenersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListListenersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::list_listeners(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListListenersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/GetListener" => {
                    #[allow(non_camel_case_types)]
                    struct GetListenerSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::GetListenerRequest>
                    for GetListenerSvc<T> {
                        type Response = super::Listener;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetListenerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::get_listener(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetListenerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/DeleteListener" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteListenerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::DeleteListenerRequest>
                    for DeleteListenerSvc<T> {
                        type Response = super::DeleteListenerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteListenerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::delete_listener(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteListenerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/DeleteListeners" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteListenersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::DeleteListenersRequest>
                    for DeleteListenersSvc<T> {
                        type Response = super::DeleteListenersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteListenersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::delete_listeners(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteListenersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StartScheduledCaller" => {
                    #[allow(non_camel_case_types)]
                    struct StartScheduledCallerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StartScheduledCallerRequest>
                    for StartScheduledCallerSvc<T> {
                        type Response = super::StartScheduledCallerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartScheduledCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_scheduled_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartScheduledCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StartScheduledCallers" => {
                    #[allow(non_camel_case_types)]
                    struct StartScheduledCallersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StartScheduledCallersRequest>
                    for StartScheduledCallersSvc<T> {
                        type Response = super::StartScheduledCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartScheduledCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::start_scheduled_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartScheduledCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/GetScheduledCaller" => {
                    #[allow(non_camel_case_types)]
                    struct GetScheduledCallerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::GetScheduledCallerRequest>
                    for GetScheduledCallerSvc<T> {
                        type Response = super::ScheduledCaller;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetScheduledCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::get_scheduled_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetScheduledCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/ListScheduledCallers" => {
                    #[allow(non_camel_case_types)]
                    struct ListScheduledCallersSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::ListScheduledCallersRequest>
                    for ListScheduledCallersSvc<T> {
                        type Response = super::ListScheduledCallersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListScheduledCallersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::list_scheduled_callers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListScheduledCallersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/CancelScheduledCaller" => {
                    #[allow(non_camel_case_types)]
                    struct CancelScheduledCallerSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::CancelScheduledCallerRequest>
                    for CancelScheduledCallerSvc<T> {
                        type Response = super::CancelScheduledCallerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelScheduledCallerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::cancel_scheduled_caller(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CancelScheduledCallerSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopCall" => {
                    #[allow(non_camel_case_types)]
                    struct StopCallSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::StopCallRequest>
                    for StopCallSvc<T> {
                        type Response = super::StopCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_call(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopCallSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopCalls" => {
                    #[allow(non_camel_case_types)]
                    struct StopCallsSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::StopCallsRequest>
                    for StopCallsSvc<T> {
                        type Response = super::StopCallsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopCallsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_calls(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopCallsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/StopAllCalls" => {
                    #[allow(non_camel_case_types)]
                    struct StopAllCallsSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::StopAllCallsRequest>
                    for StopAllCallsSvc<T> {
                        type Response = super::StopCallsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopAllCallsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stop_all_calls(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopAllCallsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/TransferCall" => {
                    #[allow(non_camel_case_types)]
                    struct TransferCallSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::TransferCallRequest>
                    for TransferCallSvc<T> {
                        type Response = super::TransferCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::transfer_call(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TransferCallSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/TransferCalls" => {
                    #[allow(non_camel_case_types)]
                    struct TransferCallsSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::TransferCallsRequest>
                    for TransferCallsSvc<T> {
                        type Response = super::TransferCallsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferCallsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::transfer_calls(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TransferCallsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/GetCall" => {
                    #[allow(non_camel_case_types)]
                    struct GetCallSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::GetCallRequest>
                    for GetCallSvc<T> {
                        type Response = super::Call;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::get_call(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCallSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/ListCalls" => {
                    #[allow(non_camel_case_types)]
                    struct ListCallsSvc<T: Calls>(pub Arc<T>);
                    impl<T: Calls> tonic::server::UnaryService<super::ListCallsRequest>
                    for ListCallsSvc<T> {
                        type Response = super::ListCallsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCallsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::list_calls(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListCallsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CallsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Calls";
    impl<T> tonic::server::NamedService for CallsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod logs_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LogsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LogsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LogsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LogsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            LogsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn stream_call_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamCallLogsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Logs/StreamCallLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Logs", "StreamCallLogs"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_call_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallLogsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Logs/ListCallLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Logs", "ListCallLogs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_call_log_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCallLogStreamRequest>,
        ) -> std::result::Result<tonic::Response<super::CallLogStream>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Logs/GetCallLogStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Logs", "GetCallLogStream"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_call_log_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCallLogStreamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallLogStreamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Logs/ListCallLogStreams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Logs", "ListCallLogStreams"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_call_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallLogsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Logs/DeleteCallLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Logs", "DeleteCallLogs"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod logs_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LogsServer.
    #[async_trait]
    pub trait Logs: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the StreamCallLogs method.
        type StreamCallLogsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StreamCallLogsResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn stream_call_logs(
            &self,
            request: tonic::Request<super::StreamCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamCallLogsStream>,
            tonic::Status,
        >;
        async fn list_call_logs(
            &self,
            request: tonic::Request<super::ListCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallLogsResponse>,
            tonic::Status,
        >;
        async fn get_call_log_stream(
            &self,
            request: tonic::Request<super::GetCallLogStreamRequest>,
        ) -> std::result::Result<tonic::Response<super::CallLogStream>, tonic::Status>;
        async fn list_call_log_streams(
            &self,
            request: tonic::Request<super::ListCallLogStreamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCallLogStreamsResponse>,
            tonic::Status,
        >;
        async fn delete_call_logs(
            &self,
            request: tonic::Request<super::DeleteCallLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCallLogsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LogsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> LogsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LogsServer<T>
    where
        T: Logs,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/ondewo.vtsi.Logs/StreamCallLogs" => {
                    #[allow(non_camel_case_types)]
                    struct StreamCallLogsSvc<T: Logs>(pub Arc<T>);
                    impl<
                        T: Logs,
                    > tonic::server::ServerStreamingService<super::StreamCallLogsRequest>
                    for StreamCallLogsSvc<T> {
                        type Response = super::StreamCallLogsResponse;
                        type ResponseStream = T::StreamCallLogsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamCallLogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Logs>::stream_call_logs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StreamCallLogsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Logs/ListCallLogs" => {
                    #[allow(non_camel_case_types)]
                    struct ListCallLogsSvc<T: Logs>(pub Arc<T>);
                    impl<T: Logs> tonic::server::UnaryService<super::ListCallLogsRequest>
                    for ListCallLogsSvc<T> {
                        type Response = super::ListCallLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCallLogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Logs>::list_call_logs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListCallLogsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Logs/GetCallLogStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetCallLogStreamSvc<T: Logs>(pub Arc<T>);
                    impl<
                        T: Logs,
                    > tonic::server::UnaryService<super::GetCallLogStreamRequest>
                    for GetCallLogStreamSvc<T> {
                        type Response = super::CallLogStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCallLogStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Logs>::get_call_log_stream(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetCallLogStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Logs/ListCallLogStreams" => {
                    #[allow(non_camel_case_types)]
                    struct ListCallLogStreamsSvc<T: Logs>(pub Arc<T>);
                    impl<
                        T: Logs,
                    > tonic::server::UnaryService<super::ListCallLogStreamsRequest>
                    for ListCallLogStreamsSvc<T> {
                        type Response = super::ListCallLogStreamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCallLogStreamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Logs>::list_call_log_streams(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListCallLogStreamsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Logs/DeleteCallLogs" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCallLogsSvc<T: Logs>(pub Arc<T>);
                    impl<
                        T: Logs,
                    > tonic::server::UnaryService<super::DeleteCallLogsRequest>
                    for DeleteCallLogsSvc<T> {
                        type Response = super::DeleteCallLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCallLogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Logs>::delete_call_logs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteCallLogsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for LogsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Logs";
    impl<T> tonic::server::NamedService for LogsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
/// Generated client implementations.
pub mod projects_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProjectsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProjectsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProjectsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ProjectsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVtsiProjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/CreateVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "CreateVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVtsiProjectRequest>,
        ) -> std::result::Result<tonic::Response<super::VtsiProject>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/GetVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "GetVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVtsiProjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/UpdateVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "UpdateVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteVtsiProjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/DeleteVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "DeleteVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deploy_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeployVtsiProjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/DeployVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "DeployVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn undeploy_vtsi_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UndeployVtsiProjectResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/UndeployVtsiProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "UndeployVtsiProject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_vtsi_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVtsiProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVtsiProjectsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.vtsi.Projects/ListVtsiProjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Projects", "ListVtsiProjects"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod projects_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProjectsServer.
    #[async_trait]
    pub trait Projects: std::marker::Send + std::marker::Sync + 'static {
        async fn create_vtsi_project(
            &self,
            request: tonic::Request<super::CreateVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVtsiProjectResponse>,
            tonic::Status,
        >;
        async fn get_vtsi_project(
            &self,
            request: tonic::Request<super::GetVtsiProjectRequest>,
        ) -> std::result::Result<tonic::Response<super::VtsiProject>, tonic::Status>;
        async fn update_vtsi_project(
            &self,
            request: tonic::Request<super::UpdateVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVtsiProjectResponse>,
            tonic::Status,
        >;
        async fn delete_vtsi_project(
            &self,
            request: tonic::Request<super::DeleteVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteVtsiProjectResponse>,
            tonic::Status,
        >;
        async fn deploy_vtsi_project(
            &self,
            request: tonic::Request<super::DeployVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeployVtsiProjectResponse>,
            tonic::Status,
        >;
        async fn undeploy_vtsi_project(
            &self,
            request: tonic::Request<super::UndeployVtsiProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UndeployVtsiProjectResponse>,
            tonic::Status,
        >;
        async fn list_vtsi_projects(
            &self,
            request: tonic::Request<super::ListVtsiProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVtsiProjectsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ProjectsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ProjectsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProjectsServer<T>
    where
        T: Projects,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/ondewo.vtsi.Projects/CreateVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::CreateVtsiProjectRequest>
                    for CreateVtsiProjectSvc<T> {
                        type Response = super::CreateVtsiProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::create_vtsi_project(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/GetVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct GetVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::GetVtsiProjectRequest>
                    for GetVtsiProjectSvc<T> {
                        type Response = super::VtsiProject;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::get_vtsi_project(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/UpdateVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::UpdateVtsiProjectRequest>
                    for UpdateVtsiProjectSvc<T> {
                        type Response = super::UpdateVtsiProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::update_vtsi_project(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/DeleteVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::DeleteVtsiProjectRequest>
                    for DeleteVtsiProjectSvc<T> {
                        type Response = super::DeleteVtsiProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::delete_vtsi_project(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/DeployVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeployVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::DeployVtsiProjectRequest>
                    for DeployVtsiProjectSvc<T> {
                        type Response = super::DeployVtsiProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeployVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::deploy_vtsi_project(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeployVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/UndeployVtsiProject" => {
                    #[allow(non_camel_case_types)]
                    struct UndeployVtsiProjectSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::UndeployVtsiProjectRequest>
                    for UndeployVtsiProjectSvc<T> {
                        type Response = super::UndeployVtsiProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UndeployVtsiProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::undeploy_vtsi_project(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UndeployVtsiProjectSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Projects/ListVtsiProjects" => {
                    #[allow(non_camel_case_types)]
                    struct ListVtsiProjectsSvc<T: Projects>(pub Arc<T>);
                    impl<
                        T: Projects,
                    > tonic::server::UnaryService<super::ListVtsiProjectsRequest>
                    for ListVtsiProjectsSvc<T> {
                        type Response = super::ListVtsiProjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVtsiProjectsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Projects>::list_vtsi_projects(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListVtsiProjectsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ProjectsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Projects";
    impl<T> tonic::server::NamedService for ProjectsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
