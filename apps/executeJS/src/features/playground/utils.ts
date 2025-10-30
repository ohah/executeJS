import { PLAYGROUND_STORAGE_KEY } from './const';
import { Playground } from './store';

export const getInitialCode = (playgroundId: string): string => {
  try {
    const playgroundStorage = localStorage.getItem(PLAYGROUND_STORAGE_KEY);

    if (playgroundStorage) {
      const parsed = JSON.parse(playgroundStorage);
      const playgrounds = new Map(parsed?.state?.playgrounds?.value);
      const playground = playgrounds.get(playgroundId) as Playground;
      const code = playground?.result?.code;

      if (code) {
        console.log('result from playgroundStorage:', code);

        return code;
      }
    }
  } catch (error) {
    console.error('error from playgroundStorage:', error);
  }

  return 'console.log("Hello, ExecuteJS!");';
};
