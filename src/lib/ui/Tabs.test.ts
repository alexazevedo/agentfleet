import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import TabsHost from "./__fixtures__/TabsHost.svelte";

describe("Tabs", () => {
  it("click activates a tab and shows its panel; ArrowRight moves on", async () => {
    const user = userEvent.setup();
    render(TabsHost);
    const [a, b, c] = screen.getAllByRole("tab");
    expect(a).toHaveAttribute("aria-selected", "true");

    await user.click(b);
    expect(a).toHaveAttribute("aria-selected", "false");
    expect(b).toHaveAttribute("aria-selected", "true");
    expect(screen.getByRole("tabpanel")).toHaveTextContent("content-b");

    await user.keyboard("{ArrowRight}");
    expect(c).toHaveAttribute("aria-selected", "true");
    expect(c).toHaveFocus();
    expect(screen.getByRole("tabpanel")).toHaveTextContent("content-c");
  });
});
