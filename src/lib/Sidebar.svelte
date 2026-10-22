<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Folder, NotebookPen, Settings, Plus, X } from "lucide-svelte";
  import AgentIcon from "./AgentIcon.svelte";
  import { collab, initCollabListener, loadRuns, queuedCount } from "./collab.svelte";
  import {
    addProject,
    closeTab,
    initStatusListener,
    setActive,
    store,
  } from "./sessions.svelte";

  onMount(() => {
    initStatusListener();
    initCollabListener();
  });

  // Initial load so badges show before any collab-changed event.
  $effect(() => {
    for (const t of store.tabs) {
      if (t.info && !(t.info.id in collab.runsBySession)) loadRuns(t.info.id);
    }
  });

  async function onAddProject() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked !== "string") return;
    try {
      store.error = "";
      await addProject(picked);
    } catch (e) {
      store.error = String(e);
    }
  }

  function openScratchpad(projectId: string) {
    store.activeKey = "scratchpad:" + projectId;
    store.view = "scratchpad";
  }

  function onNewSession(projectId: string) {
    store.error = "";
    store.preselectedProjectId = projectId;
    store.dialogOpen = true;
  }
</script>

<aside class="sidebar">
  <div class="header">
    <span>Projects</span>
    <button type="button" aria-label="Add project" onclick={onAddProject}><Plus size={14} strokeWidth={1.75} /></button>
  </div>
  <div class="list">
    {#each store.projects as p (p.id)}
      <div class="project">
        <span class="name" title={p.path}><Folder size={14} strokeWidth={1.75} /> {p.name}</span>
        <button type="button" aria-label="New session" onclick={() => onNewSession(p.id)}><Plus size={14} strokeWidth={1.75} /></button>
      </div>
      <div
        class="session"
        class:active={store.activeKey === "scratchpad:" + p.id}
        role="button"
        tabindex="0"
        onclick={() => openScratchpad(p.id)}
        onkeydown={(e) => e.key === "Enter" && openScratchpad(p.id)}
      >
        <NotebookPen size={14} strokeWidth={1.75} />
        <span>Scratchpad</span>
      </div>
      {#each store.tabs.filter((t) => t.projectId === p.id) as tab (tab.key)}
        <div
          class="session"
          class:active={tab.key === store.activeKey}
          role="button"
          tabindex="0"
          onclick={() => setActive(tab.key)}
          onkeydown={(e) => e.key === "Enter" && setActive(tab.key)}
        >
          {#if tab.info}
            <AgentIcon agent={tab.info.agent} size={14} />
            <span>{tab.info.agent === "claude" ? "Claude Code" : "Shell"}</span>
            <span class="id8">{tab.info.id8}</span>
          {:else}
            <span class="id8">starting…</span>
          {/if}
          <span class="spacer"></span>
          <button
            type="button"
            class="close"
            aria-label="Close session"
            onclick={(e) => {
              e.stopPropagation();
              closeTab(tab.key);
            }}><X size={14} strokeWidth={1.75} /></button
          >
          {#if tab.info && queuedCount(tab.info.id) > 0}
            <span class="badge" title="Queued requests">{queuedCount(tab.info.id)}</span>
          {/if}
          {#if tab.closing === "kept"}
            <span class="kept" title={tab.keptPath}>kept</span>
          {:else}
            <span class="dot {tab.status ?? 'unknown'}"></span>
          {/if}
        </div>
      {/each}
    {/each}
  </div>
  <div class="settings"><Settings size={14} strokeWidth={1.75} /> Settings</div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    background: #252526;
    border-right: 1px solid #111;
    overflow: hidden;
    min-width: 0;
    font-size: 13px;
  }
  .header,
  .project {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 10px;
  }
  .header {
    font-weight: 600;
    font-size: 12px;
    color: #aaa;
    padding-top: 10px;
  }
  .list {
    flex: 1;
    overflow-y: auto;
  }
  .name {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  button {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    font-size: 16px;
  }
  .session {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 10px 4px 28px;
    cursor: pointer;
    white-space: nowrap;
  }
  .session:hover {
    background: #2a2d2e;
  }
  .session.active {
    background: #37373d;
  }
  .id8 {
    color: #888;
    font-family: ui-monospace, monospace;
    font-size: 12px;
  }
  .spacer {
    flex: 1;
  }
  .close {
    visibility: hidden;
    padding: 0 4px;
  }
  .session:hover .close {
    visibility: visible;
  }
  .kept {
    font-size: 11px;
    color: #e3a008;
  }
  .badge {
    background: #0e639c;
    color: #fff;
    border-radius: 8px;
    font-size: 10px;
    padding: 0 5px;
    line-height: 14px;
  }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #888;
  }
  .dot.working {
    background: #3fb950;
  }
  .dot.waiting {
    background: #e3a008;
  }
  .dot.ended {
    background: #f85149;
  }
  .settings {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 10px;
    border-top: 1px solid #111;
    color: #666;
    cursor: not-allowed;
  }
</style>
