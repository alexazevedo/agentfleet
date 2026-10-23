<script lang="ts">
  import Sidebar from "./lib/Sidebar.svelte";
  import StatusBar from "./lib/StatusBar.svelte";
  import RemoveProjectDialog from "./lib/RemoveProjectDialog.svelte";
  import InitRepoDialog from "./lib/InitRepoDialog.svelte";
  import Terminal from "./lib/Terminal.svelte";
  import { NotebookPen, PanelLeft, Plus, Settings as SettingsIcon } from "lucide-svelte";
  import Messages from "./lib/Messages.svelte";
  import SettingsPane from "./lib/SettingsPane.svelte";
  import Scratchpad from "./lib/Scratchpad.svelte";
  import { queuedCount } from "./lib/collab.svelte";
  import { onMount } from "svelte";
  import { applyTheme, initTheme } from "./lib/theme.svelte";
  import { attachRestored, forgetRestorable, listRestorable, store } from "./lib/sessions.svelte";
  import { loadSettings, settings } from "./lib/settingsStore.svelte";
  import { startScheduler, stopScheduler, updater } from "./lib/updater.svelte";
  import UpdateDialog from "./lib/UpdateDialog.svelte";
  import RestoreDialog from "./lib/RestoreDialog.svelte";

  const KEY = "agentfleet.sidebarCollapsed";
  let collapsed = $state(false);

  onMount(() => initTheme());
  $effect(() => applyTheme(settings.value.theme));

  onMount(async () => {
    try {
      collapsed = localStorage.getItem(KEY) === "1";
    } catch {}
    await loadSettings().catch(() => {});
    const infos = await listRestorable();
    const mode = settings.value.restoreSessions;
    if (infos.length === 0) {
      store.restoreDecided = true;
    } else if (mode === "always") {
      attachRestored();
      store.restoreDecided = true;
    } else if (mode === "never") {
      await forgetRestorable();
      store.restoreDecided = true;
    } else {
      store.restoreDialogOpen = true; // dialog sets restoreDecided
    }
  });

  $effect(() => {
    if (store.restoreDecided && settings.loaded && settings.value.updateChecks) startScheduler();
    else stopScheduler();
    return stopScheduler;
  });

  const activeTab = $derived(store.tabs.find((t) => t.key === store.activeKey));

  function toggle() {
    collapsed = !collapsed;
    try {
      localStorage.setItem(KEY, collapsed ? "1" : "0");
    } catch {}
    // Terminal's ResizeObserver refits on layout change; nudge as a fallback.
    requestAnimationFrame(() => window.dispatchEvent(new Event("resize")));
  }
</script>

<main class="app" style:--sidebar-w={collapsed ? "0px" : "260px"}>
  <Sidebar />
  <section class="main-area">
    <button type="button" class="toggle" aria-label="Toggle sidebar" onclick={toggle}><PanelLeft size={16} strokeWidth={1.75} /></button>
    <div class="strip">
      {#if store.view === "scratchpad"}
        <span class="label"><NotebookPen size={14} strokeWidth={1.75} /> Scratchpad</span>
      {:else if store.view === "settings"}
        <span class="label"><SettingsIcon size={14} strokeWidth={1.75} /> Settings</span>
      {:else if activeTab?.info}
        {@const n = queuedCount(activeTab.info.id)}
        <button type="button" class:on={store.view === "terminal"} onclick={() => (store.view = "terminal")}>Terminal</button>
        <button type="button" class:on={store.view === "messages"} onclick={() => (store.view = "messages")}>
          Messages{n > 0 ? ` (${n})` : ""}
        </button>
      {/if}
    </div>
    <div class="pane">
      {#each store.tabs as tab (tab.key)}
        <Terminal {tab} active={tab.key === store.activeKey && store.view === "terminal"} />
      {/each}
      {#if store.view === "scratchpad" && store.activeKey?.startsWith("scratchpad:")}
        <Scratchpad projectId={store.activeKey.slice("scratchpad:".length)} />
      {:else if store.view === "settings"}
        <SettingsPane />
      {:else if store.view === "messages" && activeTab?.info}
        <Messages tab={activeTab} />
      {/if}
    </div>
    {#if store.tabs.length === 0}
      <div class="empty">
        <div class="empty-box"><Plus size={24} strokeWidth={1.75} /></div>
        <span>Add a project, then press + to start a session</span>
      </div>
    {/if}
  </section>
  <StatusBar />
  {#if store.restoreDialogOpen}<RestoreDialog />{/if}
  {#if store.removeProjectId}<RemoveProjectDialog />{/if}
  {#if store.initRepo}<InitRepoDialog />{/if}
  {#if updater.available}<UpdateDialog />{/if}
</main>

<style>
  .app {
    display: grid;
    grid-template-columns: var(--sidebar-w) 1fr;
    grid-template-rows: 1fr 24px;
    height: 100vh;
  }
  .main-area {
    position: relative;
    min-width: 0;
    min-height: 0;
  }
  .strip {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 28px;
    display: flex;
    align-items: center;
    gap: 4px;
    padding-left: 32px;
    background: var(--bg-strip);
    border-bottom: 1px solid var(--border);
    font-family: var(--font-ui);
  }
  .strip button {
    background: transparent;
    border: none;
    border-radius: var(--radius-sm);
    height: 26px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0 10px;
    font-size: var(--text-sm);
  }
  .strip button:hover {
    color: var(--text);
  }
  .strip button.on {
    color: var(--text);
    background: var(--bg-active);
  }
  .strip .label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--text);
    font-size: var(--text-sm);
    padding: 4px 10px;
  }
  .pane {
    position: absolute;
    inset: 28px 0 0 0;
  }
  .toggle {
    position: absolute;
    top: 4px;
    left: 4px;
    z-index: 5;
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
  }
  .toggle:hover {
    color: var(--text);
  }
  .empty {
    position: absolute;
    inset: 28px 0 0 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-family: var(--font-ui);
    font-size: var(--text-md);
  }
  .empty-box {
    display: flex;
    padding: var(--space-4);
    border: 1px dashed var(--border-strong);
    border-radius: var(--radius-lg);
  }
</style>
