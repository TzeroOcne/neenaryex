export type LoginRequest = (username: string, password: string) => Promise<number>;
export type CWDRequest = () => Promise<string>;