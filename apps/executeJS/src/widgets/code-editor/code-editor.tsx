import React, { useEffect, useRef } from 'react';

import { Editor, EditorProps } from '@monaco-editor/react';
import type { Options as PrettierOptions } from 'prettier';
import prettier from 'prettier/standalone';
import babel from 'prettier/plugins/babel';
import estree from 'prettier/plugins/estree';
import typescript from 'prettier/plugins/typescript';

import type { CodeEditorProps } from '../../shared/types';

const prettierOptions: PrettierOptions = {
  semi: true,
  trailingComma: 'es5',
  singleQuote: true,
  printWidth: 80,
  tabWidth: 2,
  useTabs: false,
};

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  onExecute,
  language = 'javascript',
  theme = 'vs-dark',
}) => {
  const editorRef = useRef<any>(null);
  const disposablesRef = useRef<Array<{ dispose(): void }>>([]);

  // Monaco Editor 설정
  const handleEditorDidMount: EditorProps['onMount'] = (editor, monaco) => {
    try {
      editorRef.current = editor;

      // JavaScript 포맷터 등록
      const jsDisposable =
        monaco.languages.registerDocumentFormattingEditProvider('javascript', {
          async provideDocumentFormattingEdits(model) {
            const text = model.getValue();

            try {
              const formatted = await prettier.format(text, {
                ...prettierOptions,
                parser: 'babel',
                plugins: [babel, estree],
              });

              return [
                {
                  range: model.getFullModelRange(),
                  text: formatted,
                },
              ];
            } catch (error) {
              console.error('Prettier formatting error:', error);
              return [];
            }
          },
        });

      // TypeScript 포맷터 등록
      const tsDisposable =
        monaco.languages.registerDocumentFormattingEditProvider('typescript', {
          async provideDocumentFormattingEdits(model) {
            const text = model.getValue();

            try {
              const formatted = await prettier.format(text, {
                ...prettierOptions,
                parser: 'typescript',
                plugins: [typescript, estree],
              });

              return [
                {
                  range: model.getFullModelRange(),
                  text: formatted,
                },
              ];
            } catch (error) {
              console.error('Prettier formatting error:', error);
              return [];
            }
          },
        });

      // Disposable들을 ref에 저장
      disposablesRef.current = [jsDisposable, tsDisposable];

      // 단축키 바인딩
      if (monaco && monaco.KeyMod && monaco.KeyCode) {
        // Cmd+Enter 코드 실행
        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => {
          const currentValue = editor.getValue();
          onExecute?.(currentValue);
        });

        // Cmd+Shift+F prettier 포맷 실행
        editor.addCommand(
          monaco.KeyMod.CtrlCmd | monaco.KeyMod.Shift | monaco.KeyCode.KeyF,
          () => {
            editor.getAction('editor.action.formatDocument')?.run();
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
  const editorOptions: EditorProps['options'] = {
    selectOnLineNumbers: true,
    roundedSelection: false,
    readOnly: false,
    cursorStyle: 'line' as const,
    automaticLayout: true,
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    fontSize: 14,
    lineHeight: 22,
    fontFamily: 'JetBrains Mono, Fira Code, Consolas, Monaco, monospace',
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
    padding: { top: 16, bottom: 16 },
    scrollbar: {
      vertical: 'auto' as const,
      horizontal: 'auto' as const,
      useShadows: false,
      verticalHasArrows: false,
      horizontalHasArrows: false,
      verticalScrollbarSize: 8,
      horizontalScrollbarSize: 8,
    },
    contextmenu: true,
    mouseWheelZoom: true,
    cursorBlinking: 'blink' as const,
    // Enter 키로 실행되지 않도록 설정
    quickSuggestions: false,
    suggestOnTriggerCharacters: false,
    acceptSuggestionOnEnter: 'off' as const,
  };

  // Cleanup: unmount 시 포맷터 등록 해제
  useEffect(() => {
    return () => {
      // 모든 disposable 해제
      disposablesRef.current.forEach((disposable) => {
        disposable.dispose();
      });

      disposablesRef.current = [];
    };
  }, []);

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
