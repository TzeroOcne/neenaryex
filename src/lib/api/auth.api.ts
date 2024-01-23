import { inTauri } from '$lib/const/tauri.const';
import { APIAuthExists } from '$lib/store/api.store';
import type { Invoke, LoginRequest } from '@types';

let invoke:Invoke;

APIAuthExists.set(false);
if (inTauri) (async () => {
  const tauriAPI = await import('@tauri-apps/api/tauri');
  invoke = tauriAPI.invoke;
  APIAuthExists.set(true);
})(); else APIAuthExists.set(true);

export const sendLogin:LoginRequest = (inTauri) ?
  async (username, password) => {
    invoke('login', { username, password });
    return 0;
  } :
  async () => {
    return 0;
  };
