// @generated
/// Generated client implementations.
pub mod campaigns_client {
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
    pub struct CampaignsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CampaignsClient<tonic::transport::Channel> {
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
    impl<T> CampaignsClient<T>
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
        ) -> CampaignsClient<InterceptedService<T, F>>
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
            CampaignsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/CreateCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "CreateCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/GetCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "GetCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/UpdateCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "UpdateCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCampaignResponse>,
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
                "/ondewo.vtsi.Campaigns/DeleteCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "DeleteCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_campaigns(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignsResponse>,
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
                "/ondewo.vtsi.Campaigns/ListCampaigns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "ListCampaigns"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_campaign_statistics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignStatistics>,
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
                "/ondewo.vtsi.Campaigns/GetCampaignStatistics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Campaigns", "GetCampaignStatistics"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_campaign_calls(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCampaignCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignCallsResponse>,
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
                "/ondewo.vtsi.Campaigns/ListCampaignCalls",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "ListCampaignCalls"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::StartCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/StartCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "StartCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::StopCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/StopCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "StopCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn hard_stop_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::HardStopCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/HardStopCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "HardStopCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status> {
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
                "/ondewo.vtsi.Campaigns/ResumeCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Campaigns", "ResumeCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_campaign_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamCampaignStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamCampaignStatusResponse>,
            >,
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
                "/ondewo.vtsi.Campaigns/StreamCampaignStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Campaigns", "StreamCampaignStatus"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod campaigns_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CampaignsServer.
    #[async_trait]
    pub trait Campaigns: std::marker::Send + std::marker::Sync + 'static {
        async fn create_campaign(
            &self,
            request: tonic::Request<super::CreateCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn get_campaign(
            &self,
            request: tonic::Request<super::GetCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn update_campaign(
            &self,
            request: tonic::Request<super::UpdateCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn delete_campaign(
            &self,
            request: tonic::Request<super::DeleteCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCampaignResponse>,
            tonic::Status,
        >;
        async fn list_campaigns(
            &self,
            request: tonic::Request<super::ListCampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignsResponse>,
            tonic::Status,
        >;
        async fn get_campaign_statistics(
            &self,
            request: tonic::Request<super::GetCampaignStatisticsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignStatistics>,
            tonic::Status,
        >;
        async fn list_campaign_calls(
            &self,
            request: tonic::Request<super::ListCampaignCallsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCampaignCallsResponse>,
            tonic::Status,
        >;
        async fn start_campaign(
            &self,
            request: tonic::Request<super::StartCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn stop_campaign(
            &self,
            request: tonic::Request<super::StopCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn hard_stop_campaign(
            &self,
            request: tonic::Request<super::HardStopCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        async fn resume_campaign(
            &self,
            request: tonic::Request<super::ResumeCampaignRequest>,
        ) -> std::result::Result<tonic::Response<super::Campaign>, tonic::Status>;
        /// Server streaming response type for the StreamCampaignStatus method.
        type StreamCampaignStatusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamCampaignStatusResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn stream_campaign_status(
            &self,
            request: tonic::Request<super::StreamCampaignStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamCampaignStatusStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CampaignsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CampaignsServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CampaignsServer<T>
    where
        T: Campaigns,
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
                "/ondewo.vtsi.Campaigns/CreateCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::CreateCampaignRequest>
                    for CreateCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::create_campaign(&inner, request).await
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
                        let method = CreateCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/GetCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct GetCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::GetCampaignRequest>
                    for GetCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::get_campaign(&inner, request).await
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
                        let method = GetCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/UpdateCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::UpdateCampaignRequest>
                    for UpdateCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::update_campaign(&inner, request).await
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
                        let method = UpdateCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/DeleteCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::DeleteCampaignRequest>
                    for DeleteCampaignSvc<T> {
                        type Response = super::DeleteCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::delete_campaign(&inner, request).await
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
                        let method = DeleteCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/ListCampaigns" => {
                    #[allow(non_camel_case_types)]
                    struct ListCampaignsSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::ListCampaignsRequest>
                    for ListCampaignsSvc<T> {
                        type Response = super::ListCampaignsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCampaignsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::list_campaigns(&inner, request).await
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
                        let method = ListCampaignsSvc(inner);
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
                "/ondewo.vtsi.Campaigns/GetCampaignStatistics" => {
                    #[allow(non_camel_case_types)]
                    struct GetCampaignStatisticsSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::GetCampaignStatisticsRequest>
                    for GetCampaignStatisticsSvc<T> {
                        type Response = super::CampaignStatistics;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCampaignStatisticsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::get_campaign_statistics(&inner, request)
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
                        let method = GetCampaignStatisticsSvc(inner);
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
                "/ondewo.vtsi.Campaigns/ListCampaignCalls" => {
                    #[allow(non_camel_case_types)]
                    struct ListCampaignCallsSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::ListCampaignCallsRequest>
                    for ListCampaignCallsSvc<T> {
                        type Response = super::ListCampaignCallsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCampaignCallsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::list_campaign_calls(&inner, request).await
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
                        let method = ListCampaignCallsSvc(inner);
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
                "/ondewo.vtsi.Campaigns/StartCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct StartCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::StartCampaignRequest>
                    for StartCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::start_campaign(&inner, request).await
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
                        let method = StartCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/StopCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct StopCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::StopCampaignRequest>
                    for StopCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::stop_campaign(&inner, request).await
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
                        let method = StopCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/HardStopCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct HardStopCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::HardStopCampaignRequest>
                    for HardStopCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HardStopCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::hard_stop_campaign(&inner, request).await
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
                        let method = HardStopCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/ResumeCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeCampaignSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::UnaryService<super::ResumeCampaignRequest>
                    for ResumeCampaignSvc<T> {
                        type Response = super::Campaign;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::resume_campaign(&inner, request).await
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
                        let method = ResumeCampaignSvc(inner);
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
                "/ondewo.vtsi.Campaigns/StreamCampaignStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamCampaignStatusSvc<T: Campaigns>(pub Arc<T>);
                    impl<
                        T: Campaigns,
                    > tonic::server::ServerStreamingService<
                        super::StreamCampaignStatusRequest,
                    > for StreamCampaignStatusSvc<T> {
                        type Response = super::StreamCampaignStatusResponse;
                        type ResponseStream = T::StreamCampaignStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamCampaignStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Campaigns>::stream_campaign_status(&inner, request)
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
                        let method = StreamCampaignStatusSvc(inner);
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
    impl<T> Clone for CampaignsServer<T> {
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
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Campaigns";
    impl<T> tonic::server::NamedService for CampaignsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
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
        pub async fn add_callers_to_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCallersToCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCallersToCampaignResponse>,
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
                "/ondewo.vtsi.Calls/AddCallersToCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "AddCallersToCampaign"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_scheduled_callers_to_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::AddScheduledCallersToCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddScheduledCallersToCampaignResponse>,
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
                "/ondewo.vtsi.Calls/AddScheduledCallersToCampaign",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Calls", "AddScheduledCallersToCampaign"),
                );
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
        pub async fn stream_caller_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamCallerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamCallResourceStatusResponse>,
            >,
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
                "/ondewo.vtsi.Calls/StreamCallerStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StreamCallerStatus"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn stream_listener_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamListenerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamCallResourceStatusResponse>,
            >,
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
                "/ondewo.vtsi.Calls/StreamListenerStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StreamListenerStatus"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn stream_scheduled_caller_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamScheduledCallerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::StreamCallResourceStatusResponse>,
            >,
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
                "/ondewo.vtsi.Calls/StreamScheduledCallerStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Calls", "StreamScheduledCallerStatus"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn invite_to_call(
            &mut self,
            request: impl tonic::IntoRequest<super::InviteToCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteToCallResponse>,
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
                "/ondewo.vtsi.Calls/InviteToCall",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "InviteToCall"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_call_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCallParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCallParticipantResponse>,
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
                "/ondewo.vtsi.Calls/RemoveCallParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "RemoveCallParticipant"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_call_media_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCallMediaControlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetCallMediaControlResponse>,
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
                "/ondewo.vtsi.Calls/SetCallMediaControl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "SetCallMediaControl"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_call_audio(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamCallAudioRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamCallAudioResponse>>,
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
                "/ondewo.vtsi.Calls/StreamCallAudio",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "StreamCallAudio"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn listen_call_audio(
            &mut self,
            request: impl tonic::IntoRequest<super::ListenCallAudioRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamCallAudioResponse>>,
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
                "/ondewo.vtsi.Calls/ListenCallAudio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Calls", "ListenCallAudio"));
            self.inner.server_streaming(req, path, codec).await
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
        async fn add_callers_to_campaign(
            &self,
            request: tonic::Request<super::AddCallersToCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddCallersToCampaignResponse>,
            tonic::Status,
        >;
        async fn add_scheduled_callers_to_campaign(
            &self,
            request: tonic::Request<super::AddScheduledCallersToCampaignRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddScheduledCallersToCampaignResponse>,
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
        /// Server streaming response type for the StreamCallerStatus method.
        type StreamCallerStatusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamCallResourceStatusResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn stream_caller_status(
            &self,
            request: tonic::Request<super::StreamCallerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamCallerStatusStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamListenerStatus method.
        type StreamListenerStatusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamCallResourceStatusResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn stream_listener_status(
            &self,
            request: tonic::Request<super::StreamListenerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamListenerStatusStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamScheduledCallerStatus method.
        type StreamScheduledCallerStatusStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::StreamCallResourceStatusResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn stream_scheduled_caller_status(
            &self,
            request: tonic::Request<super::StreamScheduledCallerStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamScheduledCallerStatusStream>,
            tonic::Status,
        >;
        async fn invite_to_call(
            &self,
            request: tonic::Request<super::InviteToCallRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InviteToCallResponse>,
            tonic::Status,
        >;
        async fn remove_call_participant(
            &self,
            request: tonic::Request<super::RemoveCallParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveCallParticipantResponse>,
            tonic::Status,
        >;
        async fn set_call_media_control(
            &self,
            request: tonic::Request<super::SetCallMediaControlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetCallMediaControlResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamCallAudio method.
        type StreamCallAudioStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StreamCallAudioResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn stream_call_audio(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamCallAudioRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamCallAudioStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListenCallAudio method.
        type ListenCallAudioStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StreamCallAudioResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn listen_call_audio(
            &self,
            request: tonic::Request<super::ListenCallAudioRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListenCallAudioStream>,
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
                "/ondewo.vtsi.Calls/AddCallersToCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct AddCallersToCampaignSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::AddCallersToCampaignRequest>
                    for AddCallersToCampaignSvc<T> {
                        type Response = super::AddCallersToCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCallersToCampaignRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::add_callers_to_campaign(&inner, request).await
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
                        let method = AddCallersToCampaignSvc(inner);
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
                "/ondewo.vtsi.Calls/AddScheduledCallersToCampaign" => {
                    #[allow(non_camel_case_types)]
                    struct AddScheduledCallersToCampaignSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<
                        super::AddScheduledCallersToCampaignRequest,
                    > for AddScheduledCallersToCampaignSvc<T> {
                        type Response = super::AddScheduledCallersToCampaignResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddScheduledCallersToCampaignRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::add_scheduled_callers_to_campaign(
                                        &inner,
                                        request,
                                    )
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
                        let method = AddScheduledCallersToCampaignSvc(inner);
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
                "/ondewo.vtsi.Calls/StreamCallerStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamCallerStatusSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::ServerStreamingService<
                        super::StreamCallerStatusRequest,
                    > for StreamCallerStatusSvc<T> {
                        type Response = super::StreamCallResourceStatusResponse;
                        type ResponseStream = T::StreamCallerStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamCallerStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stream_caller_status(&inner, request).await
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
                        let method = StreamCallerStatusSvc(inner);
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
                "/ondewo.vtsi.Calls/StreamListenerStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamListenerStatusSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::ServerStreamingService<
                        super::StreamListenerStatusRequest,
                    > for StreamListenerStatusSvc<T> {
                        type Response = super::StreamCallResourceStatusResponse;
                        type ResponseStream = T::StreamListenerStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamListenerStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stream_listener_status(&inner, request).await
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
                        let method = StreamListenerStatusSvc(inner);
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
                "/ondewo.vtsi.Calls/StreamScheduledCallerStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamScheduledCallerStatusSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::ServerStreamingService<
                        super::StreamScheduledCallerStatusRequest,
                    > for StreamScheduledCallerStatusSvc<T> {
                        type Response = super::StreamCallResourceStatusResponse;
                        type ResponseStream = T::StreamScheduledCallerStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::StreamScheduledCallerStatusRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stream_scheduled_caller_status(
                                        &inner,
                                        request,
                                    )
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
                        let method = StreamScheduledCallerStatusSvc(inner);
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
                "/ondewo.vtsi.Calls/InviteToCall" => {
                    #[allow(non_camel_case_types)]
                    struct InviteToCallSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::InviteToCallRequest>
                    for InviteToCallSvc<T> {
                        type Response = super::InviteToCallResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InviteToCallRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::invite_to_call(&inner, request).await
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
                        let method = InviteToCallSvc(inner);
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
                "/ondewo.vtsi.Calls/RemoveCallParticipant" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveCallParticipantSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::RemoveCallParticipantRequest>
                    for RemoveCallParticipantSvc<T> {
                        type Response = super::RemoveCallParticipantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveCallParticipantRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::remove_call_participant(&inner, request).await
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
                        let method = RemoveCallParticipantSvc(inner);
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
                "/ondewo.vtsi.Calls/SetCallMediaControl" => {
                    #[allow(non_camel_case_types)]
                    struct SetCallMediaControlSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::UnaryService<super::SetCallMediaControlRequest>
                    for SetCallMediaControlSvc<T> {
                        type Response = super::SetCallMediaControlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetCallMediaControlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::set_call_media_control(&inner, request).await
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
                        let method = SetCallMediaControlSvc(inner);
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
                "/ondewo.vtsi.Calls/StreamCallAudio" => {
                    #[allow(non_camel_case_types)]
                    struct StreamCallAudioSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::StreamingService<super::StreamCallAudioRequest>
                    for StreamCallAudioSvc<T> {
                        type Response = super::StreamCallAudioResponse;
                        type ResponseStream = T::StreamCallAudioStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::StreamCallAudioRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::stream_call_audio(&inner, request).await
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
                        let method = StreamCallAudioSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.vtsi.Calls/ListenCallAudio" => {
                    #[allow(non_camel_case_types)]
                    struct ListenCallAudioSvc<T: Calls>(pub Arc<T>);
                    impl<
                        T: Calls,
                    > tonic::server::ServerStreamingService<
                        super::ListenCallAudioRequest,
                    > for ListenCallAudioSvc<T> {
                        type Response = super::StreamCallAudioResponse;
                        type ResponseStream = T::ListenCallAudioStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListenCallAudioRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Calls>::listen_call_audio(&inner, request).await
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
                        let method = ListenCallAudioSvc(inner);
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
pub mod events_client {
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
    pub struct EventsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventsClient<tonic::transport::Channel> {
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
    impl<T> EventsClient<T>
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
        ) -> EventsClient<InterceptedService<T, F>>
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
            EventsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_vtsi_event_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
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
                "/ondewo.vtsi.Events/CreateVtsiEventSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Events", "CreateVtsiEventSubscription"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_vtsi_event_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
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
                "/ondewo.vtsi.Events/GetVtsiEventSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Events", "GetVtsiEventSubscription"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_vtsi_event_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
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
                "/ondewo.vtsi.Events/UpdateVtsiEventSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Events", "UpdateVtsiEventSubscription"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_vtsi_event_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteVtsiEventSubscriptionResponse>,
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
                "/ondewo.vtsi.Events/DeleteVtsiEventSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Events", "DeleteVtsiEventSubscription"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_vtsi_event_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVtsiEventSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVtsiEventSubscriptionsResponse>,
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
                "/ondewo.vtsi.Events/ListVtsiEventSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Events", "ListVtsiEventSubscriptions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status> {
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
                "/ondewo.vtsi.Events/CreateWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "CreateWebhook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status> {
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
                "/ondewo.vtsi.Events/GetWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "GetWebhook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status> {
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
                "/ondewo.vtsi.Events/UpdateWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "UpdateWebhook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteWebhookResponse>,
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
                "/ondewo.vtsi.Events/DeleteWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "DeleteWebhook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_webhooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebhooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhooksResponse>,
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
                "/ondewo.vtsi.Events/ListWebhooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "ListWebhooks"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn test_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::TestWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestWebhookResponse>,
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
                "/ondewo.vtsi.Events/TestWebhook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "TestWebhook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscribe_vtsi_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeVtsiEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SubscribeVtsiEventsResponse>>,
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
                "/ondewo.vtsi.Events/SubscribeVtsiEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.vtsi.Events", "SubscribeVtsiEvents"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod events_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EventsServer.
    #[async_trait]
    pub trait Events: std::marker::Send + std::marker::Sync + 'static {
        async fn create_vtsi_event_subscription(
            &self,
            request: tonic::Request<super::CreateVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
            tonic::Status,
        >;
        async fn get_vtsi_event_subscription(
            &self,
            request: tonic::Request<super::GetVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
            tonic::Status,
        >;
        async fn update_vtsi_event_subscription(
            &self,
            request: tonic::Request<super::UpdateVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtsiEventSubscription>,
            tonic::Status,
        >;
        async fn delete_vtsi_event_subscription(
            &self,
            request: tonic::Request<super::DeleteVtsiEventSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteVtsiEventSubscriptionResponse>,
            tonic::Status,
        >;
        async fn list_vtsi_event_subscriptions(
            &self,
            request: tonic::Request<super::ListVtsiEventSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVtsiEventSubscriptionsResponse>,
            tonic::Status,
        >;
        async fn create_webhook(
            &self,
            request: tonic::Request<super::CreateWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status>;
        async fn get_webhook(
            &self,
            request: tonic::Request<super::GetWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status>;
        async fn update_webhook(
            &self,
            request: tonic::Request<super::UpdateWebhookRequest>,
        ) -> std::result::Result<tonic::Response<super::Webhook>, tonic::Status>;
        async fn delete_webhook(
            &self,
            request: tonic::Request<super::DeleteWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteWebhookResponse>,
            tonic::Status,
        >;
        async fn list_webhooks(
            &self,
            request: tonic::Request<super::ListWebhooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWebhooksResponse>,
            tonic::Status,
        >;
        async fn test_webhook(
            &self,
            request: tonic::Request<super::TestWebhookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestWebhookResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the SubscribeVtsiEvents method.
        type SubscribeVtsiEventsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::SubscribeVtsiEventsResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn subscribe_vtsi_events(
            &self,
            request: tonic::Request<super::SubscribeVtsiEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SubscribeVtsiEventsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct EventsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> EventsServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EventsServer<T>
    where
        T: Events,
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
                "/ondewo.vtsi.Events/CreateVtsiEventSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVtsiEventSubscriptionSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<
                        super::CreateVtsiEventSubscriptionRequest,
                    > for CreateVtsiEventSubscriptionSvc<T> {
                        type Response = super::VtsiEventSubscription;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateVtsiEventSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::create_vtsi_event_subscription(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateVtsiEventSubscriptionSvc(inner);
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
                "/ondewo.vtsi.Events/GetVtsiEventSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct GetVtsiEventSubscriptionSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::GetVtsiEventSubscriptionRequest>
                    for GetVtsiEventSubscriptionSvc<T> {
                        type Response = super::VtsiEventSubscription;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetVtsiEventSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::get_vtsi_event_subscription(&inner, request)
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
                        let method = GetVtsiEventSubscriptionSvc(inner);
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
                "/ondewo.vtsi.Events/UpdateVtsiEventSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVtsiEventSubscriptionSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<
                        super::UpdateVtsiEventSubscriptionRequest,
                    > for UpdateVtsiEventSubscriptionSvc<T> {
                        type Response = super::VtsiEventSubscription;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateVtsiEventSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::update_vtsi_event_subscription(
                                        &inner,
                                        request,
                                    )
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
                        let method = UpdateVtsiEventSubscriptionSvc(inner);
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
                "/ondewo.vtsi.Events/DeleteVtsiEventSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVtsiEventSubscriptionSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<
                        super::DeleteVtsiEventSubscriptionRequest,
                    > for DeleteVtsiEventSubscriptionSvc<T> {
                        type Response = super::DeleteVtsiEventSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteVtsiEventSubscriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::delete_vtsi_event_subscription(
                                        &inner,
                                        request,
                                    )
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
                        let method = DeleteVtsiEventSubscriptionSvc(inner);
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
                "/ondewo.vtsi.Events/ListVtsiEventSubscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct ListVtsiEventSubscriptionsSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<
                        super::ListVtsiEventSubscriptionsRequest,
                    > for ListVtsiEventSubscriptionsSvc<T> {
                        type Response = super::ListVtsiEventSubscriptionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListVtsiEventSubscriptionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::list_vtsi_event_subscriptions(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListVtsiEventSubscriptionsSvc(inner);
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
                "/ondewo.vtsi.Events/CreateWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWebhookSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::CreateWebhookRequest>
                    for CreateWebhookSvc<T> {
                        type Response = super::Webhook;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::create_webhook(&inner, request).await
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
                        let method = CreateWebhookSvc(inner);
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
                "/ondewo.vtsi.Events/GetWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct GetWebhookSvc<T: Events>(pub Arc<T>);
                    impl<T: Events> tonic::server::UnaryService<super::GetWebhookRequest>
                    for GetWebhookSvc<T> {
                        type Response = super::Webhook;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::get_webhook(&inner, request).await
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
                        let method = GetWebhookSvc(inner);
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
                "/ondewo.vtsi.Events/UpdateWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWebhookSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::UpdateWebhookRequest>
                    for UpdateWebhookSvc<T> {
                        type Response = super::Webhook;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::update_webhook(&inner, request).await
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
                        let method = UpdateWebhookSvc(inner);
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
                "/ondewo.vtsi.Events/DeleteWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWebhookSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::DeleteWebhookRequest>
                    for DeleteWebhookSvc<T> {
                        type Response = super::DeleteWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::delete_webhook(&inner, request).await
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
                        let method = DeleteWebhookSvc(inner);
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
                "/ondewo.vtsi.Events/ListWebhooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListWebhooksSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::ListWebhooksRequest>
                    for ListWebhooksSvc<T> {
                        type Response = super::ListWebhooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWebhooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::list_webhooks(&inner, request).await
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
                        let method = ListWebhooksSvc(inner);
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
                "/ondewo.vtsi.Events/TestWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct TestWebhookSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::UnaryService<super::TestWebhookRequest>
                    for TestWebhookSvc<T> {
                        type Response = super::TestWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TestWebhookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::test_webhook(&inner, request).await
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
                        let method = TestWebhookSvc(inner);
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
                "/ondewo.vtsi.Events/SubscribeVtsiEvents" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeVtsiEventsSvc<T: Events>(pub Arc<T>);
                    impl<
                        T: Events,
                    > tonic::server::ServerStreamingService<
                        super::SubscribeVtsiEventsRequest,
                    > for SubscribeVtsiEventsSvc<T> {
                        type Response = super::SubscribeVtsiEventsResponse;
                        type ResponseStream = T::SubscribeVtsiEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeVtsiEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Events>::subscribe_vtsi_events(&inner, request).await
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
                        let method = SubscribeVtsiEventsSvc(inner);
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
    impl<T> Clone for EventsServer<T> {
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
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Events";
    impl<T> tonic::server::NamedService for EventsServer<T> {
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
/// Generated client implementations.
pub mod softphones_client {
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
    pub struct SoftphonesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SoftphonesClient<tonic::transport::Channel> {
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
    impl<T> SoftphonesClient<T>
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
        ) -> SoftphonesClient<InterceptedService<T, F>>
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
            SoftphonesClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_softphone_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSoftphoneAccountResponse>,
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
                "/ondewo.vtsi.Softphones/CreateSoftphoneAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "CreateSoftphoneAccount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_softphone_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneAccount>,
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
                "/ondewo.vtsi.Softphones/GetSoftphoneAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "GetSoftphoneAccount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_softphone_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneAccount>,
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
                "/ondewo.vtsi.Softphones/UpdateSoftphoneAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "UpdateSoftphoneAccount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_softphone_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSoftphoneAccountResponse>,
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
                "/ondewo.vtsi.Softphones/DeleteSoftphoneAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "DeleteSoftphoneAccount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_softphone_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSoftphoneAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSoftphoneAccountsResponse>,
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
                "/ondewo.vtsi.Softphones/ListSoftphoneAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "ListSoftphoneAccounts"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rotate_softphone_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::RotateSoftphoneCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateSoftphoneCredentialsResponse>,
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
                "/ondewo.vtsi.Softphones/RotateSoftphoneCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ondewo.vtsi.Softphones",
                        "RotateSoftphoneCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_softphone_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSoftphoneCertificatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSoftphoneCertificatesResponse>,
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
                "/ondewo.vtsi.Softphones/ListSoftphoneCertificates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ondewo.vtsi.Softphones",
                        "ListSoftphoneCertificates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_softphone_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSoftphoneCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneCertificate>,
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
                "/ondewo.vtsi.Softphones/GetSoftphoneCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "GetSoftphoneCertificate"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn revoke_softphone_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeSoftphoneCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneCertificate>,
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
                "/ondewo.vtsi.Softphones/RevokeSoftphoneCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ondewo.vtsi.Softphones",
                        "RevokeSoftphoneCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_softphone_provisioning(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSoftphoneProvisioningRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneProvisioning>,
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
                "/ondewo.vtsi.Softphones/GetSoftphoneProvisioning",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.vtsi.Softphones", "GetSoftphoneProvisioning"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod softphones_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SoftphonesServer.
    #[async_trait]
    pub trait Softphones: std::marker::Send + std::marker::Sync + 'static {
        async fn create_softphone_account(
            &self,
            request: tonic::Request<super::CreateSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSoftphoneAccountResponse>,
            tonic::Status,
        >;
        async fn get_softphone_account(
            &self,
            request: tonic::Request<super::GetSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneAccount>,
            tonic::Status,
        >;
        async fn update_softphone_account(
            &self,
            request: tonic::Request<super::UpdateSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneAccount>,
            tonic::Status,
        >;
        async fn delete_softphone_account(
            &self,
            request: tonic::Request<super::DeleteSoftphoneAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteSoftphoneAccountResponse>,
            tonic::Status,
        >;
        async fn list_softphone_accounts(
            &self,
            request: tonic::Request<super::ListSoftphoneAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSoftphoneAccountsResponse>,
            tonic::Status,
        >;
        async fn rotate_softphone_credentials(
            &self,
            request: tonic::Request<super::RotateSoftphoneCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateSoftphoneCredentialsResponse>,
            tonic::Status,
        >;
        async fn list_softphone_certificates(
            &self,
            request: tonic::Request<super::ListSoftphoneCertificatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSoftphoneCertificatesResponse>,
            tonic::Status,
        >;
        async fn get_softphone_certificate(
            &self,
            request: tonic::Request<super::GetSoftphoneCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneCertificate>,
            tonic::Status,
        >;
        async fn revoke_softphone_certificate(
            &self,
            request: tonic::Request<super::RevokeSoftphoneCertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneCertificate>,
            tonic::Status,
        >;
        async fn get_softphone_provisioning(
            &self,
            request: tonic::Request<super::GetSoftphoneProvisioningRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SoftphoneProvisioning>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SoftphonesServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> SoftphonesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SoftphonesServer<T>
    where
        T: Softphones,
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
                "/ondewo.vtsi.Softphones/CreateSoftphoneAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSoftphoneAccountSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::CreateSoftphoneAccountRequest>
                    for CreateSoftphoneAccountSvc<T> {
                        type Response = super::CreateSoftphoneAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSoftphoneAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::create_softphone_account(&inner, request)
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
                        let method = CreateSoftphoneAccountSvc(inner);
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
                "/ondewo.vtsi.Softphones/GetSoftphoneAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetSoftphoneAccountSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::GetSoftphoneAccountRequest>
                    for GetSoftphoneAccountSvc<T> {
                        type Response = super::SoftphoneAccount;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSoftphoneAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::get_softphone_account(&inner, request)
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
                        let method = GetSoftphoneAccountSvc(inner);
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
                "/ondewo.vtsi.Softphones/UpdateSoftphoneAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSoftphoneAccountSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::UpdateSoftphoneAccountRequest>
                    for UpdateSoftphoneAccountSvc<T> {
                        type Response = super::SoftphoneAccount;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSoftphoneAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::update_softphone_account(&inner, request)
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
                        let method = UpdateSoftphoneAccountSvc(inner);
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
                "/ondewo.vtsi.Softphones/DeleteSoftphoneAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSoftphoneAccountSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::DeleteSoftphoneAccountRequest>
                    for DeleteSoftphoneAccountSvc<T> {
                        type Response = super::DeleteSoftphoneAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSoftphoneAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::delete_softphone_account(&inner, request)
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
                        let method = DeleteSoftphoneAccountSvc(inner);
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
                "/ondewo.vtsi.Softphones/ListSoftphoneAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListSoftphoneAccountsSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::ListSoftphoneAccountsRequest>
                    for ListSoftphoneAccountsSvc<T> {
                        type Response = super::ListSoftphoneAccountsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSoftphoneAccountsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::list_softphone_accounts(&inner, request)
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
                        let method = ListSoftphoneAccountsSvc(inner);
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
                "/ondewo.vtsi.Softphones/RotateSoftphoneCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct RotateSoftphoneCredentialsSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<
                        super::RotateSoftphoneCredentialsRequest,
                    > for RotateSoftphoneCredentialsSvc<T> {
                        type Response = super::RotateSoftphoneCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RotateSoftphoneCredentialsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::rotate_softphone_credentials(
                                        &inner,
                                        request,
                                    )
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
                        let method = RotateSoftphoneCredentialsSvc(inner);
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
                "/ondewo.vtsi.Softphones/ListSoftphoneCertificates" => {
                    #[allow(non_camel_case_types)]
                    struct ListSoftphoneCertificatesSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<
                        super::ListSoftphoneCertificatesRequest,
                    > for ListSoftphoneCertificatesSvc<T> {
                        type Response = super::ListSoftphoneCertificatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListSoftphoneCertificatesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::list_softphone_certificates(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListSoftphoneCertificatesSvc(inner);
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
                "/ondewo.vtsi.Softphones/GetSoftphoneCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct GetSoftphoneCertificateSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::GetSoftphoneCertificateRequest>
                    for GetSoftphoneCertificateSvc<T> {
                        type Response = super::SoftphoneCertificate;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetSoftphoneCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::get_softphone_certificate(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetSoftphoneCertificateSvc(inner);
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
                "/ondewo.vtsi.Softphones/RevokeSoftphoneCertificate" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeSoftphoneCertificateSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<
                        super::RevokeSoftphoneCertificateRequest,
                    > for RevokeSoftphoneCertificateSvc<T> {
                        type Response = super::SoftphoneCertificate;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RevokeSoftphoneCertificateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::revoke_softphone_certificate(
                                        &inner,
                                        request,
                                    )
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
                        let method = RevokeSoftphoneCertificateSvc(inner);
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
                "/ondewo.vtsi.Softphones/GetSoftphoneProvisioning" => {
                    #[allow(non_camel_case_types)]
                    struct GetSoftphoneProvisioningSvc<T: Softphones>(pub Arc<T>);
                    impl<
                        T: Softphones,
                    > tonic::server::UnaryService<super::GetSoftphoneProvisioningRequest>
                    for GetSoftphoneProvisioningSvc<T> {
                        type Response = super::SoftphoneProvisioning;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetSoftphoneProvisioningRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Softphones>::get_softphone_provisioning(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetSoftphoneProvisioningSvc(inner);
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
    impl<T> Clone for SoftphonesServer<T> {
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
    pub const SERVICE_NAME: &str = "ondewo.vtsi.Softphones";
    impl<T> tonic::server::NamedService for SoftphonesServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
