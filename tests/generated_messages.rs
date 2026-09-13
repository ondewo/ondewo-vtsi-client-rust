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

//! Wire-level tests for the GENERATED prost messages under `src/api`.
//!
//! These are the cases that catch a broken generator: a dropped field, a shifted tag number, a
//! presence field silently coerced to its zero value, an enum whose discriminants moved. They are
//! pure encode/decode - no runtime, no socket. The gRPC plumbing is covered by
//! `tests/generated_grpc.rs`.
//!
//! VTSI orchestrates the other ONDEWO services, so the crate vendors five further `ondewo.*`
//! packages beside its own; the cases below reach across those package boundaries on purpose.

use std::collections::HashMap;

use ondewo_vtsi_client::api::google;
use ondewo_vtsi_client::api::ondewo::{nlu, qa, s2t, sip, t2s, vtsi};
use prost::Message;
use prost_types::Timestamp;

/// A fully populated [`vtsi::VtsiProject`] - scalars, a nested message that itself carries a
/// presence field and a oneof, a repeated scalar, an enum and two well-known timestamps at once.
fn sample_project() -> vtsi::VtsiProject {
    vtsi::VtsiProject {
        name: "projects/p".to_string(),
        display_name: "Outbound campaign".to_string(),
        max_callers: 20,
        max_listeners: 5,
        asterisk_configs: Some(vtsi::AsteriskConfigs {
            asterisk_port: 5060,
            asterisk_version: Some("20.4.0".to_string()),
            asterisk_configs_oneof: Some(
                vtsi::asterisk_configs::AsteriskConfigsOneof::AsteriskConfigsTargetDirectoryName(
                    "/etc/asterisk".to_string(),
                ),
            ),
        }),
        vtsi_project_status: vtsi::VtsiProjectStatus::Deployed as i32,
        created_by: "office@ondewo.com".to_string(),
        created_at: Some(Timestamp {
            seconds: 1_700_000_000,
            nanos: 123,
        }),
        modified_by: "office@ondewo.com".to_string(),
        modified_at: Some(Timestamp {
            seconds: 1_700_000_001,
            nanos: 0,
        }),
        active_callers: 3,
        active_listeners: 1,
        asterisk_port: 5060,
        nlu_agent_names: vec![
            "projects/p/agent".to_string(),
            "projects/q/agent".to_string(),
        ],
        deployed_callers: 3,
        deployed_listeners: 1,
    }
}

#[test]
fn a_project_survives_a_serialize_parse_round_trip() {
    let original = sample_project();

    let bytes = original.encode_to_vec();
    assert!(
        !bytes.is_empty(),
        "a populated VtsiProject must not encode to zero bytes"
    );
    assert_eq!(
        bytes.len(),
        original.encoded_len(),
        "encoded_len must agree with the bytes actually written"
    );

    let parsed =
        vtsi::VtsiProject::decode(bytes.as_slice()).expect("re-parsing our own bytes must work");
    assert_eq!(parsed, original);

    // Spot-check the individual fields too: a PartialEq on two identically broken values would
    // still pass above.
    assert_eq!(parsed.name, "projects/p");
    assert_eq!(parsed.max_callers, 20);
    assert_eq!(parsed.nlu_agent_names.len(), 2);
    assert_eq!(parsed.nlu_agent_names[1], "projects/q/agent");
    assert_eq!(parsed.created_at.unwrap().nanos, 123);
    let configs = parsed.asterisk_configs.unwrap();
    assert_eq!(configs.asterisk_version.as_deref(), Some("20.4.0"));
    assert_eq!(
        configs.asterisk_configs_oneof,
        Some(
            vtsi::asterisk_configs::AsteriskConfigsOneof::AsteriskConfigsTargetDirectoryName(
                "/etc/asterisk".to_string()
            )
        )
    );
}

#[test]
fn a_default_project_round_trips_to_zero_bytes() {
    let empty = vtsi::VtsiProject::default();

    assert_eq!(empty.max_callers, 0);
    assert_eq!(empty.asterisk_configs, None);
    assert!(empty.nlu_agent_names.is_empty());

    let bytes = empty.encode_to_vec();
    assert!(
        bytes.is_empty(),
        "proto3 must not put unset fields on the wire, got {bytes:?}"
    );
    assert_eq!(vtsi::VtsiProject::decode(bytes.as_slice()).unwrap(), empty);
}

/// `ListVtsiProjectsRequest.page_token` is a proto3 `optional` (explicit presence) field. An unset
/// field and a field explicitly set to the empty string are two DIFFERENT values and must stay
/// distinguishable across the wire - a generator that collapses them makes `""` unsendable.
#[test]
fn an_explicit_presence_field_distinguishes_unset_from_zero() {
    let unset = vtsi::ListVtsiProjectsRequest {
        vtsi_project_view: vtsi::VtsiProjectView::Full as i32,
        page_token: None,
        ..Default::default()
    };
    let explicit_empty = vtsi::ListVtsiProjectsRequest {
        page_token: Some(String::new()),
        ..unset.clone()
    };

    let unset_bytes = unset.encode_to_vec();
    let empty_bytes = explicit_empty.encode_to_vec();
    assert_ne!(
        unset_bytes, empty_bytes,
        "an explicitly set \"\" must occupy the wire, an unset field must not"
    );

    assert_eq!(
        vtsi::ListVtsiProjectsRequest::decode(unset_bytes.as_slice())
            .unwrap()
            .page_token,
        None
    );
    assert_eq!(
        vtsi::ListVtsiProjectsRequest::decode(empty_bytes.as_slice())
            .unwrap()
            .page_token,
        Some(String::new())
    );

    // The same for a presence FLOAT, where the zero value is 0.0.
    let unset = vtsi::SoftTimeoutConfig {
        timeout_seconds: None,
        ..Default::default()
    };
    let explicit_zero = vtsi::SoftTimeoutConfig {
        timeout_seconds: Some(0.0),
        ..Default::default()
    };
    assert_ne!(unset.encode_to_vec(), explicit_zero.encode_to_vec());
    assert_eq!(
        vtsi::SoftTimeoutConfig::decode(explicit_zero.encode_to_vec().as_slice())
            .unwrap()
            .timeout_seconds,
        Some(0.0),
        "an explicitly zeroed presence field must not come back unset"
    );
}

/// Decoding tolerates fields it does not know: an unknown tag is skipped, not an error.
#[test]
fn decoding_skips_an_unknown_field() {
    let mut bytes = vtsi::DeleteVtsiProjectRequest {
        name: "projects/p".to_string(),
    }
    .encode_to_vec();
    // tag 999, wire type 0 (varint), value 1
    bytes.extend_from_slice(&[0xB8, 0x3E, 0x01]);

    let parsed = vtsi::DeleteVtsiProjectRequest::decode(bytes.as_slice())
        .expect("an unknown field must be skipped, not rejected");
    assert_eq!(parsed.name, "projects/p");
}

#[test]
fn decoding_rejects_a_truncated_message() {
    let bytes = sample_project().encode_to_vec();
    let truncated = &bytes[..bytes.len() - 1];

    assert!(
        vtsi::VtsiProject::decode(truncated).is_err(),
        "a truncated message must not decode silently"
    );
}

/// The zero value of an enum is the one a default-constructed message carries, so it must be the
/// variant the proto declares as `= 0`.
#[test]
fn the_enum_zero_value_is_the_unspecified_variant() {
    assert_eq!(vtsi::VtsiProjectView::Unspecified as i32, 0);
    assert_eq!(
        vtsi::VtsiProjectView::try_from(0),
        Ok(vtsi::VtsiProjectView::Unspecified)
    );
    assert_eq!(
        vtsi::ListVtsiProjectsRequest::default().vtsi_project_view,
        vtsi::VtsiProjectView::Unspecified as i32,
        "a default message must carry the enum's zero value"
    );

    assert_eq!(
        vtsi::VtsiProjectView::Unspecified.as_str_name(),
        "VTSI_PROJECT_VIEW_UNSPECIFIED"
    );
    assert_eq!(
        vtsi::VtsiProjectView::from_str_name("VTSI_PROJECT_VIEW_UNSPECIFIED"),
        Some(vtsi::VtsiProjectView::Unspecified)
    );
    assert_eq!(vtsi::VtsiProjectView::from_str_name("NOT_A_VARIANT"), None);
    assert!(
        vtsi::VtsiProjectView::try_from(9_999).is_err(),
        "an out-of-range discriminant must not map to a variant"
    );

    // `VtsiProjectSortingMode` is the other shape this API uses: its zero variant is ASCENDING,
    // not an UNSPECIFIED, so a default has to be that and not a sentinel.
    assert_eq!(vtsi::VtsiProjectSortingMode::Ascending as i32, 0);
    assert_eq!(
        vtsi::VtsiProjectSortingMode::Ascending.as_str_name(),
        "ASCENDING"
    );
}

/// A non-zero enum value has to travel as its discriminant, not as the zero value.
#[test]
fn a_non_zero_enum_value_round_trips() {
    let request = vtsi::GetVtsiProjectRequest {
        name: "projects/p".to_string(),
        vtsi_project_view: vtsi::VtsiProjectView::Minimum as i32,
    };

    let parsed = vtsi::GetVtsiProjectRequest::decode(request.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, request);
    assert_eq!(
        vtsi::VtsiProjectView::try_from(parsed.vtsi_project_view),
        Ok(vtsi::VtsiProjectView::Minimum)
    );
}

/// Repeated and nested message fields have to nest, not flatten.
#[test]
fn a_nested_and_repeated_message_round_trips() {
    let response = vtsi::ListVtsiProjectsResponse {
        vtsi_projects: vec![
            sample_project(),
            vtsi::VtsiProject {
                name: "second".to_string(),
                ..Default::default()
            },
        ],
        next_page_token: "current_index-1--page_size-20".to_string(),
    };

    let parsed =
        vtsi::ListVtsiProjectsResponse::decode(response.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, response);
    assert_eq!(parsed.vtsi_projects.len(), 2);
    assert_eq!(parsed.vtsi_projects[1].name, "second");
}

/// A map field has to survive as a map - entry order on the wire is unspecified, the content is
/// what must match.
#[test]
fn a_map_field_round_trips() {
    let mut sip_headers = HashMap::new();
    sip_headers.insert("X-Campaign".to_string(), "spring".to_string());
    sip_headers.insert("X-Caller".to_string(), "ondewo".to_string());

    let config = vtsi::SipCallerConfig {
        sip_base_config: Some(vtsi::SipBaseConfig {
            sip_sim_version: "1.2.3".to_string(),
        }),
        callee_id: "+43123456789".to_string(),
        sip_headers,
    };

    let parsed = vtsi::SipCallerConfig::decode(config.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, config);
    assert_eq!(parsed.sip_headers.len(), 2);
    assert_eq!(parsed.sip_headers["X-Campaign"], "spring");
    assert_eq!(
        parsed.sip_base_config.unwrap().sip_sim_version,
        "1.2.3",
        "a nested message beside a map must not be swallowed by it"
    );
}

/// The generator emits one module per proto PACKAGE. VTSI vendors eight packages besides its own -
/// `ondewo.{nlu,qa,s2t,sip,t2s}` and `google.{api,rpc,type}` - and messages of every one of them
/// have to be reachable and usable, including a message that references another package's type.
#[test]
fn messages_of_every_generated_package_are_reachable() {
    // ondewo.qa -> ondewo.nlu: a cross-package field reference.
    let request = qa::GetAnswerRequest {
        session_id: "projects/p/agent/sessions/s".to_string(),
        text: Some(nlu::TextInput {
            text: "where is the exit?".to_string(),
            language_code: "en".to_string(),
        }),
        max_num_answers: 3,
        threshold_overall: 0.75,
        ..Default::default()
    };
    let parsed = qa::GetAnswerRequest::decode(request.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, request);
    assert_eq!(parsed.text.unwrap().text, "where is the exit?");

    // ondewo.sip
    let mut headers = HashMap::new();
    headers.insert("X-Campaign".to_string(), "spring".to_string());
    let start_call = sip::SipStartCallRequest {
        callee_id: "+43123456789".to_string(),
        headers,
    };
    assert_eq!(
        sip::SipStartCallRequest::decode(start_call.encode_to_vec().as_slice()).unwrap(),
        start_call
    );

    // ondewo.s2t
    let s2t_config = s2t::TranscribeRequestConfig {
        s2t_pipeline_id: "s2t-pipeline-1".to_string(),
        language: Some("de".to_string()),
        ..Default::default()
    };
    assert_eq!(
        s2t::TranscribeRequestConfig::decode(s2t_config.encode_to_vec().as_slice()).unwrap(),
        s2t_config
    );

    // ondewo.t2s
    let normalize = t2s::NormalizeTextRequest {
        t2s_pipeline_id: "t2s-pipeline-1".to_string(),
        text: "10 EUR".to_string(),
    };
    assert_eq!(
        t2s::NormalizeTextRequest::decode(normalize.encode_to_vec().as_slice()).unwrap(),
        normalize
    );

    // google.rpc
    let status = google::rpc::Status {
        code: 5,
        message: "not found".to_string(),
        details: vec![],
    };
    assert_eq!(
        google::rpc::Status::decode(status.encode_to_vec().as_slice()).unwrap(),
        status
    );

    // google.type
    let coordinates = google::r#type::LatLng {
        latitude: 48.208_2,
        longitude: 16.373_8,
    };
    assert_eq!(
        google::r#type::LatLng::decode(coordinates.encode_to_vec().as_slice()).unwrap(),
        coordinates
    );

    // google.api
    let pattern = google::api::CustomHttpPattern {
        kind: "GET".to_string(),
        path: "/v1/vtsi_projects".to_string(),
    };
    assert_eq!(
        google::api::CustomHttpPattern::decode(pattern.encode_to_vec().as_slice()).unwrap(),
        pattern
    );
}
