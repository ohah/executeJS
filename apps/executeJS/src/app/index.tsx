import React from 'react';
import { Providers } from './providers';
import { EditorPage } from '../pages/editor/EditorPage';

export const App: React.FC = () => {
  return (
    <Providers>
      <EditorPage />
    </Providers>
  );
};
