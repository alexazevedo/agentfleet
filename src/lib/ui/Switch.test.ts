import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import SwitchHost from "./__fixtures__/SwitchHost.svelte";

describe("Switch", () => {
  it("click flips aria-checked and calls onchange; Space flips back", async () => {
    const user = userEvent.setup();
    const onchange = vi.fn();
    render(SwitchHost, { onchange });
    const sw = screen.getByRole("switch");
    expect(sw).toHaveAttribute("aria-checked", "false");

    await user.click(sw);
    expect(sw).toHaveAttribute("aria-checked", "true");
    expect(onchange).toHaveBeenCalledWith(true);

    await user.keyboard(" ");
    expect(sw).toHaveAttribute("aria-checked", "false");
    expect(onchange).toHaveBeenLastCalledWith(false);
  });
});
