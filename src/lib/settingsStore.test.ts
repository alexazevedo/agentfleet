import { beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

import { invoke } from "@tauri-apps/api/core";
import { settings, updateSettings, type Settings } from "./settingsStore.svelte";

const mocked = vi.mocked(invoke);

function snap(): Settings {
  return JSON.parse(JSON.stringify(settings.value)) as Settings;
}

describe("updateSettings", () => {
  beforeEach(() => {
    mocked.mockReset();
  });

  it("resolves empty string and applies the returned settings", async () => {
    mocked.mockImplementation(async (_cmd, args) => (args as { settings: Settings }).settings);
    const result = await updateSettings({ terminal: { fontSize: 16 } });
    expect(result).toBe("");
    expect(settings.value.terminal.fontSize).toBe(16);
  });

  it("resolves the backend message and leaves settings untouched on rejection", async () => {
    const before = snap();
    mocked.mockRejectedValue("font size must be 9–24");
    const result = await updateSettings({ terminal: { fontSize: 30 } });
    expect(result).toBe("font size must be 9–24");
    expect(snap()).toEqual(before);
  });

  it("serialises overlapping calls so the second includes the first's change", async () => {
    mocked.mockImplementation(async (_cmd, args) => (args as { settings: Settings }).settings);
    const a = updateSettings({ terminal: { fontSize: 18 } });
    const b = updateSettings({ terminal: { lineHeight: 1.2 } });
    expect(await a).toBe("");
    expect(await b).toBe("");
    expect(mocked).toHaveBeenCalledTimes(2);
    const first = (mocked.mock.calls[0][1] as { settings: Settings }).settings;
    const second = (mocked.mock.calls[1][1] as { settings: Settings }).settings;
    expect(first.terminal.fontSize).toBe(18);
    expect(second.terminal.fontSize).toBe(18);
    expect(second.terminal.lineHeight).toBe(1.2);
  });
});
