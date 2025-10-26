import React from 'react';
import { Theme } from '@radix-ui/themes';

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers: React.FC<ProvidersProps> = ({ children }) => {
  return (
    <Theme
      appearance="dark"
      accentColor="blue"
      grayColor="gray"
      radius="medium"
      scaling="100%"
    >
      {children}
    </Theme>
  );
};
