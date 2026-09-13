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

//! End-to-end tests for the GENERATED tonic service stubs.
//!
//! `ondewo-vtsi-api` declares three services - `Calls` (28 RPCs), `Logs` and `Projects`. The
//! generated `ProjectsServer` is the smallest all-unary one, so the fake below implements a
//! COMPLETE service rather than a slice of one. It is served over a loopback socket and driven by
//! the generated `ProjectsClient`, so a request really is encoded, routed by its
//! `/ondewo.vtsi.Projects/<Method>` path, decoded, answered and decoded again. That is what
//! catches a service the generator wired to the wrong path, a codec mismatch, or a method that
//! silently went missing.
//!
//! No network beyond `127.0.0.1` and no ONDEWO server is involved.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ondewo_vtsi_client::api::ondewo::vtsi;
use ondewo_vtsi_client::api::ondewo::vtsi::projects_client::ProjectsClient;
use ondewo_vtsi_client::api::ondewo::vtsi::projects_server::{Projects, ProjectsServer};
use ondewo_vtsi_client::auth::{
    BearerTokenInterceptor, AUTHORIZATION_METADATA_KEY, CAI_TOKEN_METADATA_KEY,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Code, Request, Response, Status};

/// The project name `get_vtsi_project` answers with `not_found` for, so the error path is
/// exercised too.
const MISSING_PROJECT: &str = "does-not-exist";

/// Metadata the fake server captured from the last request it handled.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SeenMetadata {
    authorization: Option<String>,
    cai_token: Option<String>,
}

/// What the fake server saw in the last `ListVtsiProjects` request. `page_token` is a proto3
/// `optional` field, so `None` and `Some("")` have to arrive as the two different values they are.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SeenPageToken(Option<String>);

/// A minimal in-process implementation of the generated `Projects` service.
#[derive(Clone, Default)]
struct FakeProjects {
    seen: Arc<Mutex<SeenMetadata>>,
    page_token: Arc<Mutex<SeenPageToken>>,
}

impl FakeProjects {
    fn record<T>(&self, request: &Request<T>) {
        let read = |key: &str| {
            request
                .metadata()
                .get(key)
                .map(|value| value.to_str().unwrap().to_string())
        };
        *self.seen.lock().unwrap() = SeenMetadata {
            authorization: read(AUTHORIZATION_METADATA_KEY),
            cai_token: read(CAI_TOKEN_METADATA_KEY),
        };
    }

    fn seen(&self) -> SeenMetadata {
        self.seen.lock().unwrap().clone()
    }

    fn seen_page_token(&self) -> Option<String> {
        self.page_token.lock().unwrap().0.clone()
    }
}

#[tonic::async_trait]
impl Projects for FakeProjects {
    async fn create_vtsi_project(
        &self,
        request: Request<vtsi::CreateVtsiProjectRequest>,
    ) -> Result<Response<vtsi::CreateVtsiProjectResponse>, Status> {
        self.record(&request);
        let project = request
            .into_inner()
            .vtsi_project
            .ok_or_else(|| Status::invalid_argument("vtsi_project is required"))?;
        Ok(Response::new(vtsi::CreateVtsiProjectResponse {
            vtsi_project: Some(vtsi::VtsiProject {
                vtsi_project_status: vtsi::VtsiProjectStatus::Undeployed as i32,
                ..project
            }),
            error_message: String::new(),
        }))
    }

    async fn get_vtsi_project(
        &self,
        request: Request<vtsi::GetVtsiProjectRequest>,
    ) -> Result<Response<vtsi::VtsiProject>, Status> {
        self.record(&request);
        let request = request.into_inner();
        if request.name == MISSING_PROJECT {
            return Err(Status::not_found(format!(
                "no vtsi project named {}",
                request.name
            )));
        }
        Ok(Response::new(vtsi::VtsiProject {
            name: request.name,
            max_callers: 20,
            vtsi_project_status: vtsi::VtsiProjectStatus::Deployed as i32,
            ..Default::default()
        }))
    }

    async fn update_vtsi_project(
        &self,
        request: Request<vtsi::UpdateVtsiProjectRequest>,
    ) -> Result<Response<vtsi::UpdateVtsiProjectResponse>, Status> {
        self.record(&request);
        let project = request
            .into_inner()
            .vtsi_project
            .ok_or_else(|| Status::invalid_argument("vtsi_project is required"))?;
        Ok(Response::new(vtsi::UpdateVtsiProjectResponse {
            name: project.name,
            error_message: String::new(),
        }))
    }

    async fn delete_vtsi_project(
        &self,
        request: Request<vtsi::DeleteVtsiProjectRequest>,
    ) -> Result<Response<vtsi::DeleteVtsiProjectResponse>, Status> {
        self.record(&request);
        Ok(Response::new(vtsi::DeleteVtsiProjectResponse {
            name: request.into_inner().name,
            error_message: String::new(),
        }))
    }

    async fn deploy_vtsi_project(
        &self,
        request: Request<vtsi::DeployVtsiProjectRequest>,
    ) -> Result<Response<vtsi::DeployVtsiProjectResponse>, Status> {
        self.record(&request);
        Ok(Response::new(vtsi::DeployVtsiProjectResponse {
            name: request.into_inner().name,
            error_message: String::new(),
        }))
    }

    async fn undeploy_vtsi_project(
        &self,
        request: Request<vtsi::UndeployVtsiProjectRequest>,
    ) -> Result<Response<vtsi::UndeployVtsiProjectResponse>, Status> {
        self.record(&request);
        Ok(Response::new(vtsi::UndeployVtsiProjectResponse {
            name: request.into_inner().name,
            error_message: String::new(),
        }))
    }

    /// Records the presence of `page_token` before answering, so a test can assert on what the
    /// SERVER decoded rather than on what the client encoded.
    async fn list_vtsi_projects(
        &self,
        request: Request<vtsi::ListVtsiProjectsRequest>,
    ) -> Result<Response<vtsi::ListVtsiProjectsResponse>, Status> {
        self.record(&request);
        let request = request.into_inner();
        *self.page_token.lock().unwrap() = SeenPageToken(request.page_token);
        Ok(Response::new(vtsi::ListVtsiProjectsResponse {
            vtsi_projects: vec![vtsi::VtsiProject {
                name: "projects/p".to_string(),
                display_name: "Outbound campaign".to_string(),
                active_callers: 3,
                vtsi_project_status: vtsi::VtsiProjectStatus::Deployed as i32,
                nlu_agent_names: vec!["projects/p/agent".to_string()],
                ..Default::default()
            }],
            next_page_token: "current_index-1--page_size-20".to_string(),
        }))
    }
}

/// Start the generated server on an ephemeral loopback port and return it with its address.
///
/// The server task is detached; it ends when the test process does.
async fn start_server() -> (FakeProjects, SocketAddr) {
    let service = FakeProjects::default();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    let served = service.clone();
    tokio::spawn(async move {
        Server::builder()
            .add_service(ProjectsServer::new(served))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("the in-process gRPC server must not fail");
    });

    (service, addr)
}

async fn connect(addr: SocketAddr) -> Channel {
    Endpoint::from_shared(format!("http://{addr}"))
        .expect("endpoint")
        .connect_timeout(Duration::from_secs(10))
        .connect()
        .await
        .expect("the in-process gRPC server must accept a connection")
}

#[tokio::test]
async fn a_unary_call_round_trips_through_the_generated_client_and_server() {
    let (_service, addr) = start_server().await;
    let mut client = ProjectsClient::new(connect(addr).await);

    let response = client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest {
            vtsi_project_view: vtsi::VtsiProjectView::Full as i32,
            page_token: None,
            vtsi_project_sorting: None,
            nlu_agent_names: Vec::new(),
        })
        .await
        .expect("ListVtsiProjects must succeed")
        .into_inner();

    assert_eq!(response.vtsi_projects.len(), 1);
    assert_eq!(response.vtsi_projects[0].name, "projects/p");
    assert_eq!(response.vtsi_projects[0].active_callers, 3);
    assert_eq!(
        response.vtsi_projects[0].vtsi_project_status,
        vtsi::VtsiProjectStatus::Deployed as i32
    );
    assert_eq!(response.next_page_token, "current_index-1--page_size-20");
}

/// The explicit-presence guarantee of `tests/generated_messages.rs`, but over a real hop: an unset
/// `page_token` and one explicitly set to `""` have to reach the SERVER as the two different
/// values they are.
#[tokio::test]
async fn an_explicit_presence_field_survives_a_real_grpc_hop() {
    let (service, addr) = start_server().await;
    let mut client = ProjectsClient::new(connect(addr).await);

    client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest {
            page_token: None,
            ..Default::default()
        })
        .await
        .expect("ListVtsiProjects must succeed");
    assert_eq!(
        service.seen_page_token(),
        None,
        "an unset presence field must not appear on the wire"
    );

    client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest {
            page_token: Some(String::new()),
            ..Default::default()
        })
        .await
        .expect("ListVtsiProjects must succeed");
    assert_eq!(
        service.seen_page_token(),
        Some(String::new()),
        "an explicitly empty presence field must not arrive unset"
    );
}

/// A server-side `Status` has to reach the caller as that same status, not as a transport error.
#[tokio::test]
async fn a_server_error_reaches_the_client_as_its_status() {
    let (_service, addr) = start_server().await;
    let mut client = ProjectsClient::new(connect(addr).await);

    let error = client
        .get_vtsi_project(vtsi::GetVtsiProjectRequest {
            name: MISSING_PROJECT.to_string(),
            vtsi_project_view: vtsi::VtsiProjectView::Full as i32,
        })
        .await
        .expect_err("GetVtsiProject must report the missing project");

    assert_eq!(error.code(), Code::NotFound);
    assert_eq!(error.message(), "no vtsi project named does-not-exist");
}

/// Every RPC the `Projects` proto declares must exist on the generated client and be routable -
/// a method the generator dropped, or wired to the wrong path, fails here with `Unimplemented`.
#[tokio::test]
async fn every_declared_service_method_exists_and_is_routable() {
    let (_service, addr) = start_server().await;
    let mut client = ProjectsClient::new(connect(addr).await);

    let project = vtsi::VtsiProject {
        name: "projects/p".to_string(),
        max_callers: 20,
        ..Default::default()
    };
    let created = client
        .create_vtsi_project(vtsi::CreateVtsiProjectRequest {
            vtsi_project: Some(project.clone()),
            error_message: String::new(),
        })
        .await
        .expect("CreateVtsiProject")
        .into_inner();
    assert_eq!(
        created.vtsi_project.unwrap().vtsi_project_status,
        vtsi::VtsiProjectStatus::Undeployed as i32
    );

    client
        .get_vtsi_project(vtsi::GetVtsiProjectRequest {
            name: "projects/p".to_string(),
            vtsi_project_view: vtsi::VtsiProjectView::Minimum as i32,
        })
        .await
        .expect("GetVtsiProject");
    client
        .update_vtsi_project(vtsi::UpdateVtsiProjectRequest {
            vtsi_project: Some(project),
            update_mask: None,
        })
        .await
        .expect("UpdateVtsiProject");
    client
        .deploy_vtsi_project(vtsi::DeployVtsiProjectRequest {
            name: "projects/p".to_string(),
        })
        .await
        .expect("DeployVtsiProject");
    client
        .undeploy_vtsi_project(vtsi::UndeployVtsiProjectRequest {
            name: "projects/p".to_string(),
        })
        .await
        .expect("UndeployVtsiProject");
    client
        .delete_vtsi_project(vtsi::DeleteVtsiProjectRequest {
            name: "projects/p".to_string(),
        })
        .await
        .expect("DeleteVtsiProject");
    client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest::default())
        .await
        .expect("ListVtsiProjects");
}

/// The hand-written [`BearerTokenInterceptor`] has to put its metadata on the wire, where the
/// server can actually read it - asserting on the `Request` it returns would not prove that.
#[tokio::test]
async fn the_bearer_interceptor_reaches_the_server() {
    let (service, addr) = start_server().await;
    let interceptor = BearerTokenInterceptor::new("access-token-abc")
        .expect("a plain ASCII token is valid")
        .with_cai_token("cai-token-xyz")
        .expect("a plain ASCII cai token is valid");
    let mut client = ProjectsClient::with_interceptor(connect(addr).await, interceptor);

    client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest::default())
        .await
        .expect("ListVtsiProjects");

    assert_eq!(
        service.seen(),
        SeenMetadata {
            authorization: Some("Bearer access-token-abc".to_string()),
            cai_token: Some("cai-token-xyz".to_string()),
        }
    );
}

/// Without the interceptor the client must send no credentials at all - the unauthenticated path
/// (plaintext server, or an ingress that injects the bearer token) has to stay usable.
#[tokio::test]
async fn a_client_without_an_interceptor_sends_no_credentials() {
    let (service, addr) = start_server().await;
    let mut client = ProjectsClient::new(connect(addr).await);

    client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest::default())
        .await
        .expect("ListVtsiProjects");

    assert_eq!(service.seen(), SeenMetadata::default());
}

/// A client built against an address nothing listens on must surface a transport error rather
/// than panic or hang - `connect_lazy` defers the connect to the first call.
#[tokio::test]
async fn a_call_to_an_unreachable_target_fails_as_a_status() {
    let channel = Endpoint::from_static("http://127.0.0.1:1")
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy();
    let mut client = ProjectsClient::new(channel);

    let error = client
        .get_vtsi_project(vtsi::GetVtsiProjectRequest {
            name: "projects/p".to_string(),
            vtsi_project_view: vtsi::VtsiProjectView::Full as i32,
        })
        .await
        .expect_err("nothing listens on port 1");

    assert_eq!(error.code(), Code::Unavailable);
}
