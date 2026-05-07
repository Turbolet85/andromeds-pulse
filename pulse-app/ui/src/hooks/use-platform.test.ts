import { describe, expect, it } from "vitest";
import { detectPlatform } from "./use-platform";

describe("detectPlatform — userAgent matrix", () => {
  it("detects macOS from Macintosh substring", () => {
    expect(
      detectPlatform(
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15",
      ),
    ).toBe("macos");
  });

  it("detects macOS from Mac OS substring", () => {
    expect(detectPlatform("Some-UA Mac OS 15 build")).toBe("macos");
  });

  it("detects Windows from Windows substring", () => {
    expect(
      detectPlatform(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
      ),
    ).toBe("windows");
  });

  it("detects Windows from Win32 substring", () => {
    expect(detectPlatform("Some-UA Win32 build")).toBe("windows");
  });

  it("falls back to linux for non-mac non-windows UA", () => {
    expect(
      detectPlatform("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"),
    ).toBe("linux");
  });

  it("falls back to linux for empty UA", () => {
    expect(detectPlatform("")).toBe("linux");
  });

  it("is case-insensitive", () => {
    expect(detectPlatform("MOZILLA/5.0 (MACINTOSH)")).toBe("macos");
    expect(detectPlatform("MOZILLA/5.0 (WINDOWS NT 11)")).toBe("windows");
  });
});
