import React from 'react';
import { Button as RadixButton } from '@radix-ui/themes';

interface ButtonProps extends React.ComponentProps<typeof RadixButton> {
  variant?: 'solid' | 'soft' | 'outline' | 'ghost';
  size?: '1' | '2' | '3' | '4';
  children: React.ReactNode;
}

export const Button: React.FC<ButtonProps> = ({
  variant = 'solid',
  size = '2',
  children,
  className = '',
  ...props
}) => {
  return (
    <RadixButton
      variant={variant}
      size={size}
      className={`${className}`}
      {...props}
    >
      {children}
    </RadixButton>
  );
};
