<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { checkForUpdate, updater } from "./updater.svelte";
  import { appInfo, loadSettings, saveSettings, settings, type AppInfo, type Settings } from "./settingsStore.svelte";

  let form = $state<Settings>({ ...settings.value });
  let info = $state<AppInfo | null>(null);
  let error = $state("");
  let saved = $state(false);

  onMount(async () => {
    try {
      await loadSettings();
      form = { ...settings.value };
      info = await appInfo();
    } catch (e) {
      error = String(e);
    }
  });

  async function pick() {
    const picked = await open({ directory: true, multiple: false, defaultPath: form.worktreesRoot || undefined });
    if (typeof picked === "string") form.worktreesRoot = picked;
  }

  async function save() {
    saved = false;
    try {
      await saveSettings({ ...form });
      form = { ...settings.value };
      error = "";
      saved = true;
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="settings">
  <label class="field">
    <span>Worktrees root</span>
    <div class="row">
      <input type="text" bind:value={form.worktreesRoot} />
      <button type="button" onclick={pick}>Browse…</button>
    </div>
  </label>
  <label class="field">
    <span>Default agent</span>
    <select bind:value={form.defaultAgent}>
      <option value="claude">Claude Code</option>
      <option value="shell">Shell</option>
    </select>
  </label>
  <label><input type="checkbox" bind:checked={form.autoPermission} /> Claude auto-permission mode</label>
  <label><input type="checkbox" bind:checked={form.autoDelivery} /> Auto-deliver queued requests</label>
  <label><input type="checkbox" bind:checked={form.confirmQuit} /> Confirm before quitting</label>
  <div class="row">
    <label><input type="checkbox" bind:checked={form.updateChecks} /> Check for updates</label>
    <button type="button" disabled={updater.checking} onclick={() => checkForUpdate(true)}>
      {updater.checking ? "Checking…" : "Check now"}
    </button>
    {#if updater.error}<span class="err">{updater.error}</span>{:else if updater.lastResult}<span>{updater.lastResult}</span>{/if}
  </div>
  <div class="row">
    <button type="button" onclick={save}>Save</button>
    {#if saved}<span class="ok">Saved</span>{/if}
  </div>
  {#if error}<p class="err">{error}</p>{/if}
  <div class="about">
    <h4>About</h4>
    <div>Version {info?.version ?? "…"}</div>
    <div>Commit {info?.commit ?? "…"}</div>
  </div>
</div>

<style>
  .settings {
    position: absolute;
    inset: 0;
    overflow-y: auto;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    background: #1e1e1e;
    font-size: 13px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-width: 480px;
  }
  .row {
    display: flex;
    gap: 8px;
    align-items: center;
  }
  input[type="text"] {
    flex: 1;
  }
  button {
    background: #333;
    border: 1px solid #444;
    color: inherit;
    border-radius: 3px;
    cursor: pointer;
    padding: 2px 8px;
  }
  .err {
    color: #f85149;
    margin: 0;
  }
  .ok {
    color: #3fb950;
  }
  .about {
    margin-top: 16px;
    color: #aaa;
    font-family: ui-monospace, monospace;
  }
  .about h4 {
    margin: 0 0 4px;
    font-family: inherit;
  }
</style>
