<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { loadSettings, settings } from "./settingsStore.svelte";
  import { addPending, addProject, loadProjects, store, type AgentKind } from "./sessions.svelte";

  let projectId = $state("");
  let agent = $state<AgentKind>("claude");

  onMount(async () => {
    try {
      if (!settings.loaded) await loadSettings();
      agent = settings.value.defaultAgent;
    } catch {}
    await loadProjects();
    if (store.preselectedProjectId) {
      projectId = store.preselectedProjectId;
      return;
    }
    // Default: project of the most recent tab, else the first project.
    const last = store.tabs.at(-1)?.projectId;
    projectId = store.projects.find((p) => p.id === last)?.id ?? store.projects[0]?.id ?? "";
  });

  async function add() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked !== "string") return;
    try {
      store.error = "";
      projectId = (await addProject(picked)).id;
    } catch (e) {
      store.error = String(e);
    }
  }

  function close() {
    store.dialogOpen = false;
    store.error = "";
    store.preselectedProjectId = null;
  }

  function create() {
    if (!projectId) {
      store.error = "choose or add a project";
      return;
    }
    store.error = "";
    addPending(projectId, agent);
    close();
  }
</script>

<div class="backdrop">
  <div class="dialog">
    <h3>New tab</h3>
    {#if store.preselectedProjectId}
      <div>{store.projects.find((p) => p.id === store.preselectedProjectId)?.name}</div>
    {:else}
    <div class="row">
      <select bind:value={projectId}>
        {#if store.projects.length === 0}<option value="">no projects</option>{/if}
        {#each store.projects as p (p.id)}
          <option value={p.id}>{p.name}</option>
        {/each}
      </select>
      <button type="button" onclick={add}>Add project…</button>
    </div>
    {/if}
    <label><input type="radio" bind:group={agent} value="claude" /> Claude Code</label>
    <label><input type="radio" bind:group={agent} value="shell" /> Shell</label>
    {#if store.error}<p class="error">{store.error}</p>{/if}
    <div class="row actions">
      <button type="button" onclick={close}>Cancel</button>
      <button type="button" onclick={create}>Create</button>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
  }
  .dialog {
    background: #2d2d30;
    padding: 16px;
    border-radius: 6px;
    min-width: 360px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  h3 {
    margin: 0;
  }
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  .actions {
    justify-content: flex-end;
  }
  select {
    flex: 1;
  }
  .error {
    color: #f48771;
    margin: 0;
  }
</style>
