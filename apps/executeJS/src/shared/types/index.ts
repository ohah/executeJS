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

export interface LintResult {
  line: number;
  column: number;
  end_line: number;
  end_column: number;
  message: string;
  severity: string;
  rule_id: string;
}
