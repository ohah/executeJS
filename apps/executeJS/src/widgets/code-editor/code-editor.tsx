import React, { useRef, useEffect } from 'react';
import { Editor } from '@monaco-editor/react';
import type { CodeEditorProps } from '../../shared/types';

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  onExecute,
  language = 'javascript',
  theme = 'vs-dark',
}) => {
  const editorRef = useRef<any>(null);

  // Cmd+Enter 키바인딩 설정
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
        event.preventDefault();
        onExecute();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [onExecute]);

  // Monaco Editor 설정
  const handleEditorDidMount = (editor: any, monaco: any) => {
    try {
      editorRef.current = editor;

      // Cmd+Enter 키바인딩 추가
      if (monaco && monaco.KeyMod && monaco.KeyCode) {
        editor.addCommand(
          monaco.KeyMod.CmdOrCtrl | monaco.KeyCode.Enter,
          () => {
            const currentValue = editor.getValue();
            onExecute?.(currentValue);
          }
        );
      }

      // 에디터 포커스
      editor.focus();
    } catch (error) {
      console.error('Monaco Editor mount error:', error);
    }
  };

  // 에디터 옵션
  const editorOptions = {
    selectOnLineNumbers: true,
    roundedSelection: false,
    readOnly: false,
    cursorStyle: 'line' as const,
    automaticLayout: true,
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    fontSize: 14,
    lineHeight: 20,
    fontFamily: 'JetBrains Mono, Fira Code, monospace',
    wordWrap: 'on' as const,
    wrappingIndent: 'indent' as const,
    tabSize: 2,
    insertSpaces: true,
    renderWhitespace: 'selection' as const,
    renderLineHighlight: 'line' as const,
    bracketPairColorization: { enabled: true },
    guides: {
      bracketPairs: true,
      indentation: true,
    },
  };

  return (
    <div className="h-full w-full min-h-0">
      <Editor
        height="100%"
        width="100%"
        language={language}
        theme={theme}
        value={value}
        onChange={(newValue) => onChange(newValue || '')}
        onMount={handleEditorDidMount}
        options={editorOptions}
      />
    </div>
  );
};
