// 공통 타입 정의

export interface JsExecutionResult {
  code: string;
  result: string;
  timestamp: string;
  success: boolean;
  error?: string;
}

export interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
  onExecute: (code?: string) => void;
  language?: string;
  theme?: string;
}

export interface OutputPanelProps {
  result: JsExecutionResult | null;
  isExecuting: boolean;
}
