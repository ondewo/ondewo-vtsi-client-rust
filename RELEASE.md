# Release History

*****************

## Release ONDEWO VTSI Rust Client 9.0.0

### Breaking Changes

Tracking API Version [9.0.0](https://github.com/ondewo/ondewo-vtsi-api/releases/tag/9.0.0)
(8.7.0 before), a major release: binary wire-compatible in both directions, source-breaking.

* `AsteriskConfigsFiles.sip_conf_file_string` is renamed to `pjsip_conf_file_string` (field
  number 1 and type unchanged, so the bytes on the wire are identical). Migration: rename the
  field in struct literals and field accesses.
* Eleven scalars in `ondewo/vtsi/calls.proto` gained explicit presence and are `Option<_>` now:
  `InterruptionHandlingConfig.transcribe_on_disabled_interruptions`,
  `TurnDetectionConfig.turn_detection_system_prompt` and `.turn_detection_user_prompt`,
  `AudioObjectStorageConfig.activate_audio_object_storage`,
  `AudioObjectStorageServicesActivationConfig.activate_s2t` and `.activate_t2s`,
  `MessageBrokerConfig.activate_message_broker` and
  `MessageBrokerServicesActivationConfig.activate_s2t`, `.activate_nlu`, `.activate_t2s` and
  `.activate_sip`. Migration: write `Some(value)` (or `None` for "not set"), and read with
  `.unwrap_or_default()` where the old zero-value behaviour is wanted. An explicit `Some(false)`
  now reaches the server; `None` sends nothing, as the old `false` did.
* Messages gained fields (e.g. `VtsiProject.transfer_phone_number_allowlist`, `Call`,
  `TransferCallRequest`, `AsteriskConfigsVariables`), so a struct literal that names every field
  needs the new ones or `..Default::default()`.

### New Features

* New services, each with a generated `<service>_client::<Service>Client` and
  `<service>_server::<Service>Server`:
  * `Softphones` (`ondewo/vtsi/softphones.proto`): SIP accounts for human softphone users -
    account CRUD, credential rotation, certificate list / get / revoke and provisioning.
  * `Campaigns` (`ondewo/vtsi/campaigns.proto`): campaigns of outbound calls with
    `max_parallel_calls`, lifecycle (`StartCampaign`, `StopCampaign`, `HardStopCampaign`,
    `ResumeCampaign`), statistics, campaign calls, retries and the server stream
    `StreamCampaignStatus`.
  * `Events` (`ondewo/vtsi/events.proto`): `VtsiEvent` subscriptions, webhooks (custom header
    values are write-only) and the server stream `SubscribeVtsiEvents`.
* `Calls` gained `AddCallersToCampaign`, `AddScheduledCallersToCampaign`, the status streams
  `StreamCallerStatus`, `StreamListenerStatus` and `StreamScheduledCallerStatus`, and call control:
  `InviteToCall`, `RemoveCallParticipant`, `SetCallMediaControl`, the bidirectional
  `StreamCallAudio` and the server stream `ListenCallAudio`. Batch-creating requests take an
  `idempotency_key`.
* Answering machine detection for pooled persistent callers
  (`VoiceInteractionConfig.answering_machine_detection_config`), and `Call.redial_recommended`,
  `redial_reason` and `answering_machine_detection_end_description`.
* Typed transfers (`TransferCallRequest.target`, `mode`, `headers`, `ring_timeout_s`;
  `TransferCallResponse.outcome` and friends) and `VtsiProject.transfer_phone_number_allowlist`.
* SIP trunk settings on `AsteriskConfigsVariables`: `sip_trunk_transport` (`SipTrunkTransport`,
  the zero value means TLS), `sip_trunk_source_cidr`, `sip_trunk_ca_certificates_pem`,
  `sip_trunk_verify_server` and `softphone_permit_cidrs`.
* The vendored `ondewo.sip` stubs follow ondewo-sip-api 5.5.0, as in ondewo-sip-client 5.5.0:
  answering machine detection, call identity, `SipSetCallMediaControl` and `SipStreamCallAudio`.

### Tests

* `tests/generated_grpc_new_services.rs` serves the generated `Softphones`, `Campaigns` and
  `Events` servers on one loopback socket and calls every one of their RPCs (unary and server
  streaming) through the generated clients, asserting that each reaches the handler of its name.
* `tests/generated_messages.rs` pins the `pjsip_conf_file_string` wire format (field 1), the
  presence of a scalar that gained `optional`, the `SipTrunkTransport` zero value and the
  presence of `sip_trunk_source_cidr`.

### Build

* `ondewo-proto-compiler` is pinned to 5.15.5 (5.15.4 before). The vendored nlu, s2t and t2s
  APIs are unchanged (7.1.0, 7.5.0, 6.6.0); sip-api moves from 5.4.0 to 5.5.0.

*****************

## Release ONDEWO VTSI Rust Client 8.7.1

### New Features

* TLS, mutual TLS and plaintext channels: the new hand-written module `channel` builds the tonic
  `Endpoint` / `Channel` from a `ClientConfig` (`endpoint()`, `connect()`, `connect_lazy()`).
  `grpc_cert`, `grpc_client_cert` and `grpc_client_key` take PEM **content**, not file paths.
  Without `grpc_cert` the platform trust store is used (tonic `tls-native-roots`).
* Refused with a `ChannelError` before tonic sees the config: half a client identity
  (`IncompleteClientIdentity`), a client identity on a plaintext channel
  (`ClientIdentityWithoutTls`) and a certificate field that holds no PEM certificate
  (`NotAPemCertificate`).
* A plaintext channel logs a `tracing` warning naming `host:port`. `Debug` redacts the client key
  and renders no PEM, and error messages name only the field and `host:port`. A bare IPv6 host is
  bracketed, and `tls_domain_name` overrides the name the server certificate is checked against.
* README section "TLS, mutual TLS and certificates": the modes, a test PKI built with openssl, TLS
  security notes and the channel defaults. Of the python SDK channel options, the message size
  limit is applied per client (`MAX_MESSAGE_LENGTH`); keepalive pings are not set (TCP keepalive
  detects a dropped idle connection instead), and tonic has no reconnect backoff or retry policy.

### Tests

* `tests/tls_channel.rs` runs real handshakes against an in-process tonic server with an
  openssl-generated test PKI: plain TLS, mutual TLS, missing and foreign client identities, wrong
  CA, platform roots, CRLF PEMs, SAN override and IPv6 loopback.
* `tests/release_notes.rs` pins the RELEASE.md slice the GitHub release body is cut from.

### Build

* `ondewo-proto-compiler` is pinned to 5.15.4 (5.15.2 before this release), which pre-warms tonic
  `tls-native-roots` for the offline stub generation that `src/channel.rs` needs.
  `make generate_ondewo_protos` now also stages `README.md`, which the image's `cargo package`
  requires. The regenerated stubs under `src/api` are byte-identical.
* Tracking API Version [8.7.0](https://github.com/ondewo/ondewo-vtsi-api/releases/tag/8.7.0) (unchanged)

*****************

## Release ONDEWO VTSI Rust Client 8.7.0

### New Features

* Initial release of the ONDEWO VTSI (Virtual Telephony Server Interface) gRPC client for rust. The crate
  ships the complete client surface of the [ondewo-vtsi-api](https://github.com/ondewo/ondewo-vtsi-api)
  protocol buffer definitions: [prost](https://crates.io/crates/prost) message types and
  [tonic](https://crates.io/crates/tonic) service clients for every RPC, generated by the
  `ondewo-rust-proto-compiler` image of
  [ondewo-proto-compiler](https://github.com/ondewo/ondewo-proto-compiler) 5.15.1.
* The generated module tree is confined to `src/api` and is re-exported from the crate root, so a
  message declared in `package ondewo.vtsi;` is reachable as
  `ondewo_vtsi_client::ondewo::vtsi::<Message>` and each service as
  `ondewo::vtsi::<service>_client::<Service>Client`. Hand-written modules live beside it
  under `src/` and are declared in the hand-written crate barrel `src/lib.rs`, which the
  generator leaves untouched.
* The generated stubs under `src/api` are COMMITTED, so the crate builds straight from a checkout -
  no docker, no compiler image and no submodule needed. CI compiles and tests exactly those files.
* A test suite under `tests/` exercises the generated stubs rather than only the hand-written code:
  prost messages are serialized and re-parsed field by field, maps and enum discriminants are
  pinned, and the generated `ProjectsServer` - all seven unary RPCs of `ondewo.vtsi.Projects` - is
  served over a loopback socket and driven by the generated `ProjectsClient`, so every declared RPC
  really is encoded, routed by its `/ondewo.vtsi.Projects/<Method>` path, answered and decoded
  again. Explicit proto3 presence is asserted against `ListVtsiProjectsRequest.page_token` and
  `SoftTimeoutConfig.timeout_seconds`, both on the wire and across a real gRPC hop.
* A `BearerTokenInterceptor` (`src/auth.rs`) attaches the Keycloak `authorization` and `cai-token`
  metadata to every request; its `Debug` output redacts both credentials. `make coverage` gates the
  hand-written sources at 100% line coverage and the same gate runs in CI.
* `make build` runs the whole pipeline - submodule checkout, compiler image build, stub
  generation and `cargo build` - and `make check_build` asserts that a generated stub exists for
  every proto package before a release is cut.

*****************
