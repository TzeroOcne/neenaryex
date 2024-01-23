import { browser } from '$app/environment';
import { inTauri } from '$lib/const/tauri.const';
import { APISystemExists } from '$lib/store/api.store';
import type { CWDRequest, Invoke } from '@types';

let invoke:Invoke;

APISystemExists.set(false);
if (inTauri) (async () => {
  const tauriAPI = await import('@tauri-apps/api/tauri');
  invoke = tauriAPI.invoke;
  APISystemExists.set(true);
})(); else APISystemExists.set(true);

export const getCWD:CWDRequest = (inTauri) ?
  async () => {
    return await invoke('get_cd');
  } :
  async () => {
    return (browser) ? 'in browser' : 'in server';
  };
