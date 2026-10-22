<script lang="ts">
  import { attachRestored, forgetRestorable, store } from "./sessions.svelte";
  import { saveSettings, settings } from "./settingsStore.svelte";

  let remember = $state(false);
  let busy = $state(false);

  function label(i: (typeof store.pendingRestore)[number]) {
    const name = store.projects.find((p) => p.id === i.projectId)?.name ?? i.repoName;
    return `${name} · ${i.agent} ${i.id8}`;
  }

  async function choose(choice: "restore" | "fresh") {
    busy = true;
    if (remember) {
      try {
        await saveSettings({
          ...$state.snapshot(settings.value),
          restoreSessions: choice === "restore" ? "always" : "never",
        });
      } catch (e) {
        store.error = String(e);
      }
    }
    if (choice === "restore") attachRestored();
    else await forgetRestorable();
    store.restoreDialogOpen = false;
    store.restoreDecided = true;
  }
</script>

<div class="overlay">
  <div class="dialog" role="dialog" aria-modal="true">
    <h3>Restore {store.pendingRestore.length} session{store.pendingRestore.length === 1 ? "" : "s"}?</h3>
    <ul>
      {#each store.pendingRestore as i (i.id)}<li>{label(i)}</li>{/each}
    </ul>
    <label><input type="checkbox" bind:checked={remember} /> Don't ask again</label>
    <div class="row">
      <button type="button" disabled={busy} onclick={() => choose("fresh")}>Start fresh</button>
      <button type="button" disabled={busy} onclick={() => choose("restore")}>Restore</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .dialog {
    background: #252526;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 16px;
    width: 440px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    font-size: 13px;
  }
  h3 {
    margin: 0;
  }
  ul {
    margin: 0;
    padding-left: 18px;
    max-height: 240px;
    overflow: auto;
  }
  .row {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
  button {
    background: #333;
    border: 1px solid #444;
    color: inherit;
    border-radius: 3px;
    cursor: pointer;
    padding: 4px 12px;
  }
</style>
