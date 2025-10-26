import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom';
import { EditorPage } from './editor-page';

// Tauri API 모킹
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Legend State 모킹
vi.mock('@legendapp/state/react', () => ({
  useObservable: vi.fn(() => []),
}));

describe('EditorPage', () => {
  it('renders without crashing', () => {
    render(<EditorPage />);
    expect(
      screen.getByText(/코드를 실행해보세요/)
    ).toBeInTheDocument();
  });
});
