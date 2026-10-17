import { invoke } from '@tauri-apps/api/core';
export type Asset = { id: string; originalFileName: string; fileCreatedAt?: string; type?: string; isFavorite?: boolean };
export type Album = { id: string; albumName: string; assetCount?: number };
export type SearchPage = { assets: { items: Asset[]; nextCursor?: string | null; nextPage?: string | null } };
export type Session = { url: string; apiKey: string };
export function connect(s: Session) { return invoke<{name:string;email:string}>('connect', { baseUrl:s.url, apiKey:s.apiKey }); }
export function assets(s: Session, cursor?: string, favorite = false) { return invoke<SearchPage>('search_assets', { baseUrl:s.url, apiKey:s.apiKey, cursor:cursor ?? null, favorite }); }
export function albums(s: Session) { return invoke<Album[]>('list_albums', { baseUrl:s.url, apiKey:s.apiKey }); }
export function thumbnail(s: Session, id: string) { return invoke<string>('thumbnail', { baseUrl:s.url, apiKey:s.apiKey, assetId:id }); }
