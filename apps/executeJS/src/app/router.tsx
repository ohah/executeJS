import { BrowserRouter, Routes, Route } from 'react-router-dom';

import { PlaygroundPage } from '@/pages/playground';
import { SettingsPage } from '@/pages/settings';

export const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<PlaygroundPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </BrowserRouter>
  );
};
