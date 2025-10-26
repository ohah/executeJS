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
  const tabs = useObservable(() => tabsState.tabs.get());
  const activeTabId = useObservable(() => tabsState.activeTabId.get());
  const executionResult = useObservable(() => executionState.result.get());
  const isExecuting = useObservable(() => executionState.isExecuting.get());

  // 컴포넌트 마운트 시 저장된 데이터 로드
  useEffect(() => {
    // 초기화를 약간 지연시켜서 컴포넌트가 완전히 마운트된 후 실행
    const timer = setTimeout(() => {
      tabActions.loadFromStorage();
      executionActions.loadHistory();
    }, 100);

    return () => clearTimeout(timer);
  }, []);

  // 현재 활성 탭 가져오기
  const activeTab = useObservable(() => getActiveTab());

  // 디버깅용 로그
  console.log('EditorPage Debug:', {
    tabs: tabs,
    activeTabId: activeTabId,
    activeTab: activeTab.get(),
  });

  // 코드 실행 핸들러
  const handleExecuteCode = () => {
    const currentTab = activeTab.get();
    if (currentTab) {
      executionActions.executeCode(currentTab.code);
    }
  };

  // 탭 코드 변경 핸들러
  const handleCodeChange = (code: string) => {
    const currentTab = activeTab.get();
    if (currentTab) {
      tabActions.updateTabCode(currentTab.id, code);
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
              {activeTab.get() ? (
                <CodeEditor
                  value={activeTab.get()?.code || ''}
                  onChange={handleCodeChange}
                  onExecute={handleExecuteCode}
                  language="javascript"
                  theme="vs-dark"
                />
              ) : (
                <div className="flex items-center justify-center h-full text-gray-9">
                  탭을 로딩 중...
                </div>
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
