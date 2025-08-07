import { z } from "zod/v4";

const tencentSchema = z.object({
  name: z.string().nonempty("配置名不能为空"),
  kind: z.literal("tencent"),
  secretId: z.string().nonempty("Secret ID 不能为空"),
  secretKey: z.string().nonempty("Secret Key 不能为空"),
});

const cloudflareSchema = z.object({
  name: z.string().nonempty("配置名不能为空"),
  kind: z.literal("cloudflare"),
  apiKey: z.string().nonempty("apiKey 不能为空"),
});

export const schema = z.discriminatedUnion("kind", [
  tencentSchema,
  cloudflareSchema,
]);

export type Kind = z.infer<typeof schema>["kind"];

export type ProviderConfig = z.infer<typeof schema>;

export const Kinds: Kind[] = ["tencent", "cloudflare"];

export const getInputConfig = (kind: Kind) => {
  if (kind === "tencent") {
    return [
      { name: "secretId", label: "Secret ID", type: "text" },
      { name: "secretKey", label: "Secret Key", type: "password" },
    ] as const;
  }
  if (kind === "cloudflare") {
    return [{ name: "apiKey", label: "apiKey", type: "password" }] as const;
  }
  return [];
};
