import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom';
import App from './App';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('App', () => {
  it('renders ExecuteJS title', () => {
    render(<App />);
    expect(screen.getByText('ExecuteJS')).toBeInTheDocument();
  });

  it('renders JavaScript code execution section', () => {
    render(<App />);
    expect(screen.getByText('JavaScript 코드 실행')).toBeInTheDocument();
  });

  it('renders code input textarea', () => {
    render(<App />);
    expect(
      screen.getByPlaceholderText('JavaScript 코드를 입력하세요...')
    ).toBeInTheDocument();
  });

  it('renders execute button', () => {
    render(<App />);
    expect(screen.getByText('실행')).toBeInTheDocument();
  });
});
