<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    label?: Snippet;
    onchange?: (v: boolean) => void;
  }

  let { checked = $bindable(false), disabled = false, label, onchange }: Props = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<label class="switch-wrap" class:disabled>
  <!-- svelte-ignore a11y_consider_explicit_label -->
  <button
    type="button"
    role="switch"
    class="switch"
    class:on={checked}
    aria-checked={checked}
    {disabled}
    onclick={toggle}
  >
    <span class="knob"></span>
  </button>
  {#if label}
    <span class="text">{@render label()}</span>
  {/if}
</label>

<style>
  .switch-wrap {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-md);
    color: var(--text);
    cursor: pointer;
  }
  .switch-wrap.disabled {
    opacity: 0.5;
    cursor: default;
  }
  .switch {
    position: relative;
    width: 32px;
    height: 18px;
    padding: 0;
    border: none;
    border-radius: 9px;
    background: var(--neutral);
    cursor: inherit;
    transition: background 120ms;
    flex-shrink: 0;
  }
  .switch.on {
    background: var(--accent);
  }
  .switch:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
  .knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--text-inverse);
    transition: transform 120ms;
  }
  .switch.on .knob {
    transform: translateX(14px);
  }
</style>
