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

//! Call an ONDEWO VTSI server with an auto-refreshed Keycloak token.
//!
//! With `ONDEWO_VTSI_OFFLINE_TOKEN` set, the client starts up from that offline refresh token and
//! never sends a password; otherwise it logs in with `ONDEWO_VTSI_PASSWORD`.
//!
//! ```sh
//! ONDEWO_VTSI_HOST=vtsi.example.com ONDEWO_VTSI_PORT=443 \
//! ONDEWO_KEYCLOAK_URL=https://keycloak.example.com/auth \
//! ONDEWO_KEYCLOAK_REALM=ondewo-ccai-platform \
//! ONDEWO_KEYCLOAK_CLIENT_ID=ondewo-vtsi-cai-sdk-public \
//! ONDEWO_VTSI_USERNAME=technical-user@example.com \
//! ONDEWO_VTSI_OFFLINE_TOKEN=<offline refresh token> \
//!   cargo run --example keycloak_client
//! ```

use std::env;
use std::error::Error;

use ondewo_vtsi_client::api::ondewo::vtsi;
use ondewo_vtsi_client::api::ondewo::vtsi::projects_client::ProjectsClient;
use ondewo_vtsi_client::channel::ClientConfig;
use ondewo_vtsi_client::keycloak::{KeycloakConfig, KeycloakInterceptor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = KeycloakConfig::new(
        env::var("ONDEWO_KEYCLOAK_URL")?,
        env::var("ONDEWO_KEYCLOAK_REALM")?,
        env::var("ONDEWO_KEYCLOAK_CLIENT_ID")?,
        env::var("ONDEWO_VTSI_USERNAME").unwrap_or_default(),
        env::var("ONDEWO_VTSI_PASSWORD").unwrap_or_default(),
    )
    .with_refresh_token(env::var("ONDEWO_VTSI_OFFLINE_TOKEN").unwrap_or_default());
    // Validates the config, then acquires the first token; the provider keeps it fresh.
    let interceptor = KeycloakInterceptor::new(&config)?;

    let channel = ClientConfig::new(
        env::var("ONDEWO_VTSI_HOST")?,
        env::var("ONDEWO_VTSI_PORT")?.parse()?,
    )
    .connect()
    .await?;
    let mut client = ProjectsClient::with_interceptor(channel, interceptor);

    let response = client
        .list_vtsi_projects(vtsi::ListVtsiProjectsRequest {
            vtsi_project_view: vtsi::VtsiProjectView::Full as i32,
            // `page_token` is a proto3 `optional` field: `None` asks for the first page, while
            // `Some(String::new())` would be an explicitly empty token.
            page_token: None,
            vtsi_project_sorting: None,
            nlu_agent_names: Vec::new(),
            ..Default::default()
        })
        .await?
        .into_inner();

    for project in response.vtsi_projects {
        println!(
            "{} ({} callers, {} listeners)",
            project.name, project.active_callers, project.active_listeners
        );
    }
    Ok(())
}
