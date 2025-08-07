import { toast } from "@/lib/toast";
import { navigate } from "wouter/use-browser-location";
import { token } from "@/lib/storage.ts";
import type { Webhook } from "@/pages/Webhook.tsx";
import type { DnsState } from "@/pages/Home.tsx";
import type { DdnsConfig, NetCard } from "@/types/config.ts";
import type { ProviderConfig } from "@/types/provider.ts";
import type { LogItem } from "@/pages/Log.tsx";

export interface ApiResult<T> {
  code: number;
  message: string;
  data: T;
}

export async function initialize(req: {
  user: { username: string; password: string };
}): Promise<void> {
  return await request("/api/sys/init", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
  });
}

export interface SysInfo {
  configPath: string;
  initialized: boolean;
  version: string;
  commitId: string;
}

export async function system() {
  return await request<SysInfo>("/api/sys/info");
}

export async function login(req: {
  username: string;
  password: string;
}): Promise<{ token: string }> {
  return await request("/api/user/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(req),
  });
}

export async function getDnsStates() {
  return await request<DnsState[]>("/api/dns/state/list");
}

export async function getDnsConfig(name: string) {
  return await request<DdnsConfig | undefined>(`/api/dns/${name}`);
}

export async function getNicList(family: "ipv4" | "ipv6"): Promise<NetCard[]> {
  return await request(`/api/sys/addrs?family=${family}`);
}

export async function addDdnsConfig(config: DdnsConfig): Promise<void> {
  await request<void>("/api/dns", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(config),
  });
}

export async function updateDdnsConfig(config: DdnsConfig): Promise<void> {
  await request(`/api/dns/${config.name}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(config),
  });
}

export async function deleteDnsConfig(name: string): Promise<void> {
  await request(`/api/dns/${name}`, {
    method: "DELETE",
  });
}

export async function runDdns(name: string): Promise<void> {
  return await request<void>(`/api/dns/run/${name}`, {
    method: "PUT",
  });
}

export async function addProvider(provider: ProviderConfig): Promise<void> {
  await request<void>(`/api/provider`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(provider),
  });
}
export async function updateProvider(provider: ProviderConfig): Promise<void> {
  await request<void>(`/api/provider/${provider.name}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(provider),
  });
}

export async function deleteProvider(name: string): Promise<void> {
  await request<void>(`/api/provider/${name}`, {
    method: "DELETE",
  });
}

export async function getProviders(): Promise<ProviderConfig[]> {
  return await request<ProviderConfig[]>(`/api/provider/list`, {});
}

export async function getWebhooks(): Promise<Webhook[]> {
  return await request<Webhook[]>(`/api/webhook/list`);
}

export async function addWebhook(webhook: Webhook): Promise<void> {
  return await request(`/api/webhook`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(webhook),
  });
}

export async function updateWebhook(webhook: Webhook): Promise<void> {
  return await request(`/api/webhook/${webhook.name}`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(webhook),
  });
}

export async function deleteWebhook(name: string): Promise<void> {
  return await request(`/api/webhook/${name}`, {
    method: "DELETE",
  });
}
export async function testWebhook(hook: Webhook): Promise<{ data: string }> {
  return await request(`/api/webhook/run/test`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(hook),
  });
}

export async function log() {
  return await request<LogItem[]>(`/api/sys/log`);
}

async function request<T>(
  input: RequestInfo | URL,
  init?: RequestInit,
): Promise<T> {
  const rsp = await fetch(input, {
    ...init,
    headers: {
      ...init?.headers,
      Authorization: `Bearer ${token.get()}`,
    },
  });
  if (!rsp.ok) {
    toast.error(rsp.statusText);
    throw new Error(rsp.statusText);
  }
  const res: ApiResult<T> = await rsp.json();
  if (res.code == 0) {
    return res.data;
  }

  switch (res.code) {
    case 1000:
      navigate("/initialize");
      toast.error(res.message);
      break;
    case 1200:
      token.remove();
      navigate("/login");
      break;
    default:
      toast.error(res.message);
  }
  throw res;
}
