import React from 'react';
import { Box } from '@radix-ui/themes';

interface PanelProps {
  children: React.ReactNode;
  className?: string;
}

export const Panel: React.FC<PanelProps> = ({
  children,
  className = '',
  ...props
}) => {
  return (
    <Box
      className={`bg-gray-2 border border-gray-6 rounded-lg p-4 ${className}`}
      {...props}
    >
      {children}
    </Box>
  );
};
