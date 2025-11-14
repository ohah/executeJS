import React, { useRef, useState } from 'react';
import { getCurrentWindow, Window } from '@tauri-apps/api/window';

type SettingsTab =
  | 'general'
  | 'build'
  | 'formatting'
  | 'appearance'
  | 'ai'
  | 'advanced';

const TAB_TITLES: Record<SettingsTab, string> = {
  general: 'General',
  build: 'Build',
  formatting: 'Formatting',
  appearance: 'Appearance',
  ai: 'AI',
  advanced: 'Advanced',
};

const TABS: { id: SettingsTab; label: string }[] = [
  { id: 'general', label: 'General' },
  { id: 'build', label: 'Build' },
  { id: 'formatting', label: 'Formatting' },
  { id: 'appearance', label: 'Appearance' },
  { id: 'ai', label: 'AI' },
  { id: 'advanced', label: 'Advanced' },
];

export const SettingsPage: React.FC = () => {
  const windowRef = useRef<Window>(getCurrentWindow());

  const [activeTab, setActiveTab] = useState<SettingsTab>('general');

  const updateWindowTitle = async (tabId: SettingsTab) => {
    try {
      await windowRef.current.setTitle(TAB_TITLES[tabId]);
    } catch (error) {
      console.error('창 제목 변경 실패:', error);
    }
  };

  const handleTabClick = async (tabId: SettingsTab) => {
    setActiveTab(tabId);

    await updateWindowTitle(tabId);
  };

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* 사이드바 */}
      <div className="w-48 border-r border-gray-700 bg-gray-800">
        <div className="p-4 border-b border-gray-700">
          <h1 className="text-xl font-bold">Settings</h1>
        </div>
        <nav className="p-2">
          {TABS.map((tab) => (
            <button
              key={tab.id}
              onClick={() => handleTabClick(tab.id)}
              className={`w-full text-left px-4 py-2 rounded-md mb-1 transition-colors ${
                activeTab === tab.id
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-300 hover:bg-gray-700'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {/* 컨텐츠 영역 */}
      <div className="flex-1 overflow-y-auto p-8">
        {activeTab === 'general' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">General</h2>
            <p className="text-gray-400">일반 설정 옵션이 여기에 추가됩니다.</p>
          </div>
        )}

        {activeTab === 'build' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">Build</h2>
            <p className="text-gray-400">빌드 설정 옵션이 여기에 추가됩니다.</p>
          </div>
        )}

        {activeTab === 'formatting' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">Formatting</h2>
            <p className="text-gray-400">
              포맷팅 설정 옵션이 여기에 추가됩니다.
            </p>
          </div>
        )}

        {activeTab === 'appearance' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">Appearance</h2>
            <p className="text-gray-400">외관 설정 옵션이 여기에 추가됩니다.</p>
          </div>
        )}

        {activeTab === 'ai' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">AI</h2>
            <p className="text-gray-400">AI 설정 옵션이 여기에 추가됩니다.</p>
          </div>
        )}

        {activeTab === 'advanced' && (
          <div>
            <h2 className="text-2xl font-bold mb-6">Advanced</h2>
            <p className="text-gray-400">고급 설정 옵션이 여기에 추가됩니다.</p>
          </div>
        )}
      </div>
    </div>
  );
};
