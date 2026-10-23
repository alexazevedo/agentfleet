<script lang="ts">
  import { onMount } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import { Channel, invoke } from "@tauri-apps/api/core";
  import "@xterm/xterm/css/xterm.css";
  import { fail, resolve, type SessionInfo, type Tab } from "./sessions.svelte";
  import { settings } from "./settingsStore.svelte";

  let { tab, active }: { tab: Tab; active: boolean } = $props();

  let container: HTMLDivElement;
  let term: Terminal;
  let fit: FitAddon;
  let id: string | null = null;

  function refit() {
    if (!container || container.offsetParent === null) return; // hidden
    fit.fit();
    if (id) invoke("resize_session", { id, cols: term.cols, rows: term.rows }).catch(() => {});
  }

  function family(f: string) {
    return f ? `${f}, ui-monospace, Menlo, monospace` : "ui-monospace, Menlo, monospace";
  }

  let opened = false;
  $effect(() => {
    const t = settings.value.terminal;
    const o = [family(t.fontFamily), t.fontSize, t.lineHeight, t.cursorBlink] as const;
    if (!opened) return; // initial values applied at creation
    term.options.fontFamily = o[0];
    term.options.fontSize = o[1];
    term.options.lineHeight = o[2];
    term.options.cursorBlink = o[3];
    refit();
  });

  $effect(() => {
    if (active && term) {
      refit();
      term.focus();
    }
  });

  onMount(() => {
    const t = settings.value.terminal;
    term = new Terminal({
      cursorBlink: t.cursorBlink,
      fontSize: t.fontSize,
      lineHeight: t.lineHeight,
      fontFamily: family(t.fontFamily),
    });
    fit = new FitAddon();
    term.loadAddon(fit);
    term.open(container);
    fit.fit();
    opened = true;

    term.onData((d) => {
      if (id) invoke("write_session", { id, data: Array.from(new TextEncoder().encode(d)) });
    });

    const ro = new ResizeObserver(refit);
    ro.observe(container);

    const channel = new Channel<ArrayBuffer>();
    channel.onmessage = (buf) => term.write(new Uint8Array(buf));
    if (tab.info) {
      const restoredId = tab.info.id;
      invoke("attach_session", {
        id: restoredId,
        resume: tab.resume ?? true,
        cols: term.cols,
        rows: term.rows,
        onData: channel,
      })
        .then(() => {
          id = restoredId;
          refit();
        })
        .catch((e) => fail(tab.key, String(e)));
    } else {
      invoke<SessionInfo>("create_session", {
        projectId: tab.projectId,
        agent: tab.agent,
        cols: term.cols,
        rows: term.rows,
        onData: channel,
      })
        .then((info) => {
          id = info.id;
          resolve(tab.key, info);
          refit();
        })
        .catch((e) => fail(tab.key, String(e)));
    }

    return () => {
      ro.disconnect();
      term.dispose();
    };
  });
</script>

<div class="term" bind:this={container} style:display={active ? "block" : "none"}></div>

<style>
  .term {
    position: absolute;
    inset: 0;
  }
</style>
