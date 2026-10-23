import { beforeEach, describe, expect, it, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { tick } from "svelte";
import Scratchpad from "./Scratchpad.svelte";
import { loadScratchpad, saveScratchpad } from "./scratchpadStore.svelte";

vi.mock("./scratchpadStore.svelte", () => ({
  loadScratchpad: vi.fn(),
  saveScratchpad: vi.fn(),
  initScratchpadListener: vi.fn(),
  onScratchpadChanged: vi.fn(() => () => {}),
}));

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((done) => (resolve = done));
  return { promise, resolve };
}

beforeEach(() => vi.clearAllMocks());

describe("scratchpad async edits", () => {
  it("preserves typing while the initial read is in flight", async () => {
    const pending = deferred<{ text: string; revision: number }>();
    vi.mocked(loadScratchpad).mockReturnValueOnce(pending.promise);
    render(Scratchpad, { projectId: "a" });
    await userEvent.setup().type(screen.getByRole("textbox"), "my edits");
    pending.resolve({ text: "remote", revision: 1 });
    await pending.promise;
    await tick();
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("my edits"));
    expect(screen.getByText(/edited/)).toBeInTheDocument();
  });

  it("ignores a previous project's late read", async () => {
    const old = deferred<{ text: string; revision: number }>();
    vi.mocked(loadScratchpad).mockReturnValueOnce(old.promise)
      .mockResolvedValueOnce({ text: "project b", revision: 2 });
    const view = render(Scratchpad, { projectId: "a" });
    await waitFor(() => expect(loadScratchpad).toHaveBeenCalledWith("a"));
    await view.rerender({ projectId: "b" });
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("project b"));
    old.resolve({ text: "project a", revision: 1 });
    await old.promise;
    await tick();
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("project b"));
  });

  it("keeps newer edits dirty after save and uses the acknowledged revision", async () => {
    const pending = deferred<number>();
    vi.mocked(loadScratchpad).mockResolvedValue({ text: "a", revision: 1 });
    vi.mocked(saveScratchpad).mockReturnValueOnce(pending.promise).mockResolvedValueOnce(3);
    render(Scratchpad, { projectId: "a" });
    const user = userEvent.setup();
    const input = screen.getByRole("textbox");
    await waitFor(() => expect(input).toHaveValue("a"));
    await user.type(input, "b");
    await user.click(screen.getByRole("button", { name: "Save" }));
    await user.type(input, "c");
    pending.resolve(2);
    await waitFor(() => expect(screen.getByText("rev 2 (edited)")).toBeInTheDocument());
    await user.click(screen.getByRole("button", { name: "Save" }));
    expect(saveScratchpad).toHaveBeenLastCalledWith("a", "abc", 2);
  });

  it("ignores an old save after switching away and back to the same project", async () => {
    const pending = deferred<number>();
    vi.mocked(loadScratchpad).mockResolvedValueOnce({ text: "a", revision: 1 })
      .mockResolvedValueOnce({ text: "b", revision: 1 })
      .mockResolvedValueOnce({ text: "latest a", revision: 5 });
    vi.mocked(saveScratchpad).mockReturnValueOnce(pending.promise);
    const view = render(Scratchpad, { projectId: "a" });
    const user = userEvent.setup();
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("a"));
    await user.type(screen.getByRole("textbox"), " edit");
    await user.click(screen.getByRole("button", { name: "Save" }));
    await view.rerender({ projectId: "b" });
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("b"));
    await view.rerender({ projectId: "a" });
    await waitFor(() => expect(screen.getByRole("textbox")).toHaveValue("latest a"));
    pending.resolve(2);
    await pending.promise;
    await tick();
    await waitFor(() => expect(screen.getByText("rev 5")).toBeInTheDocument());
  });
});
