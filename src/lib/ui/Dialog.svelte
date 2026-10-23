<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    open: boolean;
    title: string;
    width?: number;
    children: Snippet;
    footer?: Snippet;
    dismissible?: boolean;
    onclose?: () => void;
  }

  let {
    open = $bindable(false),
    title,
    width = 420,
    children,
    footer,
    dismissible = true,
    onclose,
  }: Props = $props();

  let el: HTMLDialogElement | undefined = $state();

  $effect(() => {
    if (!el) return;
    if (open) {
      if (!el.open && typeof el.showModal === "function") el.showModal();
    } else if (el.open && typeof el.close === "function") {
      el.close();
    }
  });

  function oncancel(e: Event) {
    if (dismissible) open = false;
    else e.preventDefault();
  }

  function onclick(e: MouseEvent) {
    if (dismissible && e.target === el) open = false;
  }

  function onnativeclose() {
    open = false;
    onclose?.();
  }
</script>

<dialog
  bind:this={el}
  class="dialog"
  style="width: {width}px;"
  aria-labelledby="dialog-title"
  {oncancel}
  {onclick}
  onclose={onnativeclose}
>
  <h2 id="dialog-title">{title}</h2>
  <div class="body">{@render children()}</div>
  {#if footer}
    <div class="footer">{@render footer()}</div>
  {/if}
</dialog>

<style>
  .dialog {
    max-width: calc(100vw - 2 * var(--space-4));
    padding: 0;
    background: var(--bg-surface);
    color: var(--text);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-popover);
  }
  .dialog::backdrop {
    background: var(--overlay);
  }
  h2 {
    margin: 0;
    padding: var(--space-4) var(--space-4) 0;
    font-size: var(--text-lg);
    font-weight: 600;
  }
  .body {
    padding: var(--space-3) var(--space-4);
  }
  .footer {
    display: flex;
    gap: var(--space-2);
    justify-content: flex-end;
    padding: 0 var(--space-4) var(--space-4);
  }
</style>
