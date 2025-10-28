import React from 'react';
import { Providers } from './providers';
import { PlaygroundGroups } from '../pages/playground';

export const App: React.FC = () => {
  return (
    <Providers>
      <PlaygroundGroups />
    </Providers>
  );
};
