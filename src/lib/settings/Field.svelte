<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    label: string;
    hint?: string;
    error?: string;
    children: Snippet;
  }

  let { label, hint, error, children }: Props = $props();
</script>

<div class="field">
  <div class="label">{label}</div>
  <div class="control">
    {@render children()}
    {#if hint}<div class="hint">{hint}</div>{/if}
    {#if error}<div class="error" role="alert">{error}</div>{/if}
  </div>
</div>

<style>
  .field {
    display: flex;
    gap: var(--space-3);
    align-items: flex-start;
    padding: var(--space-2) 0;
  }
  .label {
    flex: 0 0 160px;
    color: var(--text);
    font-size: var(--text-md);
    padding-top: 4px;
  }
  .control {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    align-items: flex-start;
  }
  .hint {
    color: var(--text-muted);
    font-size: var(--text-xs);
  }
  .error {
    color: var(--danger);
    font-size: var(--text-xs);
  }
</style>
