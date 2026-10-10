// Copyright 2021-2026 ONDEWO GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! End-to-end tests for the three services `ondewo-vtsi-api` 9.0.0 added: `Softphones`,
//! `Campaigns` and `Events`.
//!
//! Each generated server is served over one loopback socket by a fake that answers every RPC with
//! the default response and records which handler it ran. Every RPC is then called once through the
//! generated client, so each method really is encoded, routed by its
//! `/ondewo.vtsi.<Service>/<Method>` path, decoded, answered and decoded again - and the recorded
//! handler names prove that every client method reaches the server handler of the same name.
//! `tests/generated_grpc.rs` covers the behaviour of a single service in depth; this suite is
//! about the wiring of the new ones.
//!
//! No network beyond `127.0.0.1` and no ONDEWO server is involved.

use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ondewo_vtsi_client::api::ondewo::vtsi;
use ondewo_vtsi_client::api::ondewo::vtsi::campaigns_client::CampaignsClient;
use ondewo_vtsi_client::api::ondewo::vtsi::campaigns_server::{Campaigns, CampaignsServer};
use ondewo_vtsi_client::api::ondewo::vtsi::events_client::EventsClient;
use ondewo_vtsi_client::api::ondewo::vtsi::events_server::{Events, EventsServer};
use ondewo_vtsi_client::api::ondewo::vtsi::softphones_client::SoftphonesClient;
use ondewo_vtsi_client::api::ondewo::vtsi::softphones_server::{Softphones, SoftphonesServer};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Request, Response, Status};

/// The server half of a server-streaming RPC, as the generated traits ask for it.
type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

/// One fake behind all three servers: it records the name of every handler it runs.
#[derive(Clone, Default)]
struct Recorder {
    handled: Arc<Mutex<Vec<&'static str>>>,
}

impl Recorder {
    fn record(&self, handler: &'static str) {
        self.handled.lock().unwrap().push(handler);
    }

    /// The handlers run so far, and forget them.
    fn take(&self) -> Vec<&'static str> {
        std::mem::take(&mut *self.handled.lock().unwrap())
    }
}

/// Implement a generated server trait on [`Recorder`] - every unary RPC answers its default
/// response, every server-streaming RPC exactly one default message - and generate the function
/// that calls each of its RPCs once through the generated client, in declaration order.
macro_rules! fake_service {
    (
        $service:ident, $client:ident, $call_every_rpc:ident,
        unary { $($method:ident: $request:ident => $response:ident,)* }
        server_streaming { $($stream_method:ident / $stream:ident: $stream_request:ident => $stream_response:ident,)* }
    ) => {
        #[tonic::async_trait]
        impl $service for Recorder {
            $(
                async fn $method(
                    &self,
                    _request: Request<vtsi::$request>,
                ) -> Result<Response<vtsi::$response>, Status> {
                    self.record(stringify!($method));
                    Ok(Response::new(vtsi::$response::default()))
                }
            )*
            $(
                type $stream = ResponseStream<vtsi::$stream_response>;

                async fn $stream_method(
                    &self,
                    _request: Request<vtsi::$stream_request>,
                ) -> Result<Response<Self::$stream>, Status> {
                    self.record(stringify!($stream_method));
                    Ok(Response::new(Box::pin(tokio_stream::iter(vec![Ok(
                        vtsi::$stream_response::default(),
                    )]))))
                }
            )*
        }

        async fn $call_every_rpc(channel: Channel) -> Vec<&'static str> {
            let mut client = $client::new(channel);
            let mut called = Vec::new();
            $(
                client
                    .$method(vtsi::$request::default())
                    .await
                    .unwrap_or_else(|status| panic!("{}: {status}", stringify!($method)));
                called.push(stringify!($method));
            )*
            $(
                let messages: Vec<_> = client
                    .$stream_method(vtsi::$stream_request::default())
                    .await
                    .unwrap_or_else(|status| panic!("{}: {status}", stringify!($stream_method)))
                    .into_inner()
                    .collect()
                    .await;
                assert_eq!(messages.len(), 1, "{}", stringify!($stream_method));
                assert!(messages[0].is_ok(), "{}", stringify!($stream_method));
                called.push(stringify!($stream_method));
            )*
            called
        }
    };
}

fake_service! {
    Softphones, SoftphonesClient, call_every_softphones_rpc,
    unary {
        create_softphone_account: CreateSoftphoneAccountRequest => CreateSoftphoneAccountResponse,
        get_softphone_account: GetSoftphoneAccountRequest => SoftphoneAccount,
        update_softphone_account: UpdateSoftphoneAccountRequest => SoftphoneAccount,
        delete_softphone_account: DeleteSoftphoneAccountRequest => DeleteSoftphoneAccountResponse,
        list_softphone_accounts: ListSoftphoneAccountsRequest => ListSoftphoneAccountsResponse,
        rotate_softphone_credentials: RotateSoftphoneCredentialsRequest => RotateSoftphoneCredentialsResponse,
        list_softphone_certificates: ListSoftphoneCertificatesRequest => ListSoftphoneCertificatesResponse,
        get_softphone_certificate: GetSoftphoneCertificateRequest => SoftphoneCertificate,
        revoke_softphone_certificate: RevokeSoftphoneCertificateRequest => SoftphoneCertificate,
        get_softphone_provisioning: GetSoftphoneProvisioningRequest => SoftphoneProvisioning,
    }
    server_streaming {}
}

fake_service! {
    Campaigns, CampaignsClient, call_every_campaigns_rpc,
    unary {
        create_campaign: CreateCampaignRequest => Campaign,
        get_campaign: GetCampaignRequest => Campaign,
        update_campaign: UpdateCampaignRequest => Campaign,
        delete_campaign: DeleteCampaignRequest => DeleteCampaignResponse,
        list_campaigns: ListCampaignsRequest => ListCampaignsResponse,
        get_campaign_statistics: GetCampaignStatisticsRequest => CampaignStatistics,
        list_campaign_calls: ListCampaignCallsRequest => ListCampaignCallsResponse,
        start_campaign: StartCampaignRequest => Campaign,
        stop_campaign: StopCampaignRequest => Campaign,
        hard_stop_campaign: HardStopCampaignRequest => Campaign,
        resume_campaign: ResumeCampaignRequest => Campaign,
    }
    server_streaming {
        stream_campaign_status / StreamCampaignStatusStream:
            StreamCampaignStatusRequest => StreamCampaignStatusResponse,
    }
}

fake_service! {
    Events, EventsClient, call_every_events_rpc,
    unary {
        create_vtsi_event_subscription: CreateVtsiEventSubscriptionRequest => VtsiEventSubscription,
        get_vtsi_event_subscription: GetVtsiEventSubscriptionRequest => VtsiEventSubscription,
        update_vtsi_event_subscription: UpdateVtsiEventSubscriptionRequest => VtsiEventSubscription,
        delete_vtsi_event_subscription: DeleteVtsiEventSubscriptionRequest => DeleteVtsiEventSubscriptionResponse,
        list_vtsi_event_subscriptions: ListVtsiEventSubscriptionsRequest => ListVtsiEventSubscriptionsResponse,
        create_webhook: CreateWebhookRequest => Webhook,
        get_webhook: GetWebhookRequest => Webhook,
        update_webhook: UpdateWebhookRequest => Webhook,
        delete_webhook: DeleteWebhookRequest => DeleteWebhookResponse,
        list_webhooks: ListWebhooksRequest => ListWebhooksResponse,
        test_webhook: TestWebhookRequest => TestWebhookResponse,
    }
    server_streaming {
        subscribe_vtsi_events / SubscribeVtsiEventsStream:
            SubscribeVtsiEventsRequest => SubscribeVtsiEventsResponse,
    }
}

/// Serve all three generated servers on one ephemeral loopback port and connect to it.
async fn serve_and_connect() -> (Recorder, Channel) {
    let recorder = Recorder::default();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr: SocketAddr = listener.local_addr().expect("local_addr");

    let served = recorder.clone();
    tokio::spawn(async move {
        Server::builder()
            .add_service(SoftphonesServer::new(served.clone()))
            .add_service(CampaignsServer::new(served.clone()))
            .add_service(EventsServer::new(served))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("the in-process gRPC server must not fail");
    });

    let channel = Endpoint::from_shared(format!("http://{addr}"))
        .expect("endpoint")
        .connect_timeout(Duration::from_secs(10))
        .connect()
        .await
        .expect("the in-process gRPC server must accept a connection");
    (recorder, channel)
}

#[tokio::test]
async fn every_softphones_rpc_reaches_its_own_handler() {
    let (recorder, channel) = serve_and_connect().await;

    let called = call_every_softphones_rpc(channel).await;

    assert_eq!(called.len(), 10, "Softphones declares ten RPCs");
    assert_eq!(recorder.take(), called);
}

#[tokio::test]
async fn every_campaigns_rpc_reaches_its_own_handler() {
    let (recorder, channel) = serve_and_connect().await;

    let called = call_every_campaigns_rpc(channel).await;

    assert_eq!(called.len(), 12, "Campaigns declares twelve RPCs");
    assert_eq!(recorder.take(), called);
}

#[tokio::test]
async fn every_events_rpc_reaches_its_own_handler() {
    let (recorder, channel) = serve_and_connect().await;

    let called = call_every_events_rpc(channel).await;

    assert_eq!(called.len(), 12, "Events declares twelve RPCs");
    assert_eq!(recorder.take(), called);
}

/// The three services share one channel, as a client of a VTSI server would: a request for one of
/// them must never be routed to another.
#[tokio::test]
async fn the_new_services_share_one_channel_without_crossing_routes() {
    let (recorder, channel) = serve_and_connect().await;

    let mut expected = call_every_softphones_rpc(channel.clone()).await;
    expected.extend(call_every_campaigns_rpc(channel.clone()).await);
    expected.extend(call_every_events_rpc(channel).await);

    assert_eq!(recorder.take(), expected);
}
