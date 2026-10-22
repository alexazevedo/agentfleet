<script lang="ts">
  import { dismissUpdate, installUpdate, updater } from "./updater.svelte";
  let busy = $state(false);

  async function install() {
    busy = true;
    await installUpdate();
    busy = false;
  }
</script>

<div class="overlay">
  <div class="dialog" role="dialog" aria-modal="true">
    <h3>Update available: {updater.available?.version}</h3>
    <pre class="notes">{updater.available?.body || "No release notes."}</pre>
    {#if updater.progress}<div>{updater.progress}</div>{/if}
    {#if updater.error}<div class="err">{updater.error}</div>{/if}
    <div class="row">
      <button type="button" disabled={busy} onclick={install}>Install</button>
      <button type="button" disabled={busy} onclick={dismissUpdate}>Later</button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .dialog {
    background: #252526;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 16px;
    width: 440px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    font-size: 13px;
  }
  h3 {
    margin: 0;
  }
  .notes {
    margin: 0;
    max-height: 240px;
    overflow: auto;
    white-space: pre-wrap;
    font-family: inherit;
  }
  .row {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }
  button {
    background: #333;
    border: 1px solid #444;
    color: inherit;
    border-radius: 3px;
    cursor: pointer;
    padding: 4px 12px;
  }
  .err {
    color: #f85149;
  }
</style>
