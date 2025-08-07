import tencent from "@/assets/tencent-icon.svg";
import cloudflare from "@/assets/cloudflare-icon.svg";
import type { Kind } from "@/types/provider.ts";

export const getLogo = (kind: Kind) => {
  if (kind === "tencent") {
    return tencent;
  }
  if (kind == "cloudflare") {
    return cloudflare;
  }
};
