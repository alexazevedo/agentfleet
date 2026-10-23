import { describe, it, expect } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import PopoverHost from "./__fixtures__/PopoverHost.svelte";

describe("Popover", () => {
  it("opens, focuses first input, closes on Escape and outside mousedown", async () => {
    const user = userEvent.setup();
    render(PopoverHost);
    const trigger = screen.getByRole("button", { name: "Trigger" });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

    await user.click(trigger);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    await waitFor(() => expect(screen.getByLabelText("Name")).toHaveFocus());

    await user.keyboard("{Escape}");
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
    expect(trigger).toHaveFocus();

    await user.click(trigger);
    expect(screen.getByRole("dialog")).toBeInTheDocument();
    await user.pointer({ keys: "[MouseLeft]", target: document.body });
    await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
  });
});
