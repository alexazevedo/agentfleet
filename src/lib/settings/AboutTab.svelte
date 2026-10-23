<script lang="ts">
  import { onMount } from "svelte";
  import { appInfo, type AppInfo } from "../settingsStore.svelte";
  import Field from "./Field.svelte";

  let info = $state<AppInfo | null>(null);
  let error = $state("");

  onMount(async () => {
    try {
      info = await appInfo();
    } catch (e) {
      error = String(e);
    }
  });
</script>

<Field label="App" {error}>
  <span class="mono">agentfleet</span>
</Field>
<Field label="Version">
  <span class="mono">{info?.version ?? "…"}</span>
</Field>
<Field label="Commit">
  <span class="mono">{info?.commit ?? "…"}</span>
</Field>

<style>
  .mono {
    font-family: var(--font-mono);
    color: var(--text-muted);
    font-size: var(--text-md);
    padding-top: 4px;
  }
</style>
