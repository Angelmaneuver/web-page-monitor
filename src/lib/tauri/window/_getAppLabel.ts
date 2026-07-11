import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

async function getAppLabel(): Promise<string> {
  await waitForBackendReady();

  try {
    return await invoke<string>('get_app_label');
  } catch {
    return '';
  }
}

async function waitForBackendReady(): Promise<void> {
  let unlisten: (() => void) | undefined;
  let settled = false;

  const cleanup = () => {
    if (unlisten) {
      unlisten();
      unlisten = undefined;
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

    void listen<string>('backend-ready', () => {
      finish();
    })
      .then((fn) => {
        unlisten = fn;
      })
      .catch(() => {
        finish();
      });

    window.setTimeout(() => {
      finish();
    }, 2000);
  });
}

export default getAppLabel;
