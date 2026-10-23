<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLButtonAttributes, "children"> {
    variant?: "default" | "primary" | "danger" | "ghost";
    size?: "sm" | "md";
    children?: Snippet;
  }

  let {
    variant = "default",
    size = "md",
    disabled = false,
    type = "button",
    onclick,
    children,
    ...rest
  }: Props = $props();
</script>

<button class="btn {variant} {size}" {type} {disabled} {onclick} {...rest}>
  {@render children?.()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-1);
    height: 28px;
    padding: 0 var(--space-3);
    border-radius: var(--radius-md);
    font-size: var(--text-md);
    font-family: inherit;
    cursor: pointer;
    background: var(--bg-control);
    border: 1px solid var(--border-strong);
    color: var(--text);
  }
  .btn.sm {
    height: 22px;
    padding: 0 var(--space-2);
    font-size: var(--text-xs);
  }
  .btn:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
  .primary {
    background: var(--accent);
    color: var(--accent-text, var(--text-inverse));
    border-color: transparent;
  }
  .primary:hover:not(:disabled) {
    background: var(--accent-hover);
  }
  .danger {
    background: var(--danger);
    color: var(--accent-text, var(--text-inverse));
    border-color: transparent;
  }
  .danger:hover:not(:disabled) {
    background: var(--danger-hover);
  }
  .ghost {
    background: transparent;
    border-color: transparent;
    color: var(--text-muted);
  }
  .ghost:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text);
  }
  .btn:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
