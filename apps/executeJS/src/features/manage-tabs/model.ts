import { observable } from '@legendapp/state';
import { storage } from '../../shared/lib/storage';
import type { Tab } from '../../shared/types';

// 탭 관리 상태
export const tabsState = observable({
  tabs: [] as Tab[],
  activeTabId: null as string | null,
});

// 탭 ID 생성기
let tabIdCounter = 1;
const generateTabId = () => `tab_${tabIdCounter++}`;

// 초기 탭 생성
const createInitialTab = (): Tab => ({
  id: generateTabId(),
  name: 'Untitled',
  code: '',
  isActive: true,
  isDirty: false,
});

// 탭 관리 액션들
export const tabActions = {
  // 새 탭 추가
  addTab: () => {
    const newTab = createInitialTab();

    // 기존 탭들을 비활성화
    tabsState.tabs.forEach((tab) => {
      tab.isActive.set(false);
    });

    // 새 탭 추가
    tabsState.tabs.push(newTab);
    tabsState.activeTabId.set(newTab.id);

    // localStorage에 저장
    storage.saveTabs(tabsState.tabs.get());
    storage.saveActiveTab(newTab.id);
  },

  // 탭 선택
  selectTab: (tabId: string) => {
    // 모든 탭 비활성화
    tabsState.tabs.forEach((tab) => {
      tab.isActive.set(tab.id.get() === tabId);
    });

    tabsState.activeTabId.set(tabId);
    storage.saveActiveTab(tabId);
  },

  // 탭 닫기
  closeTab: (tabId: string) => {
    const tabIndex = tabsState.tabs.findIndex((tab) => tab.id === tabId);
    if (tabIndex === -1) return;

    const wasActive = tabsState.tabs[tabIndex].isActive.get();
    tabsState.tabs.splice(tabIndex, 1);

    // 닫힌 탭이 활성 탭이었다면 다른 탭 선택
    if (wasActive && tabsState.tabs.length > 0) {
      const newActiveIndex = Math.min(tabIndex, tabsState.tabs.length - 1);
      tabActions.selectTab(tabsState.tabs[newActiveIndex].id.get());
    } else if (tabsState.tabs.length === 0) {
      // 모든 탭이 닫혔다면 새 탭 생성
      tabActions.addTab();
    }

    storage.saveTabs(tabsState.tabs.get());
  },

  // 탭 이름 변경
  renameTab: (tabId: string, name: string) => {
    const tab = tabsState.tabs.find((t) => t.id.get() === tabId);
    if (tab) {
      tab.name.set(name);
      storage.saveTabs(tabsState.tabs.get());
    }
  },

  // 탭 코드 변경
  updateTabCode: (tabId: string, code: string) => {
    const tab = tabsState.tabs.find((t) => t.id.get() === tabId);
    if (tab) {
      tab.code.set(code);
      tab.isDirty.set(true);
      storage.saveTabs(tabsState.tabs.get());
    }
  },

  // 탭 저장 완료 (dirty 상태 해제)
  markTabSaved: (tabId: string) => {
    const tab = tabsState.tabs.find((t) => t.id.get() === tabId);
    if (tab) {
      tab.isDirty.set(false);
      storage.saveTabs(tabsState.tabs.get());
    }
  },

  // localStorage에서 데이터 로드
  loadFromStorage: () => {
    const savedTabs = storage.loadTabs();
    const savedActiveTab = storage.loadActiveTab();

    if (savedTabs.length > 0) {
      tabsState.tabs.set(savedTabs);
      if (savedActiveTab && savedTabs.find((t) => t.id === savedActiveTab)) {
        tabsState.activeTabId.set(savedActiveTab);
        tabActions.selectTab(savedActiveTab);
      } else {
        tabActions.selectTab(savedTabs[0].id);
      }
    } else {
      // 저장된 데이터가 없으면 초기 탭 생성
      tabActions.addTab();
    }
  },
};

// 현재 활성 탭 가져오기
export const getActiveTab = () => {
  return (
    tabsState.tabs.find(
      (tab) => tab.id.get() === tabsState.activeTabId.get()
    ) || null
  );
};
