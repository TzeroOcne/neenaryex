import { browser } from '$app/environment';

export const inTauri = browser && '__TAURI__' in window;
