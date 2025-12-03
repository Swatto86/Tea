/**
 * Test setup file for Vitest
 * Configures global test environment and mocks
 */

import { vi } from 'vitest';

// Mock Tauri API
global.window = global.window || ({} as any);

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock Tauri plugin shell
vi.mock('@tauri-apps/plugin-shell', () => ({
  open: vi.fn(),
}));

// Setup DOM globals
if (typeof window !== 'undefined') {
  // Add any window-specific setup here
}

// Mock console methods for cleaner test output (optional)
// global.console = {
//   ...console,
//   log: vi.fn(),
//   debug: vi.fn(),
//   info: vi.fn(),
//   warn: vi.fn(),
//   error: vi.fn(),
// };
