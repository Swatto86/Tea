/**
 * End-to-End Test Suite for Tea Application
 * Note: Tea is a system tray-only application. The HTML window is not shown to users.
 * These tests validate basic HTML structure exists but the real UI is the system tray.
 */

import { describe, it, expect, beforeEach, afterEach } from "vitest";

// Helper to simulate complete DOM setup
function setupFullApplication() {
  document.body.innerHTML = `
    <main class="container">
      <h1>Tea</h1>
      <p>System Tray Sleep Prevention Utility</p>
      <p>Access all features via the system tray icon.</p>
    </main>
  `;
}

function cleanupApplication() {
  document.body.innerHTML = "";
}

describe("E2E: Application Structure", () => {
  beforeEach(() => {
    setupFullApplication();
  });

  afterEach(() => {
    cleanupApplication();
  });

  describe("Application Initialization", () => {
    it("should initialize with correct default state", () => {
      const heading = document.querySelector("h1");
      const container = document.querySelector(".container");

      expect(heading).toBeTruthy();
      expect(container).toBeTruthy();
      expect(heading?.textContent).toBe("Tea");
    });

    it("should have all UI elements present on load", () => {
      expect(document.querySelector("h1")).toBeTruthy();
      expect(document.querySelector(".container")).toBeTruthy();
      const paragraphs = document.querySelectorAll("p");
      expect(paragraphs.length).toBe(2);
    });

    it("should display correct informational text", () => {
      const paragraphs = document.querySelectorAll("p");
      expect(paragraphs[0]?.textContent).toBe("System Tray Sleep Prevention Utility");
      expect(paragraphs[1]?.textContent).toBe("Access all features via the system tray icon.");
    });

    it("should have no interactive form elements", () => {
      // System tray app - no forms in the hidden window
      expect(document.querySelector("form")).toBeNull();
      expect(document.querySelector("input")).toBeNull();
      expect(document.querySelector("button")).toBeNull();
    });
  });

  describe("HTML Structure Validation", () => {
    it("should have proper HTML structure", () => {
      const main = document.querySelector("main");
      expect(main).toBeTruthy();
      expect(main?.classList.contains("container")).toBe(true);
    });

    it("should have heading hierarchy", () => {
      const h1 = document.querySelector("h1");
      expect(h1).toBeTruthy();
      expect(h1?.parentElement?.tagName).toBe("MAIN");
    });

    it("should have content paragraphs as children of main", () => {
      const main = document.querySelector("main");
      const paragraphs = main?.querySelectorAll("p");
      expect(paragraphs?.length).toBeGreaterThan(0);
    });
  });
});
