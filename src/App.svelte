<script lang="ts">
  import Sidebar from "./lib/Sidebar.svelte";
  import StatusBar from "./lib/StatusBar.svelte";
  import NewTabDialog from "./lib/NewTabDialog.svelte";
  import Terminal from "./lib/Terminal.svelte";
  import { onMount } from "svelte";
  import { restoreTabs, store } from "./lib/sessions.svelte";

  const KEY = "agentfleet.sidebarCollapsed";
  let collapsed = $state(false);

  onMount(() => {
    try {
      collapsed = localStorage.getItem(KEY) === "1";
    } catch {}
    restoreTabs();
  });

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
    <button type="button" class="toggle" aria-label="Toggle sidebar" onclick={toggle}>☰</button>
    {#each store.tabs as tab (tab.key)}
      <Terminal {tab} active={tab.key === store.activeKey} />
    {/each}
    {#if store.tabs.length === 0}
      <div class="empty">Add a project, then press + to start a session</div>
    {/if}
  </section>
  <StatusBar />
  {#if store.dialogOpen}<NewTabDialog />{/if}
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
  .toggle {
    position: absolute;
    top: 4px;
    left: 4px;
    z-index: 5;
    background: transparent;
    border: none;
    color: #aaa;
    cursor: pointer;
    font-size: 16px;
  }
  .empty {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #888;
  }
</style>
