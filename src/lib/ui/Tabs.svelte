<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    tabs: { id: string; label: string }[];
    active: string;
    children: Snippet<[string]>;
  }

  let { tabs, active = $bindable(), children }: Props = $props();

  let buttons: HTMLButtonElement[] = $state([]);

  function onkeydown(e: KeyboardEvent, i: number) {
    const n = tabs.length;
    let next: number;
    switch (e.key) {
      case "ArrowRight":
        next = (i + 1) % n;
        break;
      case "ArrowLeft":
        next = (i - 1 + n) % n;
        break;
      case "Home":
        next = 0;
        break;
      case "End":
        next = n - 1;
        break;
      default:
        return;
    }
    e.preventDefault();
    active = tabs[next].id;
    buttons[next]?.focus();
  }
</script>

<div class="tablist" role="tablist">
  {#each tabs as tab, i (tab.id)}
    <button
      bind:this={buttons[i]}
      type="button"
      role="tab"
      class="tab"
      class:selected={tab.id === active}
      id="tab-{tab.id}"
      aria-selected={tab.id === active}
      aria-controls="panel-{tab.id}"
      tabindex={tab.id === active ? 0 : -1}
      onclick={() => (active = tab.id)}
      onkeydown={(e) => onkeydown(e, i)}
    >
      {tab.label}
    </button>
  {/each}
</div>
<div class="panel" role="tabpanel" id="panel-{active}" aria-labelledby="tab-{active}">
  {@render children(active)}
</div>

<style>
  .tablist {
    display: flex;
    justify-content: center;
    border-bottom: 1px solid var(--border);
  }
  .tab {
    height: 28px;
    padding: 0 var(--space-3);
    border: none;
    border-bottom: 2px solid transparent;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: var(--text-md);
    cursor: pointer;
  }
  .tab.selected {
    color: var(--text);
    border-bottom-color: var(--accent);
  }
  .tab:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
</style>
