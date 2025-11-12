import React from 'react';

import { Providers } from './providers';
import { Router } from './router';

export const App: React.FC = () => {
  return (
    <Providers>
      <Router />
    </Providers>
  );
};
