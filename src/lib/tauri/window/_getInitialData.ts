import { invoke } from '@tauri-apps/api/core';

async function getInitialData(): Promise<Initial> {
  await waitForBackendReady();

  return invoke<Initial>('get_initial_data');
}

interface Initial {
  label: string;
  kiosk: boolean;
}

async function waitForBackendReady(): Promise<void> {
  let intervalId: number | undefined;
  let settled = false;

  const cleanup = () => {
    if (typeof intervalId === 'number') {
      clearInterval(intervalId);
      intervalId = undefined;
    }
  };

  await new Promise<void>((resolve) => {
    const finish = () => {
      if (settled) {
        return;
      }

      settled = true;
      cleanup();
      resolve();
    };

    isBackendReady(finish, () => {
      intervalId = setInterval(() => {
        isBackendReady(finish);
      }, 1 * 1000);
    });
  });
}

function isBackendReady(finish: () => void, toBeContinued?: () => void | undefined) {
  invoke<boolean>('is_backend_ready')
    .then((isReady) => {
      if (isReady) {
        finish();
      } else {
        if (toBeContinued) {
          toBeContinued();
        }
      }
    })
    .catch(() => finish());
}

export default getInitialData;

export type { Initial };
