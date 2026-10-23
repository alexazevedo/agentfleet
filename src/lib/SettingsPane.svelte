<script lang="ts">
  import { onMount } from "svelte";
  import Tabs from "./ui/Tabs.svelte";
  import { loadSettings, settings } from "./settingsStore.svelte";
  import GeneralTab from "./settings/GeneralTab.svelte";
  import TerminalTab from "./settings/TerminalTab.svelte";
  import UpdatesTab from "./settings/UpdatesTab.svelte";
  import AboutTab from "./settings/AboutTab.svelte";

  const STORAGE_KEY = "agentfleet.settingsTab";
  const tabs = [
    { id: "general", label: "General" },
    { id: "terminal", label: "Terminal" },
    { id: "updates", label: "Updates" },
    { id: "about", label: "About" },
  ];

  function initialTab(): string {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored && tabs.some((t) => t.id === stored)) return stored;
    } catch {
      // storage unavailable; fall back to the first tab
    }
    return tabs[0].id;
  }

  let active = $state(initialTab());
  let ready = $state(settings.loaded);
  let error = $state("");

  $effect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, active);
    } catch {
      // storage unavailable; the tab simply is not remembered
    }
  });

  onMount(async () => {
    if (settings.loaded) return;
    try {
      await loadSettings();
      ready = true;
    } catch (e) {
      error = String(e);
    }
  });
</script>

<div class="settings">
  <Tabs {tabs} bind:active>
    {#snippet children(current)}
      <div class="panel">
        {#if error}
          <p class="err" role="alert">{error}</p>
        {:else if ready}
          {#if current === "general"}
            <GeneralTab />
          {:else if current === "terminal"}
            <TerminalTab />
          {:else if current === "updates"}
            <UpdatesTab />
          {:else}
            <AboutTab />
          {/if}
        {/if}
      </div>
    {/snippet}
  </Tabs>
</div>

<style>
  .settings {
    position: absolute;
    inset: 0;
    overflow-y: auto;
    background: var(--bg);
  }
  .panel {
    padding: var(--space-4);
    max-width: 560px;
  }
  .err {
    color: var(--danger);
    margin: 0;
  }
</style>
