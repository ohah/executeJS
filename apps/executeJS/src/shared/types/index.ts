// 공통 타입 정의

export interface JsExecutionResult {
  code: string;
  result: string;
  timestamp: string;
  success: boolean;
  error?: string;
}

export interface Tab {
  id: string;
  name: string;
  code: string;
  isActive: boolean;
  isDirty: boolean; // 저장되지 않은 변경사항이 있는지
}

export interface AppState {
  tabs: Tab[];
  activeTabId: string | null;
  executionResult: JsExecutionResult | null;
  isExecuting: boolean;
}

export interface MonacoEditorProps {
  value: string;
  onChange: (value: string) => void;
  onExecute: () => void;
  language?: string;
  theme?: string;
}

export interface TabBarProps {
  tabs: any; // Legend State Observable 타입
  activeTabId: any; // Legend State Observable 타입
  onTabSelect: (tabId: string) => void;
  onTabClose: (tabId: string) => void;
  onTabAdd: () => void;
}

export interface OutputPanelProps {
  result: any; // Legend State Observable 타입
  isExecuting: any; // Legend State Observable 타입
}
