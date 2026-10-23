<script lang="ts">
  import { onMount } from "svelte";
  import {
    initScratchpadListener,
    loadScratchpad,
    onScratchpadChanged,
    saveScratchpad,
  } from "./scratchpadStore.svelte";

  let { projectId }: { projectId: string } = $props();
  let text = $state("");
  let revision = $state(0);
  let dirty = $state(false);
  let error = $state("");

  async function reload() {
    try {
      const p = await loadScratchpad(projectId);
      text = p.text;
      revision = p.revision;
      dirty = false;
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function save() {
    try {
      revision = await saveScratchpad(projectId, text, revision);
      dirty = false;
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  // Reload when the project changes.
  $effect(() => {
    projectId;
    reload();
  });

  onMount(() => {
    initScratchpadListener();
    return onScratchpadChanged((id) => {
      if (id === projectId && !dirty) reload();
    });
  });
</script>

<div class="pad">
  <div class="bar">
    <span class="rev">rev {revision}{dirty ? " (edited)" : ""}</span>
    <button type="button" disabled={!dirty} onclick={save}>Save</button>
    <button type="button" onclick={reload}>Reload</button>
  </div>
  {#if error}
    <div class="err">
      {error}
      <button type="button" onclick={reload}>Reload</button>
    </div>
  {/if}
  <textarea
    placeholder="Project scratchpad (Markdown). Agents can use `agentfleet scratchpad`."
    bind:value={text}
    oninput={() => (dirty = true)}
  ></textarea>
</div>

<style>
  .pad {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    font-size: 13px;
  }
  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-bottom: 1px solid var(--border);
  }
  .rev {
    flex: 1;
    color: var(--text-muted);
    font-family: ui-monospace, monospace;
  }
  .err {
    color: var(--danger);
    padding: 4px 8px;
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
  textarea {
    flex: 1;
    resize: none;
    background: var(--bg-surface);
    color: inherit;
    border: none;
    padding: 8px;
    font-family: ui-monospace, monospace;
    font-size: 13px;
    outline: none;
  }
</style>
