import { describe, it, expect } from "vitest";
import { resolveTheme } from "./theme.svelte";

describe("resolveTheme", () => {
  it("system + prefers dark resolves to dark", () => {
    expect(resolveTheme("system", true)).toBe("dark");
  });

  it("system + prefers light resolves to light", () => {
    expect(resolveTheme("system", false)).toBe("light");
  });

  it("explicit light ignores the OS preference", () => {
    expect(resolveTheme("light", true)).toBe("light");
  });

  it("explicit dark ignores the OS preference", () => {
    expect(resolveTheme("dark", false)).toBe("dark");
  });
});
