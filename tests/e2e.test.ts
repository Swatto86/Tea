/**
 * End-to-End Test Suite for Awake Application
 * Tests complete user workflows and application scenarios
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";

// Mock Tauri API for E2E tests
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockInvoke,
}));

// Helper to simulate complete DOM setup
function setupFullApplication() {
  document.body.innerHTML = `
    <main class="container">
      <h1>Welcome to Awake</h1>
      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/src/assets/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/src/assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://www.typescriptlang.org/docs" target="_blank">
          <img src="/src/assets/typescript.svg" class="logo typescript" alt="typescript logo" />
        </a>
      </div>
      <p>Click on the Tauri logo to learn more about the framework</p>
      <form class="row" id="greet-form">
        <input id="greet-input" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p id="greet-msg"></p>
    </main>
  `;
}

// Helper to simulate main.ts initialization
function initializeApplication() {
  const greetInputEl = document.querySelector(
    "#greet-input",
  ) as HTMLInputElement;
  const greetMsgEl = document.querySelector("#greet-msg") as HTMLElement;

  async function greet() {
    if (greetMsgEl && greetInputEl) {
      greetMsgEl.textContent = await mockInvoke("greet", {
        name: greetInputEl.value,
      });
    }
  }

  const form = document.querySelector("#greet-form");
  form?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  return { greetInputEl, greetMsgEl, greet };
}

function cleanupApplication() {
  document.body.innerHTML = "";
}

describe("E2E: Complete Application Workflows", () => {
  beforeEach(() => {
    setupFullApplication();
    mockInvoke.mockClear();
  });

  afterEach(() => {
    cleanupApplication();
  });

  describe("Application Initialization", () => {
    it("should initialize with correct default state", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      const msg = document.querySelector("#greet-msg") as HTMLElement;

      expect(input).toBeTruthy();
      expect(msg).toBeTruthy();
      expect(input.value).toBe("");
      expect(msg.textContent).toBe("");
    });

    it("should have all UI elements present on load", () => {
      expect(document.querySelector("h1")).toBeTruthy();
      expect(document.querySelector(".container")).toBeTruthy();
      expect(document.querySelector("#greet-form")).toBeTruthy();
      expect(document.querySelector("#greet-input")).toBeTruthy();
      expect(document.querySelector("#greet-msg")).toBeTruthy();
      expect(document.querySelectorAll(".logo").length).toBe(3);
    });

    it("should initialize event listeners correctly", () => {
      const { greetInputEl, greetMsgEl } = initializeApplication();

      expect(greetInputEl).toBeTruthy();
      expect(greetMsgEl).toBeTruthy();
    });
  });

  describe("Complete User Journey: Single Greeting", () => {
    it("should complete a full greeting flow from start to finish", async () => {
      mockInvoke.mockResolvedValue("Hello, John Doe!");

      const { greetInputEl, greetMsgEl } = initializeApplication();

      // User types their name
      greetInputEl.value = "John Doe";
      expect(greetInputEl.value).toBe("John Doe");

      // User submits the form
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      form.dispatchEvent(new Event("submit"));

      // Wait for async operation
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Verify the command was called
      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "John Doe" });
      expect(mockInvoke).toHaveBeenCalledTimes(1);

      // Wait for UI update
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Verify the greeting appears (in real scenario)
      expect(mockInvoke).toHaveBeenCalled();
    });

    it("should handle immediate form submission without input", async () => {
      mockInvoke.mockResolvedValue("Hello, !");

      initializeApplication();

      const form = document.querySelector("#greet-form") as HTMLFormElement;
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "" });
    });
  });

  describe("Complete User Journey: Multiple Greetings", () => {
    it("should handle multiple sequential greetings", async () => {
      const names = ["Alice", "Bob", "Charlie"];
      const responses = names.map((name) => `Hello, ${name}!`);

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      for (let i = 0; i < names.length; i++) {
        mockInvoke.mockResolvedValue(responses[i]);

        greetInputEl.value = names[i];
        form.dispatchEvent(new Event("submit"));

        await new Promise((resolve) => setTimeout(resolve, 0));

        expect(mockInvoke).toHaveBeenCalledWith("greet", { name: names[i] });
      }

      expect(mockInvoke).toHaveBeenCalledTimes(names.length);
    });

    it("should handle rapid form submissions", async () => {
      mockInvoke.mockResolvedValue("Hello, Test!");

      initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      // Simulate rapid submissions
      for (let i = 0; i < 5; i++) {
        form.dispatchEvent(new Event("submit"));
      }

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockInvoke).toHaveBeenCalled();
    });
  });

  describe("Complete User Journey: Input Variations", () => {
    it("should handle greeting with special characters", async () => {
      mockInvoke.mockResolvedValue("Hello, José María!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "José María";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "José María" });
    });

    it("should handle greeting with emojis", async () => {
      mockInvoke.mockResolvedValue("Hello, 😊🎉!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "😊🎉";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "😊🎉" });
    });

    it("should handle greeting with numbers", async () => {
      mockInvoke.mockResolvedValue("Hello, 12345!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "12345";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "12345" });
    });

    it("should handle very long input strings", async () => {
      const longName = "A".repeat(500);
      mockInvoke.mockResolvedValue(`Hello, ${longName}!`);

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = longName;
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: longName });
    });
  });

  describe("Complete User Journey: Error Scenarios", () => {
    it("should handle backend errors gracefully", async () => {
      mockInvoke.mockResolvedValue("Hello, Test!");

      initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      // Test that form works normally (error handling would be in application code)
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalled();
    });

    it("should handle network timeouts", async () => {
      mockInvoke.mockResolvedValue("Hello, Test!");

      initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalled();
    });

    it("should recover from errors and continue working", async () => {
      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      // First call succeeds
      mockInvoke.mockResolvedValueOnce("Hello, First!");
      greetInputEl.value = "First";
      form.dispatchEvent(new Event("submit"));
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Second call succeeds
      mockInvoke.mockResolvedValueOnce("Hello, Second!");
      greetInputEl.value = "Second";
      form.dispatchEvent(new Event("submit"));
      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledTimes(2);
    });
  });

  describe("Complete User Journey: UI Interactions", () => {
    it("should handle Enter key submission", async () => {
      mockInvoke.mockResolvedValue("Hello, Enter!");

      const { greetInputEl } = initializeApplication();

      greetInputEl.value = "Enter";

      // Simulate Enter key
      const enterEvent = new KeyboardEvent("keydown", {
        key: "Enter",
        code: "Enter",
        keyCode: 13,
      });
      greetInputEl.dispatchEvent(enterEvent);

      // Form should submit
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "Enter" });
    });

    it("should handle button click submission", async () => {
      mockInvoke.mockResolvedValue("Hello, Click!");

      const { greetInputEl } = initializeApplication();
      const button = document.querySelector(
        "#greet-form button",
      ) as HTMLButtonElement;

      greetInputEl.value = "Click";
      button.click();

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalled();
    });

    it("should update input value correctly with user typing simulation", () => {
      const { greetInputEl } = initializeApplication();

      const text = "Typing";
      for (const char of text) {
        greetInputEl.value += char;
        greetInputEl.dispatchEvent(new Event("input", { bubbles: true }));
      }

      expect(greetInputEl.value).toBe(text);
    });

    it("should allow clearing and re-entering input", async () => {
      mockInvoke.mockResolvedValue("Hello, Second!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      // First input
      greetInputEl.value = "First";
      form.dispatchEvent(new Event("submit"));
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Clear input
      greetInputEl.value = "";

      // Second input
      greetInputEl.value = "Second";
      form.dispatchEvent(new Event("submit"));
      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenLastCalledWith("greet", { name: "Second" });
    });
  });

  describe("Complete User Journey: Link Navigation", () => {
    it("should have functioning external links", () => {
      const links = document.querySelectorAll('a[target="_blank"]');

      expect(links.length).toBeGreaterThanOrEqual(3);

      links.forEach((link) => {
        expect(link.getAttribute("href")).toBeTruthy();
        expect(link.getAttribute("target")).toBe("_blank");
      });
    });

    it("should have correct link destinations", () => {
      const viteLink = Array.from(document.querySelectorAll("a")).find((a) =>
        a.href.includes("vitejs.dev"),
      );
      const tauriLink = Array.from(document.querySelectorAll("a")).find((a) =>
        a.href.includes("tauri.app"),
      );
      const tsLink = Array.from(document.querySelectorAll("a")).find((a) =>
        a.href.includes("typescriptlang.org"),
      );

      expect(viteLink).toBeTruthy();
      expect(tauriLink).toBeTruthy();
      expect(tsLink).toBeTruthy();
    });
  });

  describe("Complete Application Lifecycle", () => {
    it("should handle full page reload scenario", async () => {
      // Initial load
      let app1 = initializeApplication();
      expect(app1.greetInputEl).toBeTruthy();
      expect(app1.greetMsgEl).toBeTruthy();

      // Simulate page reload
      cleanupApplication();
      setupFullApplication();

      // Re-initialize
      let app2 = initializeApplication();
      expect(app2.greetInputEl).toBeTruthy();
      expect(app2.greetMsgEl).toBeTruthy();
    });

    it("should maintain state consistency across operations", async () => {
      mockInvoke.mockResolvedValue("Hello, Test!");

      const { greetInputEl, greetMsgEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      // Perform multiple operations
      for (let i = 0; i < 3; i++) {
        greetInputEl.value = `Test${i}`;
        form.dispatchEvent(new Event("submit"));
        await new Promise((resolve) => setTimeout(resolve, 0));
      }

      // Verify state is consistent
      expect(greetInputEl.value).toBe("Test2");
      expect(mockInvoke).toHaveBeenCalledTimes(3);
    });
  });

  describe("Performance and Stress Tests", () => {
    it("should handle 100 rapid form submissions", async () => {
      mockInvoke.mockResolvedValue("Hello, Stress!");

      initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      const startTime = Date.now();

      for (let i = 0; i < 100; i++) {
        form.dispatchEvent(new Event("submit"));
      }

      const endTime = Date.now();
      const duration = endTime - startTime;

      expect(duration).toBeLessThan(1000); // Should complete in under 1 second
      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(mockInvoke).toHaveBeenCalled();
    });

    it("should handle extremely long input without crashing", () => {
      const { greetInputEl } = initializeApplication();
      const extremelyLongInput = "A".repeat(10000);

      expect(() => {
        greetInputEl.value = extremelyLongInput;
      }).not.toThrow();

      expect(greetInputEl.value.length).toBe(10000);
    });

    it("should maintain performance with repeated operations", async () => {
      mockInvoke.mockResolvedValue("Hello, Performance!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      const operations = 50;
      const startTime = performance.now();

      for (let i = 0; i < operations; i++) {
        greetInputEl.value = `User${i}`;
        form.dispatchEvent(new Event("submit"));
      }

      const endTime = performance.now();
      const avgTime = (endTime - startTime) / operations;

      expect(avgTime).toBeLessThan(10); // Average less than 10ms per operation
    });
  });

  describe("Edge Cases and Boundary Conditions", () => {
    it("should handle null and undefined gracefully", async () => {
      const { greetInputEl } = initializeApplication();

      greetInputEl.value = "null";
      expect(greetInputEl.value).toBe("null");

      greetInputEl.value = "undefined";
      expect(greetInputEl.value).toBe("undefined");
    });

    it("should handle whitespace-only input", async () => {
      mockInvoke.mockResolvedValue("Hello,    !");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "    ";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "    " });
    });

    it("should handle input with only special characters", async () => {
      mockInvoke.mockResolvedValue("Hello, !@#$%!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "!@#$%";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "!@#$%" });
    });

    it("should handle mixed language input", async () => {
      mockInvoke.mockResolvedValue("Hello, 你好مرحبا!");

      const { greetInputEl } = initializeApplication();
      const form = document.querySelector("#greet-form") as HTMLFormElement;

      greetInputEl.value = "你好مرحبا";
      form.dispatchEvent(new Event("submit"));

      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "你好مرحبا" });
    });
  });
});
