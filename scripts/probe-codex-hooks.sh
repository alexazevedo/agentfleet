#!/bin/sh
# Runtime check of the Codex hook contract used by agentfleet (ticket 32).
# Makes one real Codex model call with the user's login.
# Exit 0 ok; 1 failure (prints files); 2 codex not on PATH.
command -v codex >/dev/null 2>&1 || { echo "codex not on PATH" >&2; exit 2; }

tmp=$(mktemp -d "${TMPDIR:-/tmp}/probe-codex.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
status="$tmp/status"
payload="$tmp/payload.json"
hook="$tmp/hook.sh"
cat > "$hook" <<HOOK
#!/bin/sh
# \$1 = status word; stdin = Codex hook payload (JSON with session_id).
cat > '$payload'
printf '%s' "\$1" > '$status'
HOOK
chmod 755 "$hook"

cd "$tmp" || exit 1
codex exec --skip-git-repo-check --no-daemon --dangerously-bypass-hook-trust \
  -c "hooks.SessionStart=[{hooks=[{type=\"command\",command=\"$hook waiting\"}]}]" \
  -c "hooks.Stop=[{hooks=[{type=\"command\",command=\"$hook waiting\"}]}]" \
  'Reply with the single word pong.' </dev/null >"$tmp/out.txt" 2>&1 &
pid=$!
# macOS has no coreutils `timeout` by default: a watchdog kills codex after 180 s.
( sleep 180; kill "$pid" 2>/dev/null ) &
watchdog=$!
wait "$pid"
code=$?
kill "$watchdog" 2>/dev/null
echo "codex exit: $code"

word=$(cat "$status" 2>/dev/null)
if [ "$code" -eq 0 ] && [ "$word" = "waiting" ] && grep -q '"session_id"' "$payload" 2>/dev/null; then
  echo "status: $word"
  echo "payload has session_id: yes"
  exit 0
fi
echo "FAIL"
echo "--- status: $word"
echo "--- payload:"
cat "$payload" 2>/dev/null
echo "--- codex output (tail):"
tail -40 "$tmp/out.txt"
exit 1
