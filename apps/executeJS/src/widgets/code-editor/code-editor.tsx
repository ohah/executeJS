import React, { useCallback, useEffect, useRef } from 'react';

import { invoke } from '@tauri-apps/api/core';
import { Editor, EditorProps, Monaco } from '@monaco-editor/react';
import type { Options as PrettierOptions } from 'prettier';
import prettier from 'prettier/standalone';
import babel from 'prettier/plugins/babel';
import estree from 'prettier/plugins/estree';
import typescript from 'prettier/plugins/typescript';

import { CodeEditorProps, LintResult, LintSeverity } from '@/shared';

const prettierOptions: PrettierOptions = {
  semi: true,
  trailingComma: 'es5',
  singleQuote: true,
  printWidth: 80,
  tabWidth: 2,
  useTabs: false,
};

const severityToMarkerSeverity = (severity: LintSeverity, monaco: Monaco) => {
  switch (severity) {
    case 'error':
      return monaco.MarkerSeverity.Error;
    case 'warning':
      return monaco.MarkerSeverity.Warning;
    case 'info':
      return monaco.MarkerSeverity.Info;
    case 'hint':
      return monaco.MarkerSeverity.Hint;
    default:
      // 타입 체크로 도달 불가능하지만, 런타임 안전을 위해
      console.warn(`Unknown severity: ${severity}, defaulting to Warning`);
      return monaco.MarkerSeverity.Warning;
  }
};

export const CodeEditor: React.FC<CodeEditorProps> = ({
  value,
  onChange,
  onExecute,
  language = 'javascript',
  theme = 'vs-dark',
}) => {
  const editorRef = useRef<any>(null);
  const monacoRef = useRef<Monaco | null>(null);
  const disposablesRef = useRef<Array<{ dispose(): void }>>([]);
  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const validateCode = useCallback(async (model: any, version: number) => {
    if (debounceTimeoutRef.current) {
      clearTimeout(debounceTimeoutRef.current);
    }

    debounceTimeoutRef.current = setTimeout(async () => {
      if (!model || !monacoRef.current) return;

      try {
        const code = model.getValue();

        // Tauri 백엔드에서 oxlint 실행
        const lintResults = await invoke<Array<LintResult>>('lint_code', {
          code,
        });

        // setModelMarkers 사용
        const monaco = monacoRef.current;
        if (monaco && model.getVersionId() === version) {
          const markers = lintResults.map((result) => {
            // Monaco는 1-based 인덱스 사용
            const startColumn = Math.max(1, result.column);
            const endColumn = Math.max(startColumn + 1, result.end_column);

            // severity를 소문자로 비교하여 MarkerSeverity enum 사용
            const severity = severityToMarkerSeverity(result.severity, monaco);

            return {
              message: `${result.message} (${result.rule_id})`,
              severity,
              startLineNumber: result.line,
              startColumn: startColumn,
              endLineNumber: result.end_line,
              endColumn: endColumn,
              source: 'oxlint',
              code: result.rule_id,
            };
          });

          monaco.editor.setModelMarkers(model, 'oxlint', markers);
        }
      } catch (error) {
        console.error('oxlint validation error:', error);
        // 에러 발생 시 마커 초기화
        const monaco = monacoRef.current;
        if (monaco) {
          const model = editorRef.current?.getModel();
          if (model && model.getVersionId() === version) {
            monaco.editor.setModelMarkers(model, 'oxlint', []);
          }
        }
      }
    }, 500);
  }, []);

  // Monaco Editor 설정
  const handleEditorDidMount: EditorProps['onMount'] = (editor, monaco) => {
    try {
      editorRef.current = editor;
      monacoRef.current = monaco;

      // 기본 TypeScript validator 비활성화 (oxlint 사용 시)
      monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
        noSemanticValidation: true,
        noSyntaxValidation: true,
      });

      // 이전 등록된 포맷터가 남아있는 경우, 먼저 해제
      if (disposablesRef.current.length > 0) {
        disposablesRef.current.forEach((disposable) => {
          disposable.dispose();
        });
        disposablesRef.current = [];
      }

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

      const model = editor.getModel();

      if (model) {
        // 모델 변경 시 validation
        const contentChangeDisposable = model.onDidChangeContent(() => {
          // Reset the markers
          monaco.editor.setModelMarkers(model, 'oxlint', []);

          // Send the code to the backend for validation
          validateCode(model, model.getVersionId());
        });

        // model이 있는 경우 포맷터 + 이벤트 리스너 저장
        disposablesRef.current = [
          jsDisposable,
          tsDisposable,
          contentChangeDisposable,
        ];

        // 초기 validation
        validateCode(model, model.getVersionId());
      } else {
        // model이 없는 경우 포맷터만 저장
        disposablesRef.current = [jsDisposable, tsDisposable];
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
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
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
