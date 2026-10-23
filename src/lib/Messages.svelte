<script lang="ts">
  import { onMount } from "svelte";
  import AgentIcon from "./AgentIcon.svelte";
  import { labelOf, store, type Tab } from "./sessions.svelte";
  import { cancelRun, collab, executeRun, loadRuns, sendRequest } from "./collab.svelte";

  let { tab }: { tab: Tab } = $props();
  const me = $derived(tab.info?.id ?? "");
  const runs = $derived(collab.runsBySession[me] ?? []);
  const targets = $derived(
    store.tabs.filter(
      (t) => t.projectId === tab.projectId && t.agent !== "shell" && t.info && t.info.id !== me,
    ),
  );

  let open = $state<Record<string, boolean>>({});
  let target = $state("");
  let text = $state("");
  let error = $state("");

  $effect(() => {
    if (!targets.some((t) => t.info!.id === target)) target = targets[0]?.info!.id ?? "";
  });

  onMount(() => {
    if (tab.info) loadRuns(tab.info.id);
  });

  function agentOf(id: string) {
    return store.tabs.find((t) => t.info?.id === id)?.info?.agent;
  }

  function id8(id: string) {
    return store.tabs.find((t) => t.info?.id === id)?.info?.id8 ?? id.slice(0, 8);
  }

  async function act(fn: () => Promise<unknown>) {
    error = "";
    try {
      await fn();
    } catch (e) {
      error = String(e);
    }
  }

  async function send() {
    if (!target || !text.trim()) return;
    await act(async () => {
      await sendRequest(me, target, text);
      text = "";
    });
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      send();
    }
  }
</script>

<div class="messages">
  <div class="list">
    {#each runs as r (r.id)}
      {@const incoming = r.toSessionId === me}
      {@const other = incoming ? r.fromSessionId : r.toSessionId}
      {@const otherAgent = agentOf(other)}
      <div class="row">
        <div class="head">
          <span class="chip {r.status}">{r.status}</span>
          <span class="dir"
            >{incoming ? "←" : "→"}
            {#if otherAgent}<AgentIcon agent={otherAgent} size={12} />{/if}
            {id8(other)}</span
          >
          <span class="first">{r.prompt.split("\n")[0]}</span>
          <button type="button" onclick={() => (open[r.id] = !open[r.id])}>
            {open[r.id] ? "Hide" : "Details"}
          </button>
          {#if incoming && r.status === "queued"}
            <button type="button" onclick={() => act(() => executeRun(r.id))}>Execute</button>
          {/if}
          {#if r.status === "queued" || r.status === "running"}
            <button type="button" onclick={() => act(() => cancelRun(r.id))}>Cancel</button>
          {/if}
        </div>
        {#if open[r.id]}
          <div class="details">
            <div class="label">Prompt</div>
            <pre>{r.prompt}</pre>
            {#if r.response}<div class="label">Response</div><pre>{r.response}</pre>{/if}
            {#if r.detail}<div class="label">Detail</div><pre>{r.detail}</pre>{/if}
          </div>
        {/if}
      </div>
    {:else}
      <div class="none">No messages yet</div>
    {/each}
  </div>
  <div class="compose">
    {#if error}<div class="err">{error}</div>{/if}
    {#if targets.length === 0}
      <div class="none">No other agent session in this project to message</div>
    {/if}
    <div class="line">
      <select bind:value={target} disabled={targets.length === 0}>
        {#each targets as t (t.key)}
          <option value={t.info!.id}>{labelOf(t.agent)} {t.info!.id8}</option>
        {/each}
      </select>
      <button type="button" disabled={targets.length === 0 || !text.trim()} onclick={send}>
        Send
      </button>
    </div>
    <textarea
      rows="3"
      placeholder="Request work from another session (Cmd/Ctrl+Enter to send)"
      bind:value={text}
      onkeydown={onKey}
      disabled={targets.length === 0}
    ></textarea>
  </div>
</div>

<style>
  .messages {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    font-size: 13px;
  }
  .list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }
  .row {
    border-bottom: 1px solid var(--border);
    padding: 6px 0;
  }
  .head {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .first {
    flex: 1;
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .dir {
    font-family: ui-monospace, monospace;
    color: var(--text-muted);
  }
  .chip {
    font-size: 11px;
    padding: 1px 6px;
    border-radius: 8px;
    color: var(--text-inverse);
    background: var(--neutral);
  }
  .chip.running {
    background: var(--accent);
  }
  .chip.completed {
    background: var(--success);
  }
  .chip.cancelled,
  .chip.interrupted {
    background: var(--warning);
  }
  .chip.needsAttention {
    background: var(--danger);
  }
  button {
    background: var(--bg-control);
    border: 1px solid var(--border-strong);
    color: inherit;
    border-radius: 3px;
    cursor: pointer;
    padding: 2px 8px;
  }
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .details {
    padding: 6px 0 0 8px;
  }
  .label {
    color: var(--text-muted);
    font-size: 11px;
    margin-top: 4px;
  }
  pre {
    margin: 2px 0;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: ui-monospace, monospace;
    font-size: 12px;
  }
  .none {
    color: var(--text-muted);
    padding: 6px 0;
  }
  .err {
    color: var(--danger);
    padding-bottom: 4px;
  }
  .compose {
    border-top: 1px solid var(--border);
    padding: 8px;
  }
  .line {
    display: flex;
    gap: 8px;
    margin-bottom: 6px;
  }
  textarea {
    width: 100%;
    box-sizing: border-box;
    resize: vertical;
    background: var(--bg-surface);
    color: inherit;
    border: 1px solid var(--border-strong);
    border-radius: 3px;
    font: inherit;
  }
</style>
