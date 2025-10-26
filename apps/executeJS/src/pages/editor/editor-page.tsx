import React, { useState } from 'react';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { CodeEditor } from '@/widgets/code-editor';
import { OutputPanel } from '@/widgets/output-panel';
import { useExecutionStore } from '@/features/execute-code';

export const EditorPage: React.FC = () => {
  const [code, setCode] = useState('');
  const {
    result: executionResult,
    isExecuting,
    executeCode,
  } = useExecutionStore();

  // 코드 실행 핸들러
  const handleExecuteCode = (codeToExecute?: string) => {
    if (codeToExecute) {
      executeCode(codeToExecute);
    }
  };

  // 코드 변경 핸들러
  const handleCodeChange = (newCode: string) => {
    setCode(newCode);
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-gray-1">
      {/* 메인 컨텐츠 영역 */}
      <div className="flex-1 flex h-screen">
        <PanelGroup direction="horizontal" className="flex-1 h-full border">
          {/* 왼쪽 패널 - 코드 에디터 */}
          <Panel defaultSize={50} minSize={30}>
            <div className="h-full bg-gray-2">
              <CodeEditor
                value={code}
                onChange={handleCodeChange}
                onExecute={handleExecuteCode}
                language="javascript"
                theme="vs-dark"
              />
            </div>
          </Panel>

          {/* 리사이즈 핸들 */}
          <PanelResizeHandle className="panel-resize-handle" />

          {/* 오른쪽 패널 - 출력 결과 */}
          <Panel defaultSize={50} minSize={30}>
            <OutputPanel result={executionResult} isExecuting={isExecuting} />
          </Panel>
        </PanelGroup>
      </div>
    </div>
  );
};
