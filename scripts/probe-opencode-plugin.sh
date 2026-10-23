#!/bin/sh
# Runtime check of the opencode plugin contract used by agentfleet.
# Makes one real opencode model call with the user's login.
# Exit 0 ok; 1 failure (prints files); 2 opencode not on PATH.
command -v opencode >/dev/null 2>&1 || { echo "opencode not on PATH" >&2; exit 2; }

tmp=$(mktemp -d "${TMPDIR:-/tmp}/probe-opencode.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
status="$tmp/status"
payload="$tmp/payload.json"
plugin="$tmp/plugin.js"
cat > "$plugin" <<PLUGIN
import fs from "node:fs";
const STATUS = "$status";
const PAYLOAD = "$payload";
const write = (word) => { try { fs.writeFileSync(STATUS, word); } catch {} };
const root = () => {
  try { return JSON.parse(fs.readFileSync(PAYLOAD, "utf8")).session_id; } catch { return undefined; }
};
const mine = (id) => { const r = root(); return !r || r === id; };
export const AgentfleetStatus = async () => ({
  event: async ({ event }) => {
    const p = event.properties;
    switch (event.type) {
      case "session.created":
        if (p.info && !p.info.parentID && !root()) {
          try { fs.writeFileSync(PAYLOAD, JSON.stringify({ session_id: p.info.id })); } catch {}
          write("waiting");
        }
        break;
      case "session.status":
        if (mine(p.sessionID)) write(p.status && p.status.type === "idle" ? "waiting" : "working");
        break;
      case "session.idle":
      case "permission.updated":
        if (mine(p.sessionID)) write("waiting");
        break;
      case "permission.replied":
        if (mine(p.sessionID)) write("working");
        break;
    }
  },
  "chat.message": async (input) => { if (mine(input.sessionID)) write("working"); },
});
PLUGIN

cd "$tmp" || exit 1
# OPENCODE_PROBE_MODEL picks the model (e.g. opencode/nemotron-3.5-lightning-free);
# a user-configured MCP server that never exits can keep `opencode run` alive past
# the watchdog, so probe in a clean config dir if that happens.
model=${OPENCODE_PROBE_MODEL:+-m $OPENCODE_PROBE_MODEL}
# shellcheck disable=SC2086
OPENCODE_CONFIG_CONTENT="{\"plugin\":[\"file://$plugin\"]}" \
  opencode run $model 'Reply with the single word pong.' </dev/null >"$tmp/out.txt" 2>&1 &
pid=$!
# macOS has no coreutils `timeout` by default: a watchdog kills opencode after 180 s.
( sleep 180; kill "$pid" 2>/dev/null ) &
watchdog=$!
wait "$pid"
code=$?
kill "$watchdog" 2>/dev/null
echo "opencode exit: $code"

word=$(cat "$status" 2>/dev/null)
if [ "$code" -eq 0 ] && [ "$word" = "waiting" ] && grep -q '"session_id"' "$payload" 2>/dev/null; then
  echo "status: $word"
  echo "payload has session_id: yes"
  exit 0
fi
echo "FAIL"
echo "--- status file ---"; cat "$status" 2>/dev/null
echo "--- payload ---"; cat "$payload" 2>/dev/null
echo "--- opencode output ---"; tail -40 "$tmp/out.txt" 2>/dev/null
exit 1
