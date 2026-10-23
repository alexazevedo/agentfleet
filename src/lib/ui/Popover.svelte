<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    anchor: HTMLElement | null;
    open: boolean;
    placement?: "bottom-start" | "bottom-end";
    children: Snippet;
    onclose?: () => void;
  }

  let { anchor, open = $bindable(false), placement = "bottom-start", children, onclose }: Props =
    $props();

  let panel: HTMLDivElement | undefined = $state();
  let top = $state(0);
  let left = $state(0);
  let flipped = $state(false);
  let arrowLeft = $state(16);
  let shown = $state(false);

  function close() {
    if (!open) return;
    open = false;
    anchor?.focus();
    onclose?.();
  }

  function position() {
    if (!panel || !anchor) return;
    const rect = anchor.getBoundingClientRect();
    const { width, height } = panel.getBoundingClientRect();
    let t = rect.bottom + 8;
    flipped = false;
    if (t + height > window.innerHeight - 8) {
      t = rect.top - 8 - height;
      flipped = true;
    }
    let l = placement === "bottom-end" ? rect.right - width : rect.left;
    l = Math.max(8, Math.min(l, window.innerWidth - width - 8));
    top = t;
    left = l;
    const center = rect.left + rect.width / 2 - l;
    arrowLeft = Math.max(12, Math.min(center, Math.max(12, width - 12)));
  }

  $effect(() => {
    if (!open || !anchor || !panel) return;
    const el = panel;
    const raf = requestAnimationFrame(() => {
      position();
      shown = true;
    });
    const focusable = el.querySelector<HTMLElement>(
      "[autofocus], button, input, select, [tabindex]",
    );
    (focusable ?? el).focus();

    const onReposition = () => position();
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.stopPropagation();
        close();
      }
    };
    const onDown = (e: MouseEvent) => {
      const target = e.target as Node | null;
      if (target && (el.contains(target) || anchor?.contains(target))) return;
      close();
    };
    window.addEventListener("resize", onReposition);
    window.addEventListener("scroll", onReposition, true);
    document.addEventListener("keydown", onKey);
    document.addEventListener("mousedown", onDown);
    return () => {
      cancelAnimationFrame(raf);
      shown = false;
      window.removeEventListener("resize", onReposition);
      window.removeEventListener("scroll", onReposition, true);
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("mousedown", onDown);
    };
  });
</script>

{#if open && anchor}
  <div
    bind:this={panel}
    class="popover"
    class:shown
    class:flipped
    role="dialog"
    tabindex="-1"
    style="top: {top}px; left: {left}px; --arrow-left: {arrowLeft}px;"
  >
    {@render children()}
  </div>
{/if}

<style>
  .popover {
    position: fixed;
    z-index: 50;
    min-width: 240px;
    padding: var(--space-3);
    background: var(--bg-surface);
    color: var(--text);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-popover);
    opacity: 0;
    transform: translateY(4px);
    transition:
      opacity 120ms,
      transform 120ms;
  }
  .popover.shown {
    opacity: 1;
    transform: translateY(0);
  }
  .popover:focus {
    outline: none;
  }
  .popover::before {
    content: "";
    position: absolute;
    top: -5px;
    left: calc(var(--arrow-left) - 4px);
    width: 8px;
    height: 8px;
    background: var(--bg-surface);
    border-left: 1px solid var(--border-strong);
    border-top: 1px solid var(--border-strong);
    transform: rotate(45deg);
  }
  .popover.flipped::before {
    top: auto;
    bottom: -5px;
    border-left: none;
    border-top: none;
    border-right: 1px solid var(--border-strong);
    border-bottom: 1px solid var(--border-strong);
  }
</style>
