#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogRequest {
    #[prost(string, tag = "1")]
    pub background: std::string::String,
    #[prost(string, tag = "2")]
    pub character: std::string::String,
    #[prost(string, tag = "3")]
    pub text: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogReply {
    #[prost(bool, tag = "1")]
    pub status: bool,
    #[prost(bytes, tag = "2")]
    pub image: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod discord_bot_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DiscordBotServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DiscordBotServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DiscordBotServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Post dialog information to Puppeteer in order to generate an image."]
        #[doc = " Internally used for Discord bots. Do not call this command in the game."]
        pub async fn post_dialog(
            &mut self,
            request: impl tonic::IntoRequest<super::DialogRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::DialogReply>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/discord_bot_service.DiscordBotService/PostDialog",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for DiscordBotServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DiscordBotServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DiscordBotServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod discord_bot_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DiscordBotServiceServer."]
    #[async_trait]
    pub trait DiscordBotService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the PostDialog method."]
        type PostDialogStream: Stream<Item = Result<super::DialogReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Post dialog information to Puppeteer in order to generate an image."]
        #[doc = " Internally used for Discord bots. Do not call this command in the game."]
        async fn post_dialog(
            &self,
            request: tonic::Request<super::DialogRequest>,
        ) -> Result<tonic::Response<Self::PostDialogStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DiscordBotServiceServer<T: DiscordBotService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DiscordBotService> DiscordBotServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DiscordBotServiceServer<T>
    where
        T: DiscordBotService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/discord_bot_service.DiscordBotService/PostDialog" => {
                    #[allow(non_camel_case_types)]
                    struct PostDialogSvc<T: DiscordBotService>(pub Arc<T>);
                    impl<T: DiscordBotService>
                        tonic::server::ServerStreamingService<super::DialogRequest>
                        for PostDialogSvc<T>
                    {
                        type Response = super::DialogReply;
                        type ResponseStream = T::PostDialogStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DialogRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).post_dialog(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = PostDialogSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DiscordBotService> Clone for DiscordBotServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DiscordBotService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DiscordBotService> tonic::transport::NamedService for DiscordBotServiceServer<T> {
        const NAME: &'static str = "discord_bot_service.DiscordBotService";
    }
}
