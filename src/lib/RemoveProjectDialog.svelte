<script lang="ts">
  import { removeProject, store } from "./sessions.svelte";
  import Dialog from "./ui/Dialog.svelte";
  import Button from "./ui/Button.svelte";

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

<Dialog
  bind:open={
    () => store.removeProjectId !== null,
    (v) => {
      if (!v) cancel();
    }
  }
  title="Remove {name}?"
  dismissible={!busy}
  onclose={cancel}
>
  <div class="content">
    {#if n > 0}
      <p>
        {n} open session{n === 1 ? "" : "s"} will be closed. Clean worktrees are deleted, dirty ones stay on disk. The repo folder is not touched.
      </p>
    {:else}
      <p>The repo folder is not touched.</p>
    {/if}
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  </div>
  {#snippet footer()}
    <Button disabled={busy} onclick={cancel}>Cancel</Button>
    <Button variant="danger" disabled={busy} onclick={confirm} autofocus>Remove</Button>
  {/snippet}
</Dialog>

<style>
  .content {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    font-size: var(--text-md);
  }
  p {
    margin: 0;
  }
  .error {
    color: var(--danger);
  }
</style>
