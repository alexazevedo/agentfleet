<script lang="ts">
  import { labelOf, type AgentKind } from "./sessions.svelte";

  let { agent, size = 14 }: { agent: AgentKind; size?: number } = $props();
  const label = $derived(labelOf(agent));
</script>

<svg
  viewBox="0 0 16 16"
  width={size}
  height={size}
  role="img"
  aria-label={label}
  style="vertical-align: middle; flex: none"
>
  {#if agent === "claude"}
    {#each [0, 45, 90, 135, 180, 225, 270, 315] as a}
      <line
        x1="8"
        y1="2"
        x2="8"
        y2="6"
        stroke="var(--brand-claude)"
        stroke-width="1.6"
        stroke-linecap="round"
        transform="rotate({a} 8 8)"
      />
    {/each}
  {:else if agent === "codex"}
    <rect x="1" y="1" width="14" height="14" rx="3" fill="var(--brand-codex-bg)" />
    <path
      d="M4.5 5.5 L7 8 L4.5 10.5 M8.5 11 H11.5"
      fill="none"
      stroke="var(--brand-codex-fg)"
      stroke-width="1.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {:else if agent === "pi"}
    <path
      d="M3.5 4.5 H12.5 M6 4.5 V11.5 M10 4.5 V11.5"
      fill="none"
      stroke="var(--brand-pi)"
      stroke-width="1.6"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {:else if agent === "opencode"}
    <rect x="1" y="1" width="14" height="14" rx="3" fill="var(--brand-opencode-bg)" />
    <rect
      x="4.5"
      y="4.5"
      width="4.5"
      height="7"
      fill="none"
      stroke="var(--brand-opencode-fg)"
      stroke-width="1.4"
    />
    <rect x="10.5" y="8.5" width="2" height="3" fill="var(--brand-opencode-fg)" />
  {:else}
    <rect x="1" y="1" width="14" height="14" rx="3" fill="var(--brand-shell-bg)" stroke="var(--brand-shell-border)" />
    <path
      d="M5 5 L9 8 L5 11"
      fill="none"
      stroke="var(--brand-shell-fg)"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {/if}
</svg>
