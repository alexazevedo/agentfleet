<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import Button from "../ui/Button.svelte";
  import SegmentedControl from "../ui/SegmentedControl.svelte";
  import Switch from "../ui/Switch.svelte";
  import { settings, updateSettings, type Settings } from "../settingsStore.svelte";
  import Field from "./Field.svelte";

  let errors = $state<Record<string, string>>({});
  let rootDraft = $state(settings.value.worktreesRoot);

  async function apply(key: string, patch: Partial<Settings>) {
    errors[key] = await updateSettings(patch);
  }

  async function commitRoot(value: string = rootDraft) {
    const msg = await updateSettings({ worktreesRoot: value });
    errors.worktreesRoot = msg;
    if (msg) rootDraft = settings.value.worktreesRoot;
  }

  async function pick() {
    const picked = await open({ directory: true, multiple: false, defaultPath: rootDraft || undefined });
    if (typeof picked === "string") {
      rootDraft = picked;
      await commitRoot(picked);
    }
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Enter") (e.currentTarget as HTMLInputElement).blur();
  }
</script>

<Field label="Worktrees root" error={errors.worktreesRoot}>
  <div class="row">
    <input type="text" bind:value={rootDraft} onblur={() => commitRoot()} {onkeydown} />
    <Button variant="ghost" onclick={pick}>Browse…</Button>
  </div>
</Field>
<Field label="Default agent" error={errors.defaultAgent}>
  <SegmentedControl
    options={[
      { value: "claude", label: "Claude" },
      { value: "codex", label: "Codex" },
      { value: "shell", label: "Shell" },
    ]}
    bind:value={() => settings.value.defaultAgent, (v) => apply("defaultAgent", { defaultAgent: v as Settings["defaultAgent"] })}
  />
</Field>
<Field label="Theme" error={errors.theme}>
  <SegmentedControl
    options={[
      { value: "system", label: "System" },
      { value: "light", label: "Light" },
      { value: "dark", label: "Dark" },
    ]}
    bind:value={() => settings.value.theme, (v) => apply("theme", { theme: v as Settings["theme"] })}
  />
</Field>
<Field label="Claude auto-permission" error={errors.autoPermission}>
  <Switch bind:checked={() => settings.value.autoPermission, (v) => apply("autoPermission", { autoPermission: v })} />
</Field>
<Field label="Auto-deliver queued requests" error={errors.autoDelivery}>
  <Switch bind:checked={() => settings.value.autoDelivery, (v) => apply("autoDelivery", { autoDelivery: v })} />
</Field>
<Field label="Confirm before quitting" error={errors.confirmQuit}>
  <Switch bind:checked={() => settings.value.confirmQuit, (v) => apply("confirmQuit", { confirmQuit: v })} />
</Field>
<Field label="Restore sessions on launch" error={errors.restoreSessions}>
  <SegmentedControl
    options={[
      { value: "ask", label: "Ask" },
      { value: "always", label: "Always" },
      { value: "never", label: "Never" },
    ]}
    bind:value={() => settings.value.restoreSessions, (v) => apply("restoreSessions", { restoreSessions: v as Settings["restoreSessions"] })}
  />
</Field>

<style>
  .row {
    display: flex;
    gap: var(--space-2);
    align-items: center;
    width: 100%;
  }
  input {
    flex: 1;
    min-width: 0;
    background: var(--bg-control);
    color: var(--text);
    border: 1px solid var(--border-strong);
    border-radius: 4px;
    padding: 4px var(--space-2);
    font-size: var(--text-md);
  }
  input:focus-visible {
    outline: none;
    box-shadow: var(--focus-ring);
  }
</style>
