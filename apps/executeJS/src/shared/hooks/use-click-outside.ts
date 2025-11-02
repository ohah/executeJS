import { useEffect, RefObject } from 'react';

type ValidEvent = MouseEvent | TouchEvent;

export const useClickOutside = <T extends HTMLElement>(
  ref: RefObject<T | null>,
  onClickOutside: (event: ValidEvent) => void,
  events: string[] = ['mousedown', 'touchstart']
) => {
  useEffect(() => {
    function handleClickOutside(event: Event) {
      if (ref.current && !ref.current.contains(event.target as Node)) {
        onClickOutside(event as ValidEvent);
      }
    }

    events.forEach((event) =>
      document.addEventListener(event, handleClickOutside)
    );

    return () => {
      events.forEach((event) =>
        document.removeEventListener(event, handleClickOutside)
      );
    };
  }, [ref]);
};
