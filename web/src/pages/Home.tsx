import { Button } from "@/components/ui/button.tsx";
import { navigate } from "wouter/use-browser-location";
import {
  Activity,
  AlertCircle,
  CheckCircle,
  Clock,
  Edit,
  Globe,
  Monitor,
  Play,
  PlusIcon,
  RotateCwIcon,
  TrashIcon,
} from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { useEffect, useState } from "react";
import { deleteDnsConfig, getDnsStates, runDdns } from "@/lib/api.ts";
import { Badge } from "@/components/ui/badge.tsx";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip.tsx";
import * as React from "react";
import { getLogo } from "@/lib/logo.ts";
import type { Kind } from "@/types/provider.ts";
import { toast } from "@/lib/toast.ts";
import { ConfirmationDialog } from "@/components/ConfirmationDialog.tsx";

export interface DnsState {
  name: string;
  domain: string;
  subdomain: string;
  kind: Kind | undefined;
  ipv4: {
    state: "success" | "failure" | "pending" | "disabled";
    timestamp: number | undefined;
    addr: string | undefined;
    message: string | undefined;
  };
  ipv6: {
    state: "success" | "failure" | "pending" | "disabled";
    timestamp: number | undefined;
    addr: string | undefined;
    message: string | undefined;
  };
}
type State = DnsState["ipv4"]["state"];

function Home() {
  const [states, setStates] = useState<DnsState[]>([]);
  const [loading, setLoading] = useState(true);

  const reload = async () => {
    setLoading(true);
    try {
      setStates(await getDnsStates());
    } catch (err) {
      console.error(err);
    }
    setLoading(false);
  };

  useEffect(() => {
    reload().then();
  }, [setStates, setLoading]);

  return (
    <div className="size-full flex flex-col">
      <div className="w-3/4 mx-auto flex justify-end my-6">
        <Button variant="outline" onClick={() => navigate("/config")}>
          <PlusIcon />
          添加 DDNS
        </Button>
      </div>
      <div className="w-3/4 mx-auto grid grid-cols-6 gap-4">
        <div className="col-span-1 rounded-lg border bg-card p-4">
          <div className="flex items-center space-x-2">
            <Monitor className="h-4 w-4 text-muted-foreground" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">Total</p>
              <p className="text-lg font-bold">{states.length}</p>
            </div>
          </div>
        </div>

        <div className="col-span-1 rounded-lg border bg-card p-4">
          <div className="flex items-center space-x-2">
            <CheckCircle className="h-4 w-4 text-green-600" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">
                Active
              </p>
              <p className="text-lg font-bold text-green-600">
                {
                  states.filter(
                    (c) =>
                      c.ipv4.state === "success" || c.ipv6.state === "success",
                  ).length
                }
              </p>
            </div>
          </div>
        </div>

        <div className="col-span-1 rounded-lg border bg-card p-4">
          <div className="flex items-center space-x-2">
            <AlertCircle className="h-4 w-4 text-yellow-600" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">
                Pending
              </p>
              <p className="text-lg font-bold text-yellow-600">
                {
                  states.filter(
                    (c) =>
                      c.ipv4.state === "pending" || c.ipv6.state === "pending",
                  ).length
                }
              </p>
            </div>
          </div>
        </div>

        <div className="col-span-1 rounded-lg border bg-card p-4">
          <div className="flex items-center space-x-2">
            <AlertCircle className="h-4 w-4 text-red-600" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">Error</p>
              <p className="text-lg font-bold text-red-600">
                {
                  states.filter(
                    (c) =>
                      c.ipv4.state === "failure" || c.ipv6.state === "failure",
                  ).length
                }
              </p>
            </div>
          </div>
        </div>

        <div className="col-span-2 rounded-lg border bg-card p-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <Activity className="h-4 w-4 text-muted-foreground" />
              <div>
                <p className="text-xs font-medium text-muted-foreground">
                  Last Sync
                </p>
                <p className="text-lg font-bold">
                  {Math.max(
                    ...states.map((c) =>
                      Math.max(c.ipv4.timestamp || 0, c.ipv6.timestamp || 0),
                    ),
                  ) > 0
                    ? formatTimeAgo(
                        Math.max(
                          ...states.map((c) =>
                            Math.max(
                              c.ipv4.timestamp || 0,
                              c.ipv6.timestamp || 0,
                            ),
                          ),
                        ),
                      )
                    : "Never"}
                </p>
              </div>
            </div>
            <RotateCwIcon
              className="h-4 w-4 text-muted-foreground cursor-pointer"
              onClick={reload}
            />
          </div>
        </div>
      </div>
      <div className="w-3/4 mx-auto grow min-h-0 mb-15">
        <div className="mx-auto size-full">
          {loading ? (
            <div className="text-center text-gray-500 py-10">加载中...</div>
          ) : states.length === 0 ? (
            <div className="text-center text-gray-500 py-10">
              暂无 DNS 配置，点击 "添加 DDNS" 开始配置。
            </div>
          ) : (
            <Table states={states} reload={reload} />
          )}
        </div>
      </div>
    </div>
  );
}

function Table({
  states,
  reload,
  ...props
}: React.DetailedHTMLProps<
  React.HTMLAttributes<HTMLDivElement>,
  HTMLDivElement
> & { states: DnsState[]; reload: () => void }) {
  const [deletingConfig, setDeletingConfig] = useState<string | undefined>();
  const handleDelete = async () => {
    if (!deletingConfig) {
      return;
    }
    try {
      await deleteDnsConfig(deletingConfig);
    } catch (e) {
      console.error(e);
      return;
    }
    toast.success("Delete DdnsConfig Completed.");
    setDeletingConfig(undefined);
    reload();
  };
  return (
    <div
      className="flex flex-col rounded-lg border bg-card w-full max-h-full my-6"
      {...props}
    >
      <div className="border-b px-6 py-4">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold">DNS Configurations</h2>
          <div className="text-sm text-muted-foreground">
            {states.length} configurations
          </div>
        </div>
      </div>

      <ScrollArea className="grow min-h-0 overflow-auto">
        <div className={"divide-y"}>
          {states.map((state) => (
            <StateItem
              key={state.name}
              state={state}
              onClickDelete={() => setDeletingConfig(state.name)}
            />
          ))}
        </div>
      </ScrollArea>
      <ConfirmationDialog
        open={deletingConfig != null}
        onConfirm={handleDelete}
        onCancel={() => setDeletingConfig(undefined)}
        title="确认删除"
        description={
          <p>
            您确定要删除此 Ddns
            <span className="text-red-500 mx-2">{deletingConfig}</span>
            配置吗？此操作不可撤销。
          </p>
        }
      />
    </div>
  );
}

function StateItem({
  state,
  onClickDelete,
}: {
  state: DnsState;
  onClickDelete: () => void;
}) {
  const handleSync = async () => {
    try {
      await runDdns(state.name);
    } catch (e) {
      console.error(e);
      return;
    }
    toast.success("Sync completed.");
  };

  return (
    <div key={state.name} className="p-4 hover:bg-muted/50 transition-colors">
      <div className="grid grid-cols-11 gap-4 items-center">
        <div className="col-span-2">
          <div className="flex items-center space-x-3">
            {state.kind ? (
              <img
                className="h-8 w-8"
                src={getLogo(state.kind)}
                alt={state.kind}
              />
            ) : (
              <Globe className="h-4 w-4" />
            )}
            <div className="min-w-0">
              <p className="text-sm font-medium truncate">{state.name}</p>
              <p className="text-xs text-muted-foreground truncate">
                {state.subdomain}.{state.domain}
              </p>
            </div>
          </div>
        </div>

        <div className="col-span-3">
          <div className="flex items-center space-x-2">
            {getStatusIcon(state.ipv4.state)}
            <div className="min-w-0 flex-1">
              <div className="flex items-center space-x-2">
                <span className="text-xs font-medium">IPv4:</span>
                {state.ipv4.state === "disabled" ? (
                  <span className="text-xs text-muted-foreground">
                    Disabled
                  </span>
                ) : (
                  <Badge className={`${getBadgeClass(state.ipv4.state)}`}>
                    {state.ipv4.addr || "N/A"}
                  </Badge>
                )}
              </div>
              <Tooltip>
                <TooltipTrigger asChild>
                  <p className="text-xs text-muted-foreground truncate mt-0.5">
                    {state.ipv4.message}
                  </p>
                </TooltipTrigger>
                <TooltipContent className="w-96">
                  <p>{state.ipv4.message}</p>
                </TooltipContent>
              </Tooltip>
            </div>
          </div>
        </div>

        <div className="col-span-4">
          <div className="flex items-center space-x-2">
            {getStatusIcon(state.ipv6.state)}
            <div className="min-w-0 flex-1">
              <div className="flex items-center space-x-2">
                <span className="text-xs font-medium">IPv6:</span>
                {state.ipv6.state === "disabled" ? (
                  <span className="text-xs text-muted-foreground">
                    Disabled
                  </span>
                ) : (
                  <Badge className={`${getBadgeClass(state.ipv6.state)}`}>
                    {state.ipv6.addr ?? "N/A"}
                  </Badge>
                )}
              </div>
              <Tooltip>
                <TooltipTrigger asChild>
                  <p className="text-xs text-muted-foreground truncate mt-0.5">
                    {state.ipv6.message}
                  </p>
                </TooltipTrigger>
                <TooltipContent>
                  <p>{state.ipv6.message}</p>
                </TooltipContent>
              </Tooltip>
            </div>
          </div>
        </div>

        <div className="col-span-2 flex items-center justify-between">
          <div className="text-center">
            <p className="text-xs font-medium">
              {formatTimeAgo(
                Math.max(state.ipv4.timestamp || 0, state.ipv6.timestamp || 0),
              )}
            </p>
            <p className="text-xs text-muted-foreground">ago</p>
          </div>

          <div className="flex items-center space-x-1">
            <Button
              className="size-7 hover:bg-gray-300"
              variant="ghost"
              title="Sync Now"
              onClick={handleSync}
            >
              <Play className="size-3" />
            </Button>
            <Button
              title="Edit"
              className="size-7 hover:bg-gray-300"
              variant="ghost"
              onClick={() => navigate(`/config/${state.name}`)}
            >
              <Edit className="size-3" />
            </Button>
            <Button
              title="Delete"
              className="left-5 size-7 hover:bg-gray-300"
              variant="ghost"
              onClick={onClickDelete}
            >
              <TrashIcon className="text-red-500" />
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

const formatTimeAgo = (timestamp: number) => {
  if (!timestamp) return "Never";
  const now = Date.now();
  const diff = now - timestamp * 1000;
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return "Now";
  if (minutes < 60) return `${minutes}m`;
  if (hours < 24) return `${hours}h`;
  return `${days}d`;
};

const getStatusIcon = (status: State) => {
  switch (status) {
    case "success":
      return <CheckCircle className="h-5 w-5 text-green-600" />;
    case "pending":
      return <AlertCircle className="h-5 w-5 text-yellow-600" />;
    case "failure":
      return <AlertCircle className="h-5 w-5 text-red-600" />;
    case "disabled":
      return <Clock className="h-5 w-5 text-gray-400" />;
    default:
      return <Clock className="h-5 w-5 text-gray-400" />;
  }
};

const getBadgeClass = (status: State) => {
  switch (status) {
    case "success":
      return "bg-green-50 text-green-700 border-green-200";
    case "pending":
      return "bg-yellow-50 text-yellow-700 border-yellow-200";
    case "failure":
      return "bg-red-50 text-red-700 border-red-200";
    case "disabled":
      return "bg-gray-50 text-gray-500 border-gray-200";
    default:
      return "bg-gray-50 text-gray-500 border-gray-200";
  }
};

export { Home };
