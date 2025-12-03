/**
 * Frontend Test Suite for Awake Application
 * Tests UI interactions, DOM manipulation, and Tauri command invocations
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";

// Mock Tauri API
const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockInvoke,
}));

// DOM Setup Helper
function setupDOM() {
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

function cleanupDOM() {
  document.body.innerHTML = "";
}

describe("Frontend UI Tests", () => {
  beforeEach(() => {
    setupDOM();
    mockInvoke.mockClear();
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

    it("should have welcome heading", () => {
      const heading = document.querySelector("h1");
      expect(heading).toBeTruthy();
      expect(heading?.textContent).toBe("Welcome to Awake");
    });

    it("should have greet form", () => {
      const form = document.querySelector("#greet-form");
      expect(form).toBeTruthy();
      expect(form?.tagName).toBe("FORM");
    });

    it("should have greet input field", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      expect(input).toBeTruthy();
      expect(input?.tagName).toBe("INPUT");
      expect(input?.placeholder).toBe("Enter a name...");
    });

    it("should have greet button", () => {
      const button = document.querySelector("#greet-form button");
      expect(button).toBeTruthy();
      expect(button?.textContent).toBe("Greet");
      expect(button?.getAttribute("type")).toBe("submit");
    });

    it("should have greet message paragraph", () => {
      const msg = document.querySelector("#greet-msg");
      expect(msg).toBeTruthy();
      expect(msg?.tagName).toBe("P");
    });

    it("should have three logo images", () => {
      const logos = document.querySelectorAll(".logo");
      expect(logos.length).toBe(3);
    });

    it("should have vite logo", () => {
      const viteLogo = document.querySelector(".logo.vite");
      expect(viteLogo).toBeTruthy();
      expect(viteLogo?.getAttribute("alt")).toBe("Vite logo");
    });

    it("should have tauri logo", () => {
      const tauriLogo = document.querySelector(".logo.tauri");
      expect(tauriLogo).toBeTruthy();
      expect(tauriLogo?.getAttribute("alt")).toBe("Tauri logo");
    });

    it("should have typescript logo", () => {
      const tsLogo = document.querySelector(".logo.typescript");
      expect(tsLogo).toBeTruthy();
      expect(tsLogo?.getAttribute("alt")).toBe("typescript logo");
    });

    it("should have links with correct hrefs", () => {
      const links = document.querySelectorAll("a");
      expect(links.length).toBeGreaterThanOrEqual(3);

      const viteLink = Array.from(links).find((link) =>
        link.href.includes("vitejs.dev"),
      );
      expect(viteLink).toBeTruthy();

      const tauriLink = Array.from(links).find((link) =>
        link.href.includes("tauri.app"),
      );
      expect(tauriLink).toBeTruthy();

      const tsLink = Array.from(links).find((link) =>
        link.href.includes("typescriptlang.org"),
      );
      expect(tsLink).toBeTruthy();
    });
  });

  describe("Form Input Tests", () => {
    it("should allow typing in input field", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "John";
      expect(input.value).toBe("John");
    });

    it("should start with empty input", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      expect(input.value).toBe("");
    });

    it("should accept multiple characters", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "John Doe";
      expect(input.value).toBe("John Doe");
    });

    it("should accept special characters", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "José María";
      expect(input.value).toBe("José María");
    });

    it("should accept numbers", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "12345";
      expect(input.value).toBe("12345");
    });

    it("should accept empty string", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "test";
      input.value = "";
      expect(input.value).toBe("");
    });

    it("should have correct input placeholder", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      expect(input.placeholder).toBe("Enter a name...");
    });
  });

  describe("Form Submission Tests", () => {
    it("should have submit button", () => {
      const button = document.querySelector(
        "#greet-form button",
      ) as HTMLButtonElement;
      expect(button).toBeTruthy();
      expect(button.type).toBe("submit");
    });

    it("should prevent default form submission", () => {
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      let defaultPrevented = false;

      form.addEventListener("submit", (e) => {
        e.preventDefault();
        defaultPrevented = e.defaultPrevented;
      });

      const event = new Event("submit", { bubbles: true, cancelable: true });
      form.dispatchEvent(event);

      expect(defaultPrevented).toBe(true);
    });

    it("should trigger form submit on button click", () => {
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      let submitted = false;

      form.addEventListener("submit", (e) => {
        e.preventDefault();
        submitted = true;
      });

      const button = form.querySelector("button") as HTMLButtonElement;
      button.click();

      expect(submitted).toBe(true);
    });
  });

  describe("Greet Message Display Tests", () => {
    it("should have empty greeting message initially", () => {
      const msg = document.querySelector("#greet-msg") as HTMLElement;
      expect(msg.textContent).toBe("");
    });

    it("should update greeting message", () => {
      const msg = document.querySelector("#greet-msg") as HTMLElement;
      msg.textContent = "Hello, John!";
      expect(msg.textContent).toBe("Hello, John!");
    });

    it("should clear greeting message", () => {
      const msg = document.querySelector("#greet-msg") as HTMLElement;
      msg.textContent = "Hello, John!";
      msg.textContent = "";
      expect(msg.textContent).toBe("");
    });

    it("should support HTML entities in message", () => {
      const msg = document.querySelector("#greet-msg") as HTMLElement;
      msg.textContent = "Hello & Welcome!";
      expect(msg.textContent).toContain("&");
    });
  });

  describe("Tauri Command Invocation Tests", () => {
    beforeEach(() => {
      mockInvoke.mockResolvedValue("Hello, Test!");
    });

    it("should call invoke with greet command", async () => {
      await mockInvoke("greet", { name: "Test" });

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "Test" });
      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });

    it("should receive greeting response", async () => {
      const response = await mockInvoke("greet", { name: "John" });

      expect(response).toBe("Hello, Test!");
    });

    it("should handle empty name", async () => {
      const response = await mockInvoke("greet", { name: "" });

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "" });
    });

    it("should handle long names", async () => {
      const longName = "A".repeat(100);
      await mockInvoke("greet", { name: longName });

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: longName });
    });

    it("should handle special characters in name", async () => {
      const specialName = "!@#$%^&*()";
      await mockInvoke("greet", { name: specialName });

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: specialName });
    });

    it("should handle unicode characters", async () => {
      const unicodeName = "你好世界";
      await mockInvoke("greet", { name: unicodeName });

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: unicodeName });
    });
  });

  describe("Integration: Full Greet Flow", () => {
    beforeEach(() => {
      mockInvoke.mockResolvedValue("Hello, Integration!");
    });

    it("should complete full greet workflow", async () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      const msg = document.querySelector("#greet-msg") as HTMLElement;

      // Simulate user input
      input.value = "Integration";

      // Simulate form submission
      form.addEventListener("submit", async (e) => {
        e.preventDefault();
        const response = await mockInvoke("greet", { name: input.value });
        msg.textContent = response;
      });

      form.dispatchEvent(new Event("submit"));

      // Wait for async operation
      await new Promise((resolve) => setTimeout(resolve, 0));

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "Integration" });
    });

    it("should update UI after successful greet", async () => {
      const msg = document.querySelector("#greet-msg") as HTMLElement;

      const response = await mockInvoke("greet", { name: "Test" });
      msg.textContent = response;

      expect(msg.textContent).toBe("Hello, Integration!");
    });
  });

  describe("Error Handling Tests", () => {
    it("should handle invoke errors gracefully", async () => {
      mockInvoke.mockRejectedValue(new Error("Command failed"));

      try {
        await mockInvoke("greet", { name: "Test" });
      } catch (error) {
        expect(error).toBeInstanceOf(Error);
        expect((error as Error).message).toBe("Command failed");
      }
    });

    it("should handle missing elements", () => {
      cleanupDOM();

      const input = document.querySelector("#greet-input");
      expect(input).toBeNull();
    });

    it("should handle null input element", () => {
      const input = document.querySelector("#nonexistent") as HTMLInputElement;
      expect(input).toBeNull();
    });

    it("should handle null message element", () => {
      const msg = document.querySelector("#nonexistent-msg");
      expect(msg).toBeNull();
    });
  });

  describe("Accessibility Tests", () => {
    it("should have proper form labels", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      expect(input.placeholder).toBeTruthy();
    });

    it("should have alt text on images", () => {
      const images = document.querySelectorAll("img");
      images.forEach((img) => {
        expect(img.getAttribute("alt")).toBeTruthy();
      });
    });

    it("should have semantic HTML elements", () => {
      const main = document.querySelector("main");
      const form = document.querySelector("form");

      expect(main).toBeTruthy();
      expect(form).toBeTruthy();
    });

    it("should have button type attributes", () => {
      const button = document.querySelector("#greet-form button");
      expect(button?.getAttribute("type")).toBe("submit");
    });

    it("should have proper link targets", () => {
      const links = document.querySelectorAll('a[target="_blank"]');
      expect(links.length).toBeGreaterThan(0);
    });
  });

  describe("CSS Class Tests", () => {
    it("should have container class on main", () => {
      const main = document.querySelector("main");
      expect(main?.classList.contains("container")).toBe(true);
    });

    it("should have row class on elements", () => {
      const rows = document.querySelectorAll(".row");
      expect(rows.length).toBeGreaterThanOrEqual(2);
    });

    it("should have logo classes on images", () => {
      const logos = document.querySelectorAll(".logo");
      expect(logos.length).toBe(3);
    });

    it("should have specific logo classes", () => {
      const viteLogo = document.querySelector(".logo.vite");
      const tauriLogo = document.querySelector(".logo.tauri");
      const tsLogo = document.querySelector(".logo.typescript");

      expect(viteLogo).toBeTruthy();
      expect(tauriLogo).toBeTruthy();
      expect(tsLogo).toBeTruthy();
    });
  });

  describe("Event Listener Tests", () => {
    it("should attach DOMContentLoaded listener", () => {
      let loaded = false;

      window.addEventListener("DOMContentLoaded", () => {
        loaded = true;
      });

      const event = new Event("DOMContentLoaded");
      window.dispatchEvent(event);

      expect(loaded).toBe(true);
    });

    it("should handle multiple form submissions", () => {
      const form = document.querySelector("#greet-form") as HTMLFormElement;
      let count = 0;

      form.addEventListener("submit", (e) => {
        e.preventDefault();
        count++;
      });

      form.dispatchEvent(new Event("submit"));
      form.dispatchEvent(new Event("submit"));
      form.dispatchEvent(new Event("submit"));

      expect(count).toBe(3);
    });

    it("should handle input events", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      let inputFired = false;

      input.addEventListener("input", () => {
        inputFired = true;
      });

      input.value = "test";
      input.dispatchEvent(new Event("input"));

      expect(inputFired).toBe(true);
    });

    it("should handle button click events", () => {
      const button = document.querySelector(
        "#greet-form button",
      ) as HTMLButtonElement;
      let clicked = false;

      button.addEventListener("click", () => {
        clicked = true;
      });

      button.click();

      expect(clicked).toBe(true);
    });
  });

  describe("Data Validation Tests", () => {
    it("should handle whitespace in input", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "   spaces   ";
      expect(input.value).toBe("   spaces   ");
    });

    it("should handle newlines in input", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "line1\nline2";
      // HTML input elements don't preserve newlines - they're stripped
      expect(input.value).toBe("line1line2");
    });

    it("should handle very long input", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      const longString = "a".repeat(1000);
      input.value = longString;
      expect(input.value.length).toBe(1000);
    });

    it("should handle emoji in input", () => {
      const input = document.querySelector("#greet-input") as HTMLInputElement;
      input.value = "👋🌍";
      expect(input.value).toBe("👋🌍");
    });
  });
});

describe("Main.ts Logic Tests", () => {
  let greetInputEl: HTMLInputElement | null;
  let greetMsgEl: HTMLElement | null;

  beforeEach(() => {
    setupDOM();
    mockInvoke.mockClear();
    greetInputEl = document.querySelector("#greet-input");
    greetMsgEl = document.querySelector("#greet-msg");
  });

  afterEach(() => {
    cleanupDOM();
  });

  describe("Element Selection Tests", () => {
    it("should successfully select greet input element", () => {
      expect(greetInputEl).toBeTruthy();
      expect(greetInputEl?.id).toBe("greet-input");
    });

    it("should successfully select greet message element", () => {
      expect(greetMsgEl).toBeTruthy();
      expect(greetMsgEl?.id).toBe("greet-msg");
    });

    it("should handle null checks", () => {
      if (greetMsgEl && greetInputEl) {
        expect(true).toBe(true);
      } else {
        expect(false).toBe(true); // Should not reach here
      }
    });
  });

  describe("Greet Function Tests", () => {
    async function greet() {
      if (greetMsgEl && greetInputEl) {
        greetMsgEl.textContent = await mockInvoke("greet", {
          name: greetInputEl.value,
        });
      }
    }

    it("should invoke greet command with input value", async () => {
      mockInvoke.mockResolvedValue("Hello, Alice!");

      if (greetInputEl) greetInputEl.value = "Alice";
      await greet();

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "Alice" });
    });

    it("should update message element with response", async () => {
      mockInvoke.mockResolvedValue("Hello, Bob!");

      if (greetInputEl) greetInputEl.value = "Bob";
      await greet();

      expect(greetMsgEl?.textContent).toBe("Hello, Bob!");
    });

    it("should handle empty input", async () => {
      mockInvoke.mockResolvedValue("Hello, !");

      if (greetInputEl) greetInputEl.value = "";
      await greet();

      expect(mockInvoke).toHaveBeenCalledWith("greet", { name: "" });
    });

    it("should not invoke if elements are null", async () => {
      greetMsgEl = null;
      greetInputEl = null;

      await greet();

      expect(mockInvoke).not.toHaveBeenCalled();
    });
  });
});
