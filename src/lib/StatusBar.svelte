<script lang="ts">
  import AgentIcon from "./AgentIcon.svelte";
  import { store, tabTitle } from "./sessions.svelte";

  const active = $derived(store.tabs.find((t) => t.key === store.activeKey));
  const left = $derived.by(() => {
    const t = active;
    if (!t?.info) return null;
    const proj = store.projects.find((p) => p.id === t.projectId)?.name ?? t.info.repoName;
    return { agent: t.info.agent, project: proj, rest: ` · ${tabTitle(t.info)} ${t.info.id8} · ${t.status ?? "unknown"}` };
  });
  const running = $derived(store.tabs.filter((t) => t.info && t.status !== "ended").length);
</script>

<footer class="status-bar">
  <span>
    {#if left}
      <AgentIcon agent={left.agent} size={12} /> <strong>{left.project}</strong>{left.rest}
    {:else}
      No session
    {/if}
  </span>
  <span>{running} running {running === 1 ? "session" : "sessions"}</span>
</footer>

<style>
  .status-bar {
    grid-column: 1 / -1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
    font-size: var(--text-xs);
    font-family: var(--font-ui);
    background: var(--bg-strip);
    color: var(--text-muted);
    border-top: 1px solid var(--border);
  }
  strong {
    color: var(--text);
    font-weight: 500;
  }
</style>
