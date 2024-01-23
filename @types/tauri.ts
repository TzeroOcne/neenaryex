import type { InvokeArgs } from '@tauri-apps/api/tauri';

export type Invoke = <T>(name:string, args?: InvokeArgs) => Promise<T>;