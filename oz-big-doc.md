# Oz Orchestration Final Status
Date: 2026-04-30  
Workspace context: `~/reverse_warp_orchestration`  
Primary repo-local doc location: `~/reverse_warp_orchestration/warp/research/oz`
## Purpose and safety scope
This document consolidates the current state of the local Warp “Oz” orchestration reverse-engineering/documentation work.
The work so far has been limited to:
- local source analysis of the open-source Warp repo
- local test harnesses
- local patches
- local mock server design and implementation
- local compile/check/build attempts
- local runtime-preparation work
The intended next phase is a macOS GUI build/runtime test of the open-source Warp OSS binary against a local mock server.
Safety boundaries remain:
- Do not contact production Warp services.
- Do not send requests to `app.warp.dev`, `rtc.app.warp.dev`, `sessions.app.warp.dev`, or other Warp-operated services.
- Do not attempt auth bypass, token replay, credential harvesting, traffic replay against production, stress testing, or attacks.
- Keep protocol work local-source-based unless explicitly changed later.
- Treat credentials, auth headers, cookies, tokens, and `runtime_skills` payloads as sensitive.
- Mock server logging should redact sensitive fields and avoid raw body capture unless deliberately reviewed and sanitized.
## Directory map
Top-level workspace:
```text
~/reverse_warp_orchestration

Main Warp repo:

~/reverse_warp_orchestration/warp

Repo-local research docs:

~/reverse_warp_orchestration/warp/research/oz

Side research docs:

~/reverse_warp_orchestration/research/oz

Local mock server:

~/reverse_warp_orchestration/oz-mock-server

Run logs:

~/reverse_warp_orchestration/run_logs

There are two research directories. This matters:

~/reverse_warp_orchestration/research/oz
~/reverse_warp_orchestration/warp/research/oz

Most durable/final docs should live in the repo-local directory:

~/reverse_warp_orchestration/warp/research/oz

High-level conclusion

The best current model is that Oz-related orchestration is bifurcated:

1. Cloud control plane path
    This appears to cover managed/ambient agent orchestration through REST and SSE-style event streaming.
    Current evidence supports REST calls and an agent event stream path. The agent realtime path appears to use SSE via stream_agent_events, not GraphQL WebSocket.
2. Remote Oz CLI/server path
    This is a separate local/remote-server protocol path using length-prefixed protobuf over stdio.
    This side is much more solidly verified than the cloud-control-plane side because it has a passing local Rust harness.

Do not collapse these into one protocol. They appear to be distinct paths.

Important source paths

Important source files identified so far:

app/src/server/server_api.rs
app/src/server/server_api/ai.rs
app/src/auth/credentials.rs
app/src/lib.rs
app/src/mock_override.rs
app/src/ai/agent_sdk/ambient.rs
app/src/ai/agent/api.rs
app/src/ai/skills/skill_manager.rs
app/src/pane_group/pane/terminal_pane.rs
crates/warp_core/src/channel/config.rs
crates/warp_core/src/channel/mod.rs
crates/warp_core/src/channel/state.rs
crates/warp_cli/src/lib.rs
crates/remote_server/proto/remote_server.proto
crates/remote_server/src/protocol.rs
crates/remote_server/src/transport.rs
crates/http_client
crates/websocket

Prior research quality notes

The first Gemini pass was not reliable enough to treat as authoritative.

Known issues from that pass:

* endpoint_inventory.json was empty.
* open_questions.md was empty.
* verification lacked enough direct line-number evidence
* oz_debug_logging.patch had fake-looking/minimal git hashes
* REST/SSE/GraphQL/protobuf claims were under-supported
* some claims were too polished relative to the evidence

The second and later repair passes were better, but still need skepticism. Some claims were later corrected manually or narrowed.

Useful repaired/generated docs include:

AUDIT_REPORT.md
endpoint_inventory.json
02_cloud_api_schema_REPAIRED.md
type_inventory.json
TYPE_EVIDENCE.md
REALTIME_TRANSPORT_AUDIT.md
REMOTE_PROTO_EVIDENCE.md
AUTH_EVIDENCE.md
PATCH_AUDIT.md
oz_debug_logging_REPAIRED.patch
10_final_protocol_reference_REPAIRED.md
SESSION_STATE.md
open_questions.md
findings.json
RUNTIME_SKILLS_EVIDENCE.md
REMOTE_PROTO_LIMIT_EVIDENCE.md
GRAPHQL_OPERATION_EVIDENCE.md
INTERRUPTED_SESSION_RECOVERY.md
LOCAL_MOCK_REDIRECT_COMPILE_VERIFICATION.md
AGENT_REST_SSE_SCHEMA_EVIDENCE.md
AGENT_REST_SSE_SCHEMA.json
MOCK_SERVER_PLAN.md
MOCK_SERVER_IMPLEMENTATION.md
BUILD_RUNTIME_DIAGNOSIS.md
SOURCE_LEVEL_REDIRECT_VERIFICATION.md

However, no generated doc should be treated as ground truth unless backed by source paths, local harnesses, or directly observed runtime logs.

Verified findings

These are findings with strong local evidence or passing local tests.

Remote protobuf framing

Status: verified enough.

Existing harness:

~/reverse_warp_orchestration/warp/crates/remote_server/tests/protocol_harness.rs

Verified command:

cd ~/reverse_warp_orchestration/warp
cargo test -p remote_server --test protocol_harness

Reported result: 5 tests passed.

Covered behavior:

* successful client/server round trip
* 4-byte little-endian prefix validation
* oversized frame rejection
* truncated frame behavior
* invalid protobuf payload behavior

Findings:

* protobuf frames use a 4-byte little-endian length prefix
* max message size is:

64 * 1024 * 1024

* oversized frames produce:

ProtocolError::MessageTooLarge

* truncated frames produce:

ProtocolError::UnexpectedEof

* invalid protobuf payloads produce:

ProtocolError::Decode

* the transport uses stdio
* stderr is safe for debug logging because the framed protobuf stream uses stdout/stdin

Relevant source files:

crates/remote_server/src/protocol.rs
crates/remote_server/src/transport.rs
crates/remote_server/proto/remote_server.proto

Verdict:

The remote protobuf side is done enough for the current phase. Further work here is optional unless the Mac runtime test reveals an integration issue.

Full app compile-check

Status: verified by cargo check, not by successful binary build.

Reported command:

cd ~/reverse_warp_orchestration/warp
cargo check -p warp

Reported output:

Checking warp v0.1.0 (/home/imapotatoes11/reverse_warp_orchestration/warp/app)
Building [=====================> ] 1410/1416: warp
Finished dev profile [unoptimized + debuginfo] target(s) in 14m 37s

Verdict:

The Warp app crate type-checks with the current local changes. This does not prove the GUI binary builds or runs.

warp_core and warp_cli compile-check

Status: verified earlier.

Reported:

cargo check -p warp_core -p warp_cli passed

Verdict:

The lower-level channel/CLI-related crates were not broken by the redirect-related work.

Mock server health check

Status: locally verified.

Mock server path:

~/reverse_warp_orchestration/oz-mock-server

Correct startup pattern:

cd ~/reverse_warp_orchestration/oz-mock-server
. venv/bin/activate
python -m uvicorn server:app --host 127.0.0.1 --port 8787 --log-level debug

Health endpoint result:

{"status":"ok"}

Verdict:

The local FastAPI mock server starts and responds to /health.

Important caveat:

The mock server has not yet been hit by a running Warp GUI binary.

Source-level verified findings

These findings have source-level or harness-level verification, but not full GUI runtime verification.

OSS local mock redirect helper

Status: source-level verified and compile-check verified, but not GUI runtime verified.

Patch path:

~/reverse_warp_orchestration/research/oz/LOCAL_MOCK_REDIRECT_v2.patch

The v2 approach extracts redirect logic into:

app/src/mock_override.rs

The app startup path in:

app/src/lib.rs

calls:

mock_override::apply_oss_mock_server_root_override_for_local_testing();

in the non-standard-override branch.

The helper internally checks that the channel is OSS before applying the override.

The helper only calls:

ChannelState::override_server_root_url(url)

It does not override:

* WebSocket URL
* session-sharing URL
* auth behavior
* tokens/cookies
* request bodies

Expected environment variable:

WARP_MOCK_APP_URL

Expected runtime shape:

WARP_MOCK_APP_URL=http://127.0.0.1:8787 ./target/debug/warp-oss

Verdict:

The redirect is narrowly scoped and safe by design, but only verified at source/check level so far.

Original local mock redirect patch

Original patch path:

~/reverse_warp_orchestration/research/oz/LOCAL_MOCK_REDIRECT.patch

Original diff:

diff --git a/app/src/lib.rs b/app/src/lib.rs
index b65346d..1db45bd 100644
--- a/app/src/lib.rs
+++ b/app/src/lib.rs
@@ -524,6 +524,12 @@ pub fn run() -> Result<()> {
                 eprintln!("Error: Invalid session sharing server URL: {e:#}");
             }
         }
+    } else if matches!(ChannelState::channel(), warp_core::channel::Channel::Oss) {
+        if let Ok(url) = std::env::var("WARP_MOCK_APP_URL") {
+            if let Err(e) = ChannelState::override_server_root_url(url) {
+                eprintln!("Error: Invalid mock server root URL: {e:#}");
+            }
+        }
     }
 
     if let Some(command) = args.command() {

Verdict:

The original patch is a real git diff and is conceptually narrow, but v2 is preferable because it isolates the behavior in app/src/mock_override.rs and was source-level harness verified.

Channel URL defaults and override gates

Status: source-level supported.

Relevant base URLs are in:

crates/warp_core/src/channel/config.rs

Known production defaults include:

https://app.warp.dev
wss://rtc.app.warp.dev/graphql/v2
wss://sessions.app.warp.dev

Relevant CLI/env override constants are in:

crates/warp_cli/src/lib.rs

Known constants:

WARP_SERVER_ROOT_URL
WARP_WS_SERVER_URL
WARP_SESSION_SHARING_SERVER_URL

The normal channel gate is in:

crates/warp_core/src/channel/mod.rs

Known behavior:

* Dev
* Local
* Integration

allow server URL overrides.

Known behavior:

* Stable
* Preview
* Oss

block the normal override path.

The local v2 helper adds a deliberately narrow OSS-only mock server-root override using WARP_MOCK_APP_URL.

Verdict:

This supports the need for a special local OSS mock override if the goal is to redirect the OSS binary’s REST control-plane calls to a local server.

Cloud REST call routing through server_root_url

Status: source-level supported, not runtime verified.

Many cloud REST paths appear to use:

ChannelState::server_root_url()

Relevant files include:

app/src/server/server_api.rs
app/src/server/server_api/ai.rs
app/src/auth/auth_manager.rs
app/src/ai/agent_sdk/ambient.rs
app/src/ai/agent/api.rs

Verdict:

Changing server_root_url should redirect many REST calls. Full runtime confirmation is still missing.

Plausible but unverified findings

These are likely enough to guide cautious next steps, but should not be written as final protocol truth.

Agent cloud control plane uses REST + SSE

Current evidence supports this model:

* cloud-control-plane operations use REST-style calls
* agent event streaming appears to use SSE through stream_agent_events
* source-backed SSE event-type examples currently include:
    * new_message
    * run_started

Important caution:

Only new_message and run_started are currently source-backed examples from the evidence gathered so far.

Do not claim broader event taxonomy unless source-backed or runtime-observed.

GraphQL exists, but likely not agent realtime

GraphQL/WebSocket-related code exists.

Known GraphQL operation examples include:

GetWarpDriveUpdates

Current conclusion:

GraphQL appears more related to Warp Drive / metadata update paths than agent realtime.

Do not claim GraphQL is the agent realtime protocol unless a source path directly proves it.

SSE event schema is partial

The current SSE event schema is incomplete.

AGENT_REST_SSE_SCHEMA.json was manually corrected because an earlier Gemini-generated version overclaimed event examples.

Correct safer representation:

AgentRunEvent.event_type = "String; source-backed examples found so far: new_message, run_started. Do not assume run_in_progress/run_succeeded unless separately proven."

Mock server first-pass behavior should therefore be conservative:

* hold the SSE connection open with no events, or
* emit only a minimal source-backed new_message event

Do not emit these as if verified:

run_in_progress
run_succeeded

unless separately source-backed or runtime-tested.

runtime_skills

Status: partially resolved, not fully.

Current understanding:

* runtime_skills appears to be a Vec<String>
* each string appears to contain standard Base64
* payload appears to be a prost-encoded multi_agent_api::Skill
* population/encoding evidence points to:

app/src/pane_group/pane/terminal_pane.rs

around:

resolve_runtime_skills

Important unresolved point:

The exact Skill schema appears to come from an external/generated dependency outside the repo-local source examined so far.

Verdict:

Treat runtime_skills as opaque outbound blobs.

Mock server logging should record only safe metadata, such as:

* count
* individual string lengths
* hash prefixes
* whether decode appears possible

Do not log raw values by default.

Explicitly unverified / not attempted

Full GUI runtime redirect

Status: not verified.

The runtime redirect has not been proven because the Ubuntu VM could not finish building the GUI binary.

There is no confirmed observation yet of Warp making local requests to:

http://127.0.0.1:8787

Therefore, this claim is not yet valid:

The OSS GUI runtime successfully redirects cloud REST calls to the local mock server.

Correct current wording:

The redirect patch is source-level and compile-check verified, but full GUI runtime redirect remains unverified.

Actual target/debug/warp-oss binary

Status: not produced on Ubuntu VM.

Known binary target:

app/src/bin/oss.rs

Expected binary path if built:

target/debug/warp-oss

Known correct build command:

cd ~/reverse_warp_orchestration/warp
cargo build --bin warp-oss

Known failed/blocked runtime state:

ls -lh target/debug/warp-oss target/debug/deps/warp-oss* 2>/dev/null || true

returned no output after the interrupted build.

Known process check after interruption showed no active cargo/rustc/linker processes.

Verdict:

The Ubuntu VM reached heavy final compilation/link stages but did not produce the binary.

Mock server hit by Warp

Status: not observed.

The mock server was health-tested manually, but no Warp-originated /api/v1/... calls have been observed.

Therefore, do not infer request shapes from runtime logs yet.

Production behavior

Status: intentionally not tested.

No production services should be contacted.

Do not use production traffic as a source of truth.

Do not compare local behavior by hitting production endpoints.

Auth bypass

Status: not attempted and should not be attempted.

The redirect patch does not bypass auth and should not be extended to do so.

If local runtime requires auth-like responses, the mock should return safe placeholder responses based on local source expectations, not stolen/replayed production credentials.

Build/runtime status on Ubuntu VM

Actual OSS binary name confirmed:

cargo run --bin warp-oss

Expected binary if built:

target/debug/warp-oss

The Ubuntu VM could not complete full GUI binary build within reasonable limits.

Known behavior:

Building [=====================> ] 1350/1352: warp

The build appears to choke near final app crate compilation/linking under approximately 4 GB RAM.

This is not currently evidence of a redirect-patch failure.

Diagnosis:

* cargo build --bin warp-oss is the correct command
* the binary target is app/src/bin/oss.rs
* the current blocker is resources/final GUI build/linking
* cargo check -p warp passing is meaningful but not equivalent to a binary build

Mock server status

Mock server path:

~/reverse_warp_orchestration/oz-mock-server

Docs:

~/reverse_warp_orchestration/warp/research/oz/MOCK_SERVER_IMPLEMENTATION.md
~/reverse_warp_orchestration/warp/research/oz/MOCK_SERVER_PLAN.md

Current implementation characteristics:

* FastAPI-based
* binds to 127.0.0.1:8787
* has /health
* redacts sensitive headers/fields
* does not log raw runtime_skills
* avoids treating unverified SSE event types as confirmed

Known-good startup:

cd ~/reverse_warp_orchestration/oz-mock-server
. venv/bin/activate
python -m uvicorn server:app --host 127.0.0.1 --port 8787 --log-level debug

Known-good health check:

curl -sv http://127.0.0.1:8787/health

Expected response:

{"status":"ok"}

Known earlier mistakes:

* using bare uvicorn outside the venv
* MOCKDIR unset, causing /venv/bin/activate
* pasting a literal trailing ...

Correct approach is to use the venv and run:

python -m uvicorn ...

Runtime testing status

Runtime testing has not happened yet.

The next meaningful test is on macOS, because the Ubuntu VM could not produce the GUI binary.

Runtime redirect should only be considered verified if the mock server logs show Warp-originated requests to local endpoints after launching with:

WARP_MOCK_APP_URL=http://127.0.0.1:8787 ./target/debug/warp-oss

If the app attempts to contact production URLs, stop immediately and inspect the override path before proceeding.

Artifacts and tarballs

Known artifacts:

~/warp-oz-protobuf-harness-passing.tar.gz
~/reverse_warp_orchestration/warp-oz-local-mock-redirect-20260429T171258Z.tar.gz

Accidental huge source redirect tarball:

~/reverse_warp_orchestration/warp-oz-source-redirect-verified-20260430T154749Z.tar.gz

Size:

1.1G

Likely cause:

included harness/target build artifacts

Lean source redirect tarball:

~/reverse_warp_orchestration/warp-oz-source-redirect-verified-lean-20260430T182029Z.tar.gz

Size:

56K

Mac handoff tarball:

~/reverse_warp_orchestration/warp-oz-mac-handoff-20260430T182452Z.tar.gz

Size:

68K

Created by:

cd ~/reverse_warp_orchestration
TS="$(date -u +%Y%m%dT%H%M%SZ)"
OUT="warp-oz-mac-handoff-${TS}.tar.gz"
tar \
  --exclude='.git' \
  --exclude='target' \
  --exclude='node_modules' \
  --exclude='__pycache__' \
  --exclude='*.pyc' \
  --exclude='venv' \
  --exclude='.venv' \
  --exclude='*.log' \
  --exclude='*.pid' \
  -czf "$OUT" \
  warp/research/oz \
  research/oz/LOCAL_MOCK_REDIRECT.patch \
  research/oz/LOCAL_MOCK_REDIRECT_v2.patch \
  research/oz/harness \
  oz-mock-server \
  run_logs
ls -lh "$OUT"
echo
echo "Created: ~/reverse_warp_orchestration/$OUT"

Recommended Mac GUI runtime test

This is the safe next phase.

1. Unpack / place handoff files

Move the handoff tarball to the Mac and unpack it into the intended working area.

Preserve the distinction between:

warp/research/oz
research/oz
oz-mock-server
run_logs

2. Apply or verify the v2 redirect patch

Patch path in the handoff:

research/oz/LOCAL_MOCK_REDIRECT_v2.patch

From the Mac Warp checkout:

cd warp
git status --short
git apply --check ../research/oz/LOCAL_MOCK_REDIRECT_v2.patch
git apply ../research/oz/LOCAL_MOCK_REDIRECT_v2.patch

If the patch is already applied or does not apply cleanly, inspect manually.

Expected source-level shape:

app/src/mock_override.rs

exists, and:

app/src/lib.rs

calls:

mock_override::apply_oss_mock_server_root_override_for_local_testing();

The helper should:

* check Channel::Oss
* read WARP_MOCK_APP_URL
* call ChannelState::override_server_root_url(url)
* not touch WebSocket/session-sharing URLs
* not bypass auth
* not log secrets

3. Start local mock server

From the handoff root:

cd oz-mock-server
python3 -m venv venv
. venv/bin/activate
python -m pip install -r requirements.txt
python -m uvicorn server:app --host 127.0.0.1 --port 8787 --log-level debug

In another terminal:

curl -sv http://127.0.0.1:8787/health

Expected:

{"status":"ok"}

4. Build Warp OSS binary on Mac

From the Mac Warp checkout:

cd warp
cargo build --bin warp-oss

Expected binary:

target/debug/warp-oss

5. Run only against local mock

Only run with the local mock env var set:

cd warp
WARP_MOCK_APP_URL=http://127.0.0.1:8787 ./target/debug/warp-oss

Do not run the binary in a way that allows accidental production testing.

6. Capture logs

Capture:

* mock server terminal output
* Warp stderr/stdout if available
* any local runtime logs created under the handoff/run log directory
* exact command used
* exact environment variables used

Do not capture or paste secrets.

If logs include auth headers, cookies, tokens, or raw runtime_skills, redact them before preserving or sharing.

7. Verification condition

Runtime redirect is verified only if the mock server logs show Warp-originated requests hitting:

127.0.0.1:8787

especially endpoints under likely cloud-control-plane paths such as:

/api/v1/...

8. Stop condition

If Warp attempts to contact production services, stop immediately.

Production URLs to watch for:

https://app.warp.dev
wss://rtc.app.warp.dev/graphql/v2
wss://sessions.app.warp.dev

If this happens, inspect:

app/src/lib.rs
app/src/mock_override.rs
crates/warp_core/src/channel/state.rs
crates/warp_core/src/channel/mod.rs
crates/warp_core/src/channel/config.rs

before running again.

Open questions

1. Does the v2 redirect work in a real macOS GUI runtime?

Current status:

* source-level verified
* compile-check verified
* not runtime verified

Needed evidence:

* mock server logs showing local requests from Warp
* no production contacts during the test

2. Which REST endpoints are actually called by first-run / agent flows?

Current status:

* source inventory exists
* no runtime request log from Warp yet

Needed evidence:

* local mock server logs during first GUI launch
* local mock server logs during attempted agent/Oz interaction
* compare observed paths to endpoint_inventory.json and source call sites

3. What exact SSE event schema does the running app require?

Current status:

* partial source-backed evidence
* new_message and run_started are source-backed examples
* run_in_progress and run_succeeded should not be treated as source-backed yet

Needed evidence:

* source path proving additional event types, or
* runtime behavior showing what the client accepts/requires from local mock

Conservative mock behavior:

* hold SSE open with no events, or
* emit only minimal source-backed new_message

4. What is the exact runtime_skills schema?

Current status:

* probably Base64 of prost-encoded multi_agent_api::Skill
* exact Skill schema unresolved because it appears external/generated

Needed evidence:

* locate generated or external schema source
* avoid raw logging until resolved

Current safe handling:

* opaque blob
* log only count/length/hash prefix

5. What role, if any, does GraphQL WebSocket play in agent runtime?

Current status:

* GraphQL exists
* known operation example: GetWarpDriveUpdates
* current evidence suggests GraphQL is not the agent realtime path

Needed evidence:

* direct source path connecting GraphQL WebSocket to agent realtime, or
* local runtime logs showing agent path using GraphQL

Current safe claim:

* GraphQL/WebSocket exists in Warp, but agent realtime currently appears SSE-based.

6. Does the mock need to model auth/session state?

Current status:

* not yet known
* no runtime request logs yet

Needed evidence:

* first Mac runtime test
* observed failing endpoints/status codes

Constraint:

* do not bypass auth
* do not replay production tokens
* local mock may provide safe placeholder responses if source-backed and needed for local-only testing

Current verdict

The remote protobuf side is solid and locally tested.

The cloud REST/SSE control-plane side is partially mapped but not fully runtime-validated.

The local OSS mock redirect is narrow and source-level verified, but full GUI runtime redirect remains unverified until the Mac build/test.

The mock server is implemented and health-tested, but has not yet received live requests from Warp.

The next correct move is a macOS GUI runtime test with WARP_MOCK_APP_URL=http://127.0.0.1:8787, while watching carefully for any accidental production URL access.
