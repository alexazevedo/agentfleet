<script lang="ts">
  import { initProject, store } from "./sessions.svelte";
  import Dialog from "./ui/Dialog.svelte";
  import Button from "./ui/Button.svelte";

  const req = store.initRepo as NonNullable<typeof store.initRepo>;
  const name = req.path.split("/").filter(Boolean).at(-1) ?? req.path;
  let busy = $state(false);
  let error = $state("");

  function cancel() {
    store.initRepo = null;
    req.done?.(null);
  }

  async function confirm() {
    busy = true;
    error = "";
    try {
      const p = await initProject(req.path);
      store.initRepo = null;
      req.done?.(p);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog
  bind:open={
    () => store.initRepo !== null,
    (v) => {
      if (!v) cancel();
    }
  }
  title="Initialize a git repo in {name}?"
  dismissible={!busy}
  onclose={cancel}
>
  <div class="content">
    <p class="path" title={req.path}>{req.path}</p>
    <p>
      This folder is not a git repository. Each session runs in its own worktree, so the project needs one.
    </p>
    <p>Runs <code>git init</code> and creates an empty initial commit. Nothing is pushed anywhere.</p>
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  </div>
  {#snippet footer()}
    <Button disabled={busy} onclick={cancel}>Cancel</Button>
    <Button variant="primary" disabled={busy} onclick={confirm} autofocus>
      {busy ? "Initializing…" : "Initialize"}
    </Button>
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
  .path {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  code {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }
  .error {
    color: var(--danger);
  }
</style>
