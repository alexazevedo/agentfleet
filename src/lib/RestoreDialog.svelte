<script lang="ts">
  import { attachRestored, forgetRestorable, store } from "./sessions.svelte";
  import { saveSettings, settings } from "./settingsStore.svelte";
  import Dialog from "./ui/Dialog.svelte";
  import Button from "./ui/Button.svelte";
  import Switch from "./ui/Switch.svelte";

  let remember = $state(false);
  let busy = $state(false);

  const count = $derived(store.pendingRestore.length);

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

<Dialog
  bind:open={
    () => store.restoreDialogOpen,
    (v) => {
      store.restoreDialogOpen = v;
    }
  }
  title="Restore {count} session{count === 1 ? '' : 's'}?"
  dismissible={false}
>
  <div class="content">
    <ul>
      {#each store.pendingRestore as i (i.id)}<li>{label(i)}</li>{/each}
    </ul>
    <Switch bind:checked={remember}>
      {#snippet label()}Don't ask again{/snippet}
    </Switch>
  </div>
  {#snippet footer()}
    <Button disabled={busy} onclick={() => choose("fresh")}>Start fresh</Button>
    <Button variant="primary" disabled={busy} onclick={() => choose("restore")} autofocus>Restore</Button>
  {/snippet}
</Dialog>

<style>
  .content {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    font-size: var(--text-md);
  }
  ul {
    margin: 0;
    padding-left: 18px;
    max-height: 240px;
    overflow: auto;
  }
</style>
