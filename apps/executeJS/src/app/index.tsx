import React from 'react';
import { Providers } from './providers';
import { EditorPage } from '../pages/playground';

export const App: React.FC = () => {
  return (
    <Providers>
      <EditorPage />
    </Providers>
  );
};
