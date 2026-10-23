<script lang="ts">
  import Switch from "../ui/Switch.svelte";
  import { settings, updateSettings } from "../settingsStore.svelte";
  import Field from "./Field.svelte";

  let errors = $state<Record<string, string>>({});
  let family = $state(settings.value.terminal.fontFamily);
  let size = $state(String(settings.value.terminal.fontSize));
  let height = $state(String(settings.value.terminal.lineHeight));

  function reset() {
    family = settings.value.terminal.fontFamily;
    size = String(settings.value.terminal.fontSize);
    height = String(settings.value.terminal.lineHeight);
  }

  async function commitFamily() {
    const msg = await updateSettings({ terminal: { fontFamily: family } });
    errors.fontFamily = msg;
    if (msg) reset();
  }

  async function commitNumber(key: "fontSize" | "lineHeight", raw: string) {
    const n = Number(raw);
    if (raw.trim() === "" || Number.isNaN(n)) {
      errors[key] = "enter a number";
      return;
    }
    const msg = await updateSettings({ terminal: { [key]: n } });
    errors[key] = msg;
    if (msg) reset();
  }

  async function commitBlink(v: boolean) {
    errors.cursorBlink = await updateSettings({ terminal: { cursorBlink: v } });
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter") (e.currentTarget as HTMLInputElement).blur();
  }
</script>

<Field label="Font family" error={errors.fontFamily}>
  <input type="text" placeholder="system monospace" bind:value={family} onblur={commitFamily} {onkeydown} />
</Field>
<Field label="Font size" hint="Applies to open terminals immediately." error={errors.fontSize}>
  <input
    type="number"
    min="9"
    max="24"
    step="1"
    bind:value={size}
    onblur={() => commitNumber("fontSize", size)}
    {onkeydown}
  />
</Field>
<Field label="Line height" error={errors.lineHeight}>
  <input
    type="number"
    min="1.0"
    max="1.6"
    step="0.05"
    bind:value={height}
    onblur={() => commitNumber("lineHeight", height)}
    {onkeydown}
  />
</Field>
<Field label="Cursor blink" error={errors.cursorBlink}>
  <Switch bind:checked={() => settings.value.terminal.cursorBlink, (v) => commitBlink(v)} />
</Field>

<style>
  input {
    width: 100%;
    background: var(--bg-control);
    color: var(--text);
    border: 1px solid var(--border-strong);
    border-radius: 4px;
    padding: 4px var(--space-2);
    font-size: var(--text-md);
  }
  input[type="number"] {
    width: 96px;
  }
  input:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
</style>
