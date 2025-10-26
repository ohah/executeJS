import React, { useEffect } from 'react';
import { Panel, PanelGroup, PanelResizeHandle } from 'react-resizable-panels';
import { useObservable } from '@legendapp/state/react';
import { TabBar } from '../../widgets/tab-bar/TabBar';
import { CodeEditor } from '../../widgets/code-editor/CodeEditor';
import { OutputPanel } from '../../widgets/output-panel/OutputPanel';
import {
  tabsState,
  tabActions,
  getActiveTab,
} from '../../features/manage-tabs/model';
import {
  executionState,
  executionActions,
} from '../../features/execute-code/model';

export const EditorPage: React.FC = () => {
  const tabs = useObservable(tabsState.tabs);
  const activeTabId = useObservable(tabsState.activeTabId);
  const executionResult = useObservable(executionState.result);
  const isExecuting = useObservable(executionState.isExecuting);

  // 컴포넌트 마운트 시 저장된 데이터 로드
  useEffect(() => {
    tabActions.loadFromStorage();
    executionActions.loadHistory();
  }, []);

  // 현재 활성 탭 가져오기
  const activeTab = getActiveTab();

  // 코드 실행 핸들러
  const handleExecuteCode = () => {
    if (activeTab) {
      executionActions.executeCode(activeTab.code.get());
    }
  };

  // 탭 코드 변경 핸들러
  const handleCodeChange = (code: string) => {
    if (activeTab) {
      tabActions.updateTabCode(activeTab.id.get(), code);
    }
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-gray-1">
      {/* 탭 바 */}
      <TabBar
        tabs={tabs}
        activeTabId={activeTabId}
        onTabSelect={tabActions.selectTab}
        onTabClose={tabActions.closeTab}
        onTabAdd={tabActions.addTab}
      />

      {/* 메인 컨텐츠 영역 */}
      <div className="flex-1 flex">
        <PanelGroup direction="horizontal" className="flex-1">
          {/* 왼쪽 패널 - 코드 에디터 */}
          <Panel defaultSize={50} minSize={30}>
            <div className="h-full bg-gray-2">
              {activeTab && (
                <CodeEditor
                  value={activeTab.code.get()}
                  onChange={handleCodeChange}
                  onExecute={handleExecuteCode}
                  language="javascript"
                  theme="vs-dark"
                />
              )}
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
