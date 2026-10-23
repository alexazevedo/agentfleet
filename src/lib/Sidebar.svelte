<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Folder, NotebookPen, Settings, Plus, FolderPlus, Trash2, X } from "lucide-svelte";
  import Button from "./ui/Button.svelte";
  import AgentIcon from "./AgentIcon.svelte";
  import NewSessionPopover from "./NewSessionPopover.svelte";
  import { collab, initCollabListener, loadRuns, queuedCount } from "./collab.svelte";
  import {
    addProjectOrOfferInit,
    closeTab,
    initStatusListener,
    initSpawnListener,
    setActive,
    store,
    tabTitle,
  } from "./sessions.svelte";

  onMount(() => {
    initStatusListener();
    initSpawnListener();
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
      await addProjectOrOfferInit(picked);
    } catch (e) {
      store.error = String(e);
    }
  }

  function openScratchpad(projectId: string) {
    store.activeKey = "scratchpad:" + projectId;
    store.view = "scratchpad";
  }

  function openSettings() {
    store.activeKey = "settings";
    store.view = "settings";
  }

  function onNewSession(e: MouseEvent, projectId: string | null) {
    store.error = "";
    store.newSession = { anchor: e.currentTarget as HTMLElement, projectId };
  }
</script>

<aside class="sidebar">
  <div class="header">
    <span>Projects</span>
    <span class="actions">
      <Button variant="ghost" size="sm" aria-label="Add project" onclick={onAddProject}><FolderPlus size={14} strokeWidth={1.75} /></Button>
      <Button variant="ghost" size="sm" aria-label="New session" onclick={(e: MouseEvent) => onNewSession(e, null)}><Plus size={14} strokeWidth={1.75} /></Button>
    </span>
  </div>
  {#if store.error && store.newSession === null}
    <div class="banner" role="alert">
      {store.error}
      <button type="button" aria-label="Dismiss" onclick={() => (store.error = "")}>×</button>
    </div>
  {/if}
  <div class="list">
    {#each store.projects as p (p.id)}
      <div class="project">
        <span class="name" title={p.path}><Folder size={14} strokeWidth={1.75} /> {p.name}</span>
        <span class="actions">
          <Button variant="ghost" size="sm" aria-label="New session" onclick={(e: MouseEvent) => onNewSession(e, p.id)}><Plus size={14} strokeWidth={1.75} /></Button>
          <span class="remove"><Button variant="ghost" size="sm" aria-label="Remove project" onclick={() => (store.removeProjectId = p.id)}><Trash2 size={14} strokeWidth={1.75} /></Button></span>
        </span>
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
            <span>{tabTitle(tab.info)}</span>
            <span class="id8">{tab.info.id8}</span>
            {#if tab.info.model}<span class="model">{tab.info.model}</span>{/if}
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
  <div
    class="settings"
    class:active={store.view === "settings"}
    role="button"
    tabindex="0"
    onclick={openSettings}
    onkeydown={(e) => e.key === "Enter" && openSettings()}
  >
    <Settings size={14} strokeWidth={1.75} /> Settings
  </div>
  <NewSessionPopover />
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
    overflow: hidden;
    min-width: 0;
    font-family: var(--font-ui);
    font-size: var(--text-md);
    color: var(--text);
  }
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) 10px var(--space-1);
    font-size: var(--text-xs);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    font-weight: 600;
  }
  .project {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    margin: 0 4px;
    padding: 0 6px;
    border-radius: var(--radius-sm);
  }
  .project:hover {
    background: var(--bg-hover);
  }
  .banner {
    background: color-mix(in srgb, var(--danger) 15%, transparent);
    color: var(--danger);
    font-size: var(--text-sm);
    padding: var(--space-1) var(--space-2);
  }
  .banner button {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
  }
  .list {
    flex: 1;
    overflow-y: auto;
  }
  .actions {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    flex: none;
  }
  .name {
    flex: 1;
    min-width: 0;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .session {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 26px;
    margin: 0 4px;
    padding: 0 6px 0 24px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    cursor: pointer;
    white-space: nowrap;
  }
  .session:hover {
    background: var(--bg-hover);
  }
  .session.active {
    background: var(--bg-active);
    color: var(--text);
  }
  .id8 {
    color: var(--text-muted);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }
  .model {
    color: var(--text-muted);
    font-size: var(--text-xs);
    max-width: 9em;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .spacer {
    flex: 1;
  }
  .close {
    visibility: hidden;
    padding: 0 4px;
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
  }
  .session:hover .close {
    visibility: visible;
  }
  .close:hover {
    color: var(--text);
  }
  .remove {
    visibility: hidden;
    display: inline-flex;
  }
  .project:hover .remove {
    visibility: visible;
  }
  .remove:hover :global(button) {
    color: var(--danger);
  }
  .kept {
    font-size: var(--text-xs);
    color: var(--warning);
  }
  .badge {
    background: var(--accent);
    color: var(--text-inverse);
    border-radius: 8px;
    font-size: 10px;
    padding: 0 5px;
    line-height: 14px;
  }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--neutral);
    box-shadow: 0 0 0 1px var(--bg-sidebar);
    flex: none;
  }
  .dot.working {
    background: var(--success);
  }
  .dot.waiting {
    background: var(--warning);
  }
  .dot.blocked {
    background: var(--warning);
  }
  .dot.ended {
    background: var(--danger);
  }
  .settings {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 32px;
    padding: 0 10px;
    border-top: 1px solid var(--border);
    color: var(--text-muted);
    cursor: pointer;
  }
  .settings:hover {
    background: var(--bg-hover);
  }
  .settings.active {
    background: var(--bg-active);
    color: var(--text);
  }
</style>
