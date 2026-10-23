<script lang="ts">
  import { dismissUpdate, installUpdate, updater } from "./updater.svelte";
  import Dialog from "./ui/Dialog.svelte";
  import Button from "./ui/Button.svelte";
  let busy = $state(false);

  async function install() {
    busy = true;
    await installUpdate();
    busy = false;
  }
</script>

<Dialog
  bind:open={
    () => !!updater.available,
    (v) => {
      if (!v) dismissUpdate();
    }
  }
  title="Update available: {updater.available?.version}"
  dismissible={!busy}
  onclose={dismissUpdate}
>
  <div class="content">
    <pre class="notes">{updater.available?.body || "No release notes."}</pre>
    {#if updater.progress}<div>{updater.progress}</div>{/if}
    {#if updater.error}<div class="err">{updater.error}</div>{/if}
  </div>
  {#snippet footer()}
    <Button disabled={busy} onclick={dismissUpdate}>Later</Button>
    <Button variant="primary" disabled={busy} onclick={install}>Install</Button>
  {/snippet}
</Dialog>

<style>
  .content {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    font-size: var(--text-md);
  }
  .notes {
    margin: 0;
    max-height: 240px;
    overflow: auto;
    white-space: pre-wrap;
    font-family: var(--font-ui);
  }
  .err {
    color: var(--danger);
  }
</style>
