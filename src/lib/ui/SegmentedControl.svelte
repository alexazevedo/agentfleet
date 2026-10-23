<script lang="ts">
  interface Props {
    options: { value: string; label: string }[];
    value: string;
    disabled?: boolean;
    onchange?: (v: string) => void;
  }

  let { options, value = $bindable(), disabled = false, onchange }: Props = $props();

  let buttons: HTMLButtonElement[] = $state([]);

  function select(i: number, focus = false) {
    const opt = options[i];
    if (!opt) return;
    if (opt.value !== value) {
      value = opt.value;
      onchange?.(value);
    }
    if (focus) buttons[i]?.focus();
  }

  function onkeydown(e: KeyboardEvent, i: number) {
    const n = options.length;
    let next: number;
    switch (e.key) {
      case "ArrowRight":
      case "ArrowDown":
        next = (i + 1) % n;
        break;
      case "ArrowLeft":
      case "ArrowUp":
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
    select(next, true);
  }

  const selectedIndex = $derived(options.findIndex((o) => o.value === value));
</script>

<div class="seg" role="radiogroup" class:disabled>
  {#each options as opt, i (opt.value)}
    <button
      bind:this={buttons[i]}
      type="button"
      role="radio"
      class="segment"
      class:selected={opt.value === value}
      aria-checked={opt.value === value}
      tabindex={i === (selectedIndex === -1 ? 0 : selectedIndex) ? 0 : -1}
      {disabled}
      onclick={() => select(i)}
      onkeydown={(e) => onkeydown(e, i)}
    >
      {opt.label}
    </button>
  {/each}
</div>

<style>
  .seg {
    display: inline-flex;
    background: var(--bg-hover);
    border-radius: var(--radius-md);
    padding: 2px;
    gap: 2px;
  }
  .segment {
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-family: inherit;
    font-size: var(--text-sm);
    padding: 2px var(--space-3);
    border-radius: var(--radius-sm);
    cursor: pointer;
  }
  .segment.selected {
    background: var(--bg-surface);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    color: var(--text);
  }
  .segment:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
  .segment:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
