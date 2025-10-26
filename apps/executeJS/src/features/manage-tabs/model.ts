import { observable } from '@legendapp/state';
import { storage } from '../../shared/lib/storage';
import type { Tab } from '../../shared/types';

// 탭 관리 상태

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

const initialTab = createInitialTab();

export const tabsState = observable({
  tabs: [initialTab] as Tab[],
  activeTabId: initialTab.id,
});
// 탭 관리 액션들
export const tabActions = {
  // 새 탭 추가
  addTab: () => {
    console.log('addTab called');
    const newTab = createInitialTab();
    console.log('Created new tab:', newTab);

    // 기존 탭들을 비활성화
    tabsState.tabs.forEach((tab) => {
      tab.isActive.set(false);
    });

    // 새 탭 추가 (ObservableArray에 올바르게 추가)
    const currentTabs = tabsState.tabs.get();
    console.log('Current tabs before add:', currentTabs);

    tabsState.tabs.set([...currentTabs, newTab]);
    tabsState.activeTabId.set(newTab.id);

    console.log('Tabs after add:', tabsState.tabs.get());
    console.log('Active tab ID set to:', newTab.id);

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
    console.log('loadFromStorage called');

    // 이미 로드된 경우 중복 로딩 방지
    if (tabsState.tabs.get().length > 0) {
      console.log('Tabs already loaded, skipping');
      return;
    }

    const savedTabs = storage.loadTabs();
    const savedActiveTab = storage.loadActiveTab();

    console.log('loadFromStorage Debug:', {
      savedTabs,
      savedActiveTab,
      savedTabsLength: savedTabs.length,
    });

    if (savedTabs.length > 0) {
      console.log('Loading saved tabs');
      tabsState.tabs.set(savedTabs);
      if (savedActiveTab && savedTabs.find((t) => t.id === savedActiveTab)) {
        tabsState.activeTabId.set(savedActiveTab);
        tabActions.selectTab(savedActiveTab);
      } else {
        tabActions.selectTab(savedTabs[0].id);
      }
    } else {
      console.log('No saved tabs, creating initial tab');
      // 저장된 데이터가 없으면 초기 탭 생성
      tabActions.addTab();
    }
  },
};

// 현재 활성 탭 가져오기
export const getActiveTab = () => {
  const activeId = tabsState.activeTabId.get();
  const tabs = tabsState.tabs.get();

  console.log('getActiveTab Debug:', {
    activeId,
    tabs,
    tabsLength: tabs.length,
    foundTab: tabs.find((tab) => tab.id === activeId),
  });

  return tabs.find((tab) => tab.id === activeId) || null;
};
