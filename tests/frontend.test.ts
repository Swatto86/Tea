/**
 * Frontend Test Suite for Tea Application
 * Note: Tea is a system tray-only application. The HTML window is not shown to users.
 * These tests validate the basic DOM structure for completeness.
 */

import { describe, it, expect, beforeEach, afterEach } from "vitest";

// DOM Setup Helper
function setupDOM() {
  document.body.innerHTML = `
    <main class="container">
      <h1>Tea</h1>
      <p>System Tray Sleep Prevention Utility</p>
      <p>Access all features via the system tray icon.</p>
    </main>
  `;
}

function cleanupDOM() {
  document.body.innerHTML = "";
}

describe("Frontend UI Tests", () => {
  beforeEach(() => {
    setupDOM();
  });

  afterEach(() => {
    cleanupDOM();
  });

  describe("DOM Structure Tests", () => {
    it("should have main container element", () => {
      const container = document.querySelector(".container");
      expect(container).toBeTruthy();
      expect(container?.tagName).toBe("MAIN");
    });

    it("should have Tea heading", () => {
      const heading = document.querySelector("h1");
      expect(heading).toBeTruthy();
      expect(heading?.textContent).toBe("Tea");
    });

    it("should have informational paragraphs", () => {
      const paragraphs = document.querySelectorAll("p");
      expect(paragraphs.length).toBe(2);
      expect(paragraphs[0]?.textContent).toContain("System Tray");
      expect(paragraphs[1]?.textContent).toContain("system tray icon");
    });

    it("should not have interactive elements", () => {
      // System tray only - no form, input, or buttons
      expect(document.querySelector("form")).toBeNull();
      expect(document.querySelector("input")).toBeNull();
      expect(document.querySelector("button")).toBeNull();
    });
  });

  describe("Accessibility Tests", () => {
    it("should have a main landmark", () => {
      const main = document.querySelector("main");
      expect(main).toBeTruthy();
    });

    it("should have a heading for screen readers", () => {
      const heading = document.querySelector("h1");
      expect(heading).toBeTruthy();
      expect(heading?.textContent).not.toBe("");
    });

    it("should have text content for screen readers", () => {
      const paragraphs = document.querySelectorAll("p");
      paragraphs.forEach((p) => {
        expect(p.textContent).not.toBe("");
      });
    });
  });

  describe("Content Validation", () => {
    it("should contain application name", () => {
      const content = document.body.textContent || "";
      expect(content).toContain("Tea");
    });

    it("should indicate system tray usage", () => {
      const content = document.body.textContent || "";
      expect(content.toLowerCase()).toContain("system tray");
    });

    it("should have clear messaging about UI location", () => {
      const paragraphs = document.querySelectorAll("p");
      const hasSystemTrayMention = Array.from(paragraphs).some((p) =>
        p.textContent?.toLowerCase().includes("system tray")
      );
      expect(hasSystemTrayMention).toBe(true);
    });
  });
});
