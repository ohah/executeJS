import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API
Object.defineProperty(window, '__TAURI__', {
  value: {
    invoke: vi.fn(),
  },
});

// Mock console methods to avoid noise in tests
global.console = {
  ...console,
  log: vi.fn(),
  debug: vi.fn(),
  info: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
};
