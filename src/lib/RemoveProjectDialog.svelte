<script lang="ts">
  import { removeProject, store } from "./sessions.svelte";

  const id = store.removeProjectId as string;
  const name = $derived(store.projects.find((p) => p.id === id)?.name ?? "project");
  const n = $derived(store.tabs.filter((t) => t.projectId === id).length);
  let busy = $state(false);
  let error = $state("");

  function cancel() {
    store.removeProjectId = null;
  }

  async function confirm() {
    busy = true;
    error = "";
    try {
      await removeProject(id);
      store.removeProjectId = null;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && !busy && cancel()} />

<div class="backdrop">
  <div class="dialog">
    <h3>Remove {name}?</h3>
    {#if n > 0}
      <p>
        {n} open session{n === 1 ? "" : "s"} will be closed. Clean worktrees are deleted, dirty ones stay on disk. The repo folder is not touched.
      </p>
    {:else}
      <p>The repo folder is not touched.</p>
    {/if}
    {#if error}<p class="error">{error}</p>{/if}
    <div class="row actions">
      <button type="button" disabled={busy} onclick={cancel}>Cancel</button>
      <button type="button" class="danger" disabled={busy} onclick={confirm}>Remove</button>
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
  h3,
  p {
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
  .danger {
    background: #f85149;
    color: #fff;
    border: none;
  }
  .error {
    color: #f48771;
  }
</style>
