#!/bin/sh
# Runtime check of the pi extension contract used by agentfleet.
# Makes one real pi model call with the user's default provider/model.
# Exit 0 ok; 1 failure (prints files); 2 pi not on PATH.
command -v pi >/dev/null 2>&1 || { echo "pi not on PATH" >&2; exit 2; }

tmp=$(mktemp -d "${TMPDIR:-/tmp}/probe-pi.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
status="$tmp/status"
ext="$tmp/ext.js"
cat > "$ext" <<EXT
import fs from "node:fs";
const STATUS = "$status";
const write = (word) => { try { fs.writeFileSync(STATUS, word); } catch {} };
export default function (pi) {
  pi.on("session_start", async () => write("waiting"));
  pi.on("agent_start", async () => write("working"));
  pi.on("agent_end", async () => write("waiting"));
}
EXT

cd "$tmp" || exit 1
# PI_PROBE_ARGS can pick a provider/model, e.g. "--provider ollama --model gemma4:12b-mlx";
# the default uses pi's configured default model.
# shellcheck disable=SC2086
pi -e "$ext" --no-session --no-extensions --no-skills --no-context-files ${PI_PROBE_ARGS:-} \
  -p 'Reply with the single word pong.' </dev/null >"$tmp/out.txt" 2>&1 &
pid=$!
# macOS has no coreutils `timeout` by default: a watchdog kills pi after 180 s.
( sleep 180; kill "$pid" 2>/dev/null ) &
watchdog=$!
wait "$pid"
code=$?
kill "$watchdog" 2>/dev/null
echo "pi exit: $code"

word=$(cat "$status" 2>/dev/null)
if [ "$code" -eq 0 ] && [ "$word" = "waiting" ]; then
  echo "status: $word"
  exit 0
fi
echo "FAIL"
echo "--- status file ---"; cat "$status" 2>/dev/null
echo "--- pi output ---"; cat "$tmp/out.txt" 2>/dev/null
exit 1
