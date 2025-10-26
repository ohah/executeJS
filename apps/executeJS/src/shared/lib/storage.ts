// localStorage 헬퍼 함수들

const STORAGE_KEYS = {
  TABS: 'executejs_tabs',
  ACTIVE_TAB: 'executejs_active_tab',
} as const;

export const storage = {
  // 탭 데이터 저장
  saveTabs: (tabs: any[]) => {
    try {
      localStorage.setItem(STORAGE_KEYS.TABS, JSON.stringify(tabs));
    } catch (error) {
      console.error('Failed to save tabs to localStorage:', error);
    }
  },

  // 탭 데이터 로드
  loadTabs: (): any[] => {
    try {
      const data = localStorage.getItem(STORAGE_KEYS.TABS);
      return data ? JSON.parse(data) : [];
    } catch (error) {
      console.error('Failed to load tabs from localStorage:', error);
      return [];
    }
  },

  // 활성 탭 ID 저장
  saveActiveTab: (tabId: string) => {
    try {
      localStorage.setItem(STORAGE_KEYS.ACTIVE_TAB, tabId);
    } catch (error) {
      console.error('Failed to save active tab to localStorage:', error);
    }
  },

  // 활성 탭 ID 로드
  loadActiveTab: (): string | null => {
    try {
      return localStorage.getItem(STORAGE_KEYS.ACTIVE_TAB);
    } catch (error) {
      console.error('Failed to load active tab from localStorage:', error);
      return null;
    }
  },

  // 모든 데이터 초기화
  clear: () => {
    try {
      localStorage.removeItem(STORAGE_KEYS.TABS);
      localStorage.removeItem(STORAGE_KEYS.ACTIVE_TAB);
    } catch (error) {
      console.error('Failed to clear localStorage:', error);
    }
  },
};
