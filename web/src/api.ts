import axios from 'axios';

const BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:8080';

const http = axios.create({ baseURL: BASE });

export interface CreateLinkResponse {
  code: string;
  url: string;
  short_url: string;
}

export interface LinkStats {
  code: string;
  url: string;
  clicks: number;
  created_at: string;
}

export async function createLink(url: string): Promise<CreateLinkResponse> {
  const { data } = await http.post<CreateLinkResponse>('/links', { url });
  return data;
}

export async function fetchStats(code: string): Promise<LinkStats> {
  const { data } = await http.get<LinkStats>(`/links/${code}/stats`);
  return data;
}
