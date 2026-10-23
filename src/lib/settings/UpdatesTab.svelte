<script lang="ts">
  import Button from "../ui/Button.svelte";
  import Switch from "../ui/Switch.svelte";
  import { settings, updateSettings } from "../settingsStore.svelte";
  import { checkForUpdate, updater } from "../updater.svelte";
  import Field from "./Field.svelte";

  let error = $state("");

  async function commit(v: boolean) {
    error = await updateSettings({ updateChecks: v });
  }
</script>

<Field label="Automatic checks" error={error}>
  <Switch bind:checked={() => settings.value.updateChecks, (v) => commit(v)}>
    {#snippet label()}Check for updates automatically{/snippet}
  </Switch>
</Field>
<Field label="Manual check">
  <div class="row">
    <Button disabled={updater.checking} onclick={() => checkForUpdate(true)}>
      {updater.checking ? "Checking…" : "Check now"}
    </Button>
    {#if updater.error}
      <span class="err">{updater.error}</span>
    {:else if updater.lastResult}
      <span class="result">{updater.lastResult}</span>
    {/if}
  </div>
</Field>

<style>
  .row {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }
  .err {
    color: var(--danger);
    font-size: var(--text-md);
  }
  .result {
    color: var(--text-muted);
    font-size: var(--text-md);
  }
</style>
