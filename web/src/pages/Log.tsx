import { useState, useEffect, useMemo, useRef, type RefObject } from "react";
import { Info, AlertTriangle, XCircle, Bug } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { log } from "@/lib/api.ts";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@/components/ui/tooltip.tsx";

export interface LogItem {
  timestamp: number;
  config: string | undefined;
  level: "INFO" | "WARN" | "ERROR" | "DEBUG";
  module: string;
  message: string;
}
type Level = LogItem["level"];

const Log = () => {
  const [logs, setLogs] = useState<LogItem[]>([]);
  const [filterLevel, setFilterLevel] = useState<Level | undefined>();

  useEffect(() => {
    log().then(setLogs);
  }, []);

  // 过滤日志
  const filteredLogs = useMemo(() => {
    return logs.filter(
      (log) => filterLevel === undefined || log.level === filterLevel,
    );
  }, [logs, filterLevel]);

  return (
    <div className="flex flex-col h-full w-2/3 mx-auto p-4">
      <div className="w-full flex justify-end shrink-0">
        <Select onValueChange={(val) => setFilterLevel(val as Level)}>
          <SelectTrigger className="bg-white w-40">
            <SelectValue placeholder="Select Log Level" />
          </SelectTrigger>
          <SelectContent className="bg-white w-40">
            {(["INFO", "WARN", "DEBUG", "ERROR"] as Level[]).map((level) => (
              <SelectItem value={level}>
                {getLevelIcon(level)}
                <span>{level}</span>
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      <div className="flex flex-col rounded-lg border bg-card w-full min-h-0 my-2">
        <div className="flex shrink-0 border-b px-8 items-center h-16 overflow-hidden bg-accent">
          <div className="w-44 shrink-0">时间</div>
          <div className="w-40 shrink-0">配置</div>
          <div className="w-24 shrink-0">级别</div>
          <div className="w-64 shrink-0">模块</div>
          <div className="grow shrink-0">消息</div>
        </div>
        <ScrollArea className="grow w-full min-h-0 overflow-auto">
          <div className="divide-y">
            {filteredLogs.length == 0 ? (
              <div className="text-center p-4 font-mono text-sm text-muted-foreground">
                <p className="font-mono">暂无符合条件的日志</p>
              </div>
            ) : (
              filteredLogs.map((log) => <LogItem item={log} />)
            )}
          </div>
        </ScrollArea>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between text-xs text-muted-foreground">
        <div>显示 {filteredLogs.length} 条日志</div>
        <div>总计 {logs.length} 条记录</div>
      </div>
    </div>
  );
};
const LogItem = ({ item: log }: { item: LogItem }) => {
  const ref = useRef<HTMLParagraphElement>(null);
  const [open, setOpen] = useState(false);
  const hasEllipsis = (ref: RefObject<HTMLParagraphElement | null>) => {
    return (
      (ref.current && ref.current.scrollWidth > ref.current.offsetWidth) ??
      false
    );
  };
  return (
    <div
      id={`${log.timestamp}-${log.module}`}
      className="w-full flex px-6 py-2 hover:bg-muted/50 transition-colors"
    >
      <div className="w-44 shrink-0">
        <span className="p-2 align-middle font-mono text-xs text-muted-foreground">
          {formatTimestamp(log.timestamp)}
        </span>
      </div>
      <div className="w-40 shrink-0">
        <span className="inline-flex items-center rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-inset ring-gray-500/10">
          {log.config ?? "-"}
        </span>
      </div>
      <div className="w-24 shrink-0">
        <span
          className={`inline-flex items-center gap-x-1 rounded-full px-2.5 py-1 text-xs font-medium ${getLevelStyle(log.level)}`}
        >
          {getLevelIcon(log.level)}
          {log.level}
        </span>
      </div>
      <div className="w-64 shrink-0">
        <span className="inline-flex items-center rounded-md bg-gray-50 px-2 py-1 text-xs font-medium text-gray-600 ring-1 ring-inset ring-gray-500/10">
          {log.module}
        </span>
      </div>
      <div className="grow my-auto min-w-0">
        <Tooltip
          open={open}
          onOpenChange={(v) => setOpen(v && hasEllipsis(ref))}
        >
          <TooltipTrigger asChild>
            <p ref={ref} className="font-mono text-xs leading-relaxed truncate">
              {log.message}
            </p>
          </TooltipTrigger>
          <TooltipContent className="w-96 break-words font-mono text-xs ">
            {log.message}
          </TooltipContent>
        </Tooltip>
      </div>
    </div>
  );
};
// 获取日志级别样式
const getLevelStyle = (level: Level) => {
  const styles = {
    INFO: "text-blue-600 bg-blue-50",
    WARN: "text-amber-600 bg-amber-50",
    ERROR: "text-red-600 bg-red-50",
    DEBUG: "text-gray-600 bg-gray-50",
  };
  return styles[level];
};
const formatTimestamp = (timestamp: number) => {
  return new Date(timestamp * 1000).toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
};

// 获取日志级别图标
const getLevelIcon = (level: Level) => {
  const icons = {
    INFO: <Info className="w-4 h-4" />,
    WARN: <AlertTriangle className="w-4 h-4" />,
    ERROR: <XCircle className="w-4 h-4" />,
    DEBUG: <Bug className="w-4 h-4" />,
  };
  return icons[level];
};
export default Log;
