#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterData {
    #[prost(string, tag = "1")]
    pub user_name: std::string::String,
    #[prost(string, tag = "2")]
    pub nickname: std::string::String,
    #[prost(string, tag = "3")]
    pub email: std::string::String,
    #[prost(string, tag = "4")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterReply {
    #[prost(bool, tag = "1")]
    pub status: bool,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginData {
    #[prost(string, tag = "1")]
    pub account: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginReply {
    #[prost(bool, tag = "1")]
    pub status: bool,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
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
pub mod game_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct GameClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameClient<tonic::transport::Channel> {
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
    impl<T> GameClient<T>
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
        pub async fn register(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterData>,
        ) -> Result<tonic::Response<super::RegisterReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/game.Game/Register");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginData>,
        ) -> Result<tonic::Response<super::LoginReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/game.Game/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
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
            let path = http::uri::PathAndQuery::from_static("/game.Game/PostDialog");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for GameClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GameClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GameClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod game_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GameServer."]
    #[async_trait]
    pub trait Game: Send + Sync + 'static {
        async fn register(
            &self,
            request: tonic::Request<super::RegisterData>,
        ) -> Result<tonic::Response<super::RegisterReply>, tonic::Status>;
        async fn login(
            &self,
            request: tonic::Request<super::LoginData>,
        ) -> Result<tonic::Response<super::LoginReply>, tonic::Status>;
        #[doc = "Server streaming response type for the PostDialog method."]
        type PostDialogStream: Stream<Item = Result<super::DialogReply, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn post_dialog(
            &self,
            request: tonic::Request<super::DialogRequest>,
        ) -> Result<tonic::Response<Self::PostDialogStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GameServer<T: Game> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Game> GameServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GameServer<T>
    where
        T: Game,
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
                "/game.Game/Register" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterSvc<T: Game>(pub Arc<T>);
                    impl<T: Game> tonic::server::UnaryService<super::RegisterData> for RegisterSvc<T> {
                        type Response = super::RegisterReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterData>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RegisterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/game.Game/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: Game>(pub Arc<T>);
                    impl<T: Game> tonic::server::UnaryService<super::LoginData> for LoginSvc<T> {
                        type Response = super::LoginReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginData>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/game.Game/PostDialog" => {
                    #[allow(non_camel_case_types)]
                    struct PostDialogSvc<T: Game>(pub Arc<T>);
                    impl<T: Game> tonic::server::ServerStreamingService<super::DialogRequest> for PostDialogSvc<T> {
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
    impl<T: Game> Clone for GameServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Game> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Game> tonic::transport::NamedService for GameServer<T> {
        const NAME: &'static str = "game.Game";
    }
}
