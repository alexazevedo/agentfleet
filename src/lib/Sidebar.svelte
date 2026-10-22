<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import {
    addProject,
    closeTab,
    initStatusListener,
    setActive,
    store,
  } from "./sessions.svelte";

  onMount(initStatusListener);

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

  function onNewSession(projectId: string) {
    store.error = "";
    store.preselectedProjectId = projectId;
    store.dialogOpen = true;
  }
</script>

<aside class="sidebar">
  <div class="header">
    <span>Projects</span>
    <button type="button" aria-label="Add project" onclick={onAddProject}>+</button>
  </div>
  <div class="list">
    {#each store.projects as p (p.id)}
      <div class="project">
        <span class="name" title={p.path}>📁 {p.name}</span>
        <button type="button" aria-label="New session" onclick={() => onNewSession(p.id)}>+</button>
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
            }}>×</button
          >
          {#if tab.closing === "kept"}
            <span class="kept" title={tab.keptPath}>kept</span>
          {:else}
            <span class="dot {tab.status ?? 'unknown'}"></span>
          {/if}
        </div>
      {/each}
    {/each}
  </div>
  <div class="settings">Settings</div>
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
    padding-left: 36px; /* room for the toggle button */
  }
  .list {
    flex: 1;
    overflow-y: auto;
  }
  .name {
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
    padding: 8px 10px;
    border-top: 1px solid #111;
    color: #666;
    cursor: not-allowed;
  }
</style>
