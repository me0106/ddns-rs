import { z } from "zod/v4";

const api = z.object({
  method: z.literal("api"),
  endpoint: z.url({ message: "API Endpoint must be a valid URL." }).optional(),
});
const nic = z.object({
  method: z.literal("nic"),
  interface: z.string().trim().optional(),
});
const cmd = z.object({
  method: z.literal("cmd"),
  command: z.string().trim().optional(),
});

const methodSchema = z.discriminatedUnion("method", [api, nic, cmd]);

const ipAddressSchema = z
  .object({
    enabled: z.boolean(),
  })
  .and(methodSchema)
  .check(({ value: data, issues }) => {
    if (data.enabled) {
      if (data.method === "api" && !data.endpoint) {
        issues.push({
          code: "custom",
          message: "API Endpoint is required. ",
          input: data,
          path: ["endpoint"],
        });
        return;
      }
      if (data.method === "cmd" && !data.command) {
        issues.push({
          code: "custom",
          message: "Commands are required.",
          input: data,
          path: ["command"],
        });
        return;
      }
      if (data.method === "nic" && !data.interface) {
        issues.push({
          code: "custom",
          message: "Network Interface is required.",
          input: data,
          path: ["interface"],
        });
      }
    }
  });
// Define Zod Schema for validation
export const networkConfigSchema = z.object({
  name: z.string().trim().nonempty({ error: "Config Name is required." }),
  interval: z.number().min(30),
  domain: z.string().trim().nonempty({ error: "Domain is required." }),
  subdomain: z.string().trim().nonempty({ error: "Subdomain is required." }),
  ipv4: ipAddressSchema,
  ipv6: ipAddressSchema.clone(),
  provider: z.string().nonempty({ error: "Provider is required." }),
  webhook: z.string().optional(),
});

export type DdnsConfig = z.infer<typeof networkConfigSchema>;
export type NetCard = {
  name: string;
  addrs: string[];
};
