import React, { useState } from 'react';

import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { PlayIcon, StopIcon } from '@radix-ui/react-icons';

import { CodeEditor } from '@/widgets/code-editor';
import { OutputPanel } from '@/widgets/output-panel';
import {
  getInitialCode,
  Playground,
  usePlaygroundStore,
} from '@/features/playground';

interface PlaygroundProps {
  playground: Playground;
}

export const PlaygroundWidget: React.FC<PlaygroundProps> = ({ playground }) => {
  const { id, isExecuting, result: executionResult } = playground;

  const [code, setCode] = useState(getInitialCode(id));

  const { executeCode } = usePlaygroundStore();

  // 코드 실행 핸들러
  const handleExecuteCode = (codeToExecute?: string) => {
    const codeToRun = codeToExecute || code;
    if (codeToRun.trim()) {
      executeCode({ code: codeToRun, playgroundId: id });
    }
  };

  // 코드 변경 핸들러
  const handleCodeChange = (newCode: string) => {
    setCode(newCode);
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-slate-950 text-white">
      {/* 헤더 */}
      <div className="flex items-center justify-between px-4 py-3 bg-slate-900 border-b border-slate-800">
        <div className="flex items-center gap-3">
          <div className="text-sm font-medium text-slate-300">ExecuteJS</div>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={() => handleExecuteCode()}
            disabled={isExecuting || !code.trim()}
            className="flex items-center gap-2 px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:cursor-not-allowed text-white text-sm font-medium rounded-md transition-colors"
          >
            {isExecuting ? (
              <>
                <StopIcon className="w-4 h-4" />
                실행 중...
              </>
            ) : (
              <>
                <PlayIcon className="w-4 h-4" />
                실행 (Cmd+Enter)
              </>
            )}
          </button>
        </div>
      </div>

      {/* 메인 컨텐츠 영역 */}
      <div className="flex-1 flex">
        <PanelGroup direction="horizontal" className="flex-1">
          {/* 왼쪽 패널 - 코드 에디터 */}
          <Panel defaultSize={50} minSize={30}>
            <div className="h-full bg-slate-900 border-r border-slate-800">
              <div className="h-8 bg-slate-800 border-b border-slate-700 flex items-center px-4">
                <span className="text-xs font-medium text-slate-400 uppercase tracking-wide">
                  Editor
                </span>
              </div>
              <div className="h-[calc(100%-2rem)]">
                <CodeEditor
                  value={code}
                  onChange={handleCodeChange}
                  onExecute={handleExecuteCode}
                  language="javascript"
                  theme="vs-dark"
                />
              </div>
            </div>
          </Panel>

          {/* 리사이즈 핸들 */}
          <PanelResizeHandle className="w-1 bg-slate-800 hover:bg-slate-700 transition-colors" />

          {/* 오른쪽 패널 - 출력 결과 */}
          <Panel defaultSize={50} minSize={30}>
            <div className="h-full bg-slate-900">
              <div className="h-8 bg-slate-800 border-b border-slate-700 flex items-center px-4">
                <span className="text-xs font-medium text-slate-400 uppercase tracking-wide">
                  Output
                </span>
              </div>
              <div className="h-[calc(100%-2rem)]">
                <OutputPanel
                  result={executionResult}
                  isExecuting={isExecuting}
                />
              </div>
            </div>
          </Panel>
        </PanelGroup>
      </div>
    </div>
  );
};
