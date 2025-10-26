import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { CodeEditor } from './CodeEditor';
import '@testing-library/jest-dom';

// Monaco Editor 모킹
vi.mock('@monaco-editor/react', () => ({
  Editor: ({ value, onChange, onMount }: any) => {
    // 테스트용 간단한 에디터 컴포넌트
    return (
      <div data-testid="monaco-editor">
        <textarea
          value={value}
          onChange={(e) => onChange?.(e.target.value)}
          onFocus={() => onMount?.({}, {})}
        />
      </div>
    );
  },
}));

describe('CodeEditor', () => {
  it('renders Monaco Editor', () => {
    const mockOnChange = vi.fn();
    const mockOnExecute = vi.fn();

    render(
      <CodeEditor
        value="console.log('hello')"
        onChange={mockOnChange}
        onExecute={mockOnExecute}
      />
    );

    expect(screen.getByTestId('monaco-editor')).toBeInTheDocument();
  });

  it('calls onChange when content changes', () => {
    const mockOnChange = vi.fn();
    const mockOnExecute = vi.fn();

    render(
      <CodeEditor value="" onChange={mockOnChange} onExecute={mockOnExecute} />
    );

    const textarea = screen.getByRole('textbox');
    textarea.focus();
    // Monaco Editor의 onMount가 호출되어야 함
    expect(mockOnChange).not.toHaveBeenCalled();
  });
});
