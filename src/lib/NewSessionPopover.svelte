<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import Popover from "./ui/Popover.svelte";
  import SegmentedControl from "./ui/SegmentedControl.svelte";
  import Button from "./ui/Button.svelte";
  import { loadSettings, settings } from "./settingsStore.svelte";
  import { addPending, addProjectOrOfferInit, store, type AgentKind } from "./sessions.svelte";

  let projectId = $state("");
  let agent = $state<AgentKind>("claude");

  const target = $derived(store.newSession);
  const fixedProject = $derived(
    target?.projectId ? store.projects.find((p) => p.id === target.projectId) : undefined,
  );

  // Re-initialise the form each time the popover opens.
  $effect(() => {
    if (!target) return;
    const preset = target.projectId;
    (async () => {
      try {
        if (!settings.loaded) await loadSettings();
      } catch {}
      agent = settings.value.defaultAgent;
    })();
    if (preset) {
      projectId = preset;
      return;
    }
    const last = store.tabs.at(-1)?.projectId;
    projectId = store.projects.find((p) => p.id === last)?.id ?? store.projects[0]?.id ?? "";
  });

  function close() {
    store.newSession = null;
  }

  async function add() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked !== "string") return;
    const anchor = target?.anchor ?? null;
    try {
      store.error = "";
      const p = await addProjectOrOfferInit(picked, (made) => {
        // Clicking the init dialog closed this popover: reopen it, on the new project if any.
        if (anchor) store.newSession = { anchor, projectId: made?.id ?? null };
      });
      if (p) projectId = p.id;
    } catch (e) {
      store.error = String(e);
    }
  }

  function create() {
    if (!projectId) {
      store.error = "choose or add a project";
      return;
    }
    addPending(projectId, agent);
    store.error = "";
    close();
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key !== "Enter") return;
    const el = e.target as HTMLElement;
    // Buttons activate natively; a <select> uses Enter to confirm its choice.
    if (el.tagName === "BUTTON" || el.tagName === "SELECT") return;
    e.preventDefault();
    create();
  }
</script>

<Popover anchor={target?.anchor ?? null} open={target !== null} onclose={close}>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="panel" {onkeydown}>
    {#if target?.projectId === null}
      <div class="row">
        <select aria-label="Project" bind:value={projectId}>
          {#if store.projects.length === 0}<option value="">no projects</option>{/if}
          {#each store.projects as p (p.id)}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
        <Button variant="ghost" size="sm" onclick={add}>Add project…</Button>
      </div>
    {:else}
      <div class="caption">{fixedProject?.name ?? ""}</div>
    {/if}
    <SegmentedControl
      options={[
        { value: "claude", label: "Claude Code" },
        { value: "codex", label: "Codex" },
        { value: "pi", label: "Pi" },
        { value: "opencode", label: "opencode" },
        { value: "shell", label: "Shell" },
      ]}
      bind:value={agent}
    />
    {#if store.error}<p class="error">{store.error}</p>{/if}
    <div class="footer">
      <!-- svelte-ignore a11y_autofocus -->
      <Button variant="primary" onclick={create} autofocus>Create</Button>
    </div>
  </div>
</Popover>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    min-width: 240px;
    padding: var(--space-2);
  }
  .row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  select {
    flex: 1;
    min-width: 0;
  }
  .caption {
    color: var(--text-muted);
    font-size: var(--text-sm);
  }
  .error {
    margin: 0;
    color: var(--danger);
    font-size: var(--text-sm);
  }
  .footer {
    display: flex;
    justify-content: flex-end;
  }
</style>
