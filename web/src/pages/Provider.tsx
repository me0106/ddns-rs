import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Edit, PlusIcon, Trash2 } from "lucide-react";
import {
  addProvider,
  deleteProvider,
  getProviders,
  updateProvider,
} from "@/lib/api.ts";
import { toast } from "@/lib/toast";
import { getLogo } from "@/lib/logo.ts";
import {
  getInputConfig,
  Kinds,
  type ProviderConfig,
  schema,
} from "@/types/provider.ts";
import { ConfirmationDialog } from "@/components/ConfirmationDialog.tsx";
import { Badge } from "@/components/ui/badge.tsx";

/**
 * Main Provider component to manage and display DNS provider.
 */
export default function Provider() {
  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [editingOpen, setEditingOpen] = useState(false);
  const [editingProvider, setEditingProvider] = useState<
    ProviderConfig | undefined
  >();
  const [deletingProvider, setDeletingProvider] = useState<
    ProviderConfig | undefined
  >();

  const reload = async () => {
    setProviders(await getProviders());
  };

  const handleDelete = async () => {
    if (deletingProvider?.name) {
      try {
        await deleteProvider(deletingProvider.name);
      } catch (err) {
        console.error(err);
        return;
      }
    }
    setDeletingProvider(undefined);
    await reload();
  };

  useEffect(() => {
    reload().then();
  }, []);

  return (
    <div className="size-full flex flex-col overflow-hidden">
      <div className="mx-auto w-3/5 shrink-0 my-6 px-4 flex justify-end">
        <Button variant="outline" onClick={() => setEditingOpen(true)}>
          <PlusIcon />
          添加 Provider
        </Button>
      </div>
      <ScrollArea className="grow w-3/5 mx-auto min-h-0 overflow-hidden pb-8">
        <div className="w-full mx-auto px-4 space-y-4">
          {providers.length === 0 ? (
            <div className="text-center text-gray-500 dark:text-gray-400 py-10">
              暂无 Provider，点击 "添加 Provider" 开始配置。
            </div>
          ) : (
            providers.map((provider) => (
              <ItemContent
                key={provider.name}
                provider={provider}
                onClickEditing={() => {
                  setEditingProvider(provider);
                  setEditingOpen(true);
                }}
                onDelete={() => setDeletingProvider(provider)}
              />
            ))
          )}
        </div>
      </ScrollArea>

      <DnsProviderDialog
        open={editingOpen}
        onOpenChange={setEditingOpen}
        onSaveCompleted={async () => {
          setEditingOpen(false);
          setEditingProvider(undefined);
          await reload();
        }}
        config={editingProvider}
      />

      <ConfirmationDialog
        open={deletingProvider != null}
        onConfirm={handleDelete}
        onCancel={() => setDeletingProvider(undefined)}
        title="确认删除"
        description={`您确定要删除此 Provider [${deletingProvider?.name}] 配置吗？此操作不可撤销。`}
      />
    </div>
  );
}

function ItemContent({
  provider,
  onClickEditing,
  onDelete,
}: {
  provider: ProviderConfig;
  onClickEditing: () => void;
  onDelete: () => void;
}) {
  const masking = (value: string) => {
    return (
      value.substring(0, 2) +
      "*".repeat(value.length - value.length / 2) +
      value.substring(value.length - 2, value.length)
    );
  };

  return (
    <div className="flex flex-row items-center w-full p-4 border border-gray-200 rounded-lg shadow-sm bg-white hover:shadow-md transition-shadow duration-200">
      <img
        className="size-10 rounded-full object-cover mr-4 mb-2"
        src={getLogo(provider.kind)}
        alt={`${provider.kind} logo`}
      />
      <div className="flex-grow overflow-hidden">
        <div className="font-semibold text-lg text-gray-900 ">
          {provider.name}
        </div>
        <div className="text-muted-foreground text-sm flex flex-col sm:flex-row sm:space-x-2 mt-1">
          {getInputConfig(provider.kind).map((config) => {
            return (
              <div key={config.name}>
                <span className="mr-1">{config.label}:</span>
                <Badge variant="secondary">
                  {masking((provider as Record<string, string>)[config.name])}
                </Badge>
              </div>
            );
          })}
        </div>
      </div>
      <div className="flex flex-row gap-3 items-center">
        <Edit
          className="size-5 text-blue-500 cursor-pointer hover:text-blue-600 transition-colors duration-200"
          onClick={onClickEditing}
        />
        <Trash2
          className="size-5 text-red-500 cursor-pointer hover:text-red-600 transition-colors duration-200"
          onClick={onDelete}
        />
      </div>
    </div>
  );
}

export function DnsProviderDialog({
  config,
  open,
  onOpenChange,
  onSaveCompleted,
}: {
  config?: ProviderConfig;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSaveCompleted?: (provider: ProviderConfig) => Promise<void>;
}) {
  const form = useForm<ProviderConfig>({
    mode: "all",
    resolver: zodResolver(schema),
    defaultValues: config ?? {
      name: "",
      kind: "tencent",
      secretId: "",
      secretKey: "",
    },
  });
  useEffect(() => form.reset(config), [config, form]);

  const kind = form.watch("kind");

  const onSubmit = async (data: ProviderConfig) => {
    try {
      await (config ? updateProvider(data) : addProvider(data));
      toast.success("Provider added successfully.");
      await onSaveCompleted?.(data);
    } catch (e) {
      console.error(e);
    }
  };
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px] bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
        <DialogHeader>
          <DialogTitle className="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {config ? "编辑 Provider" : "添加 Provider"}
          </DialogTitle>
          <DialogDescription className="text-gray-600 dark:text-gray-400">
            {config
              ? "修改您的 Provider 配置。"
              : "添加一个新的 DNS Provider 配置。"}
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <FormField
            control={form.control}
            name={"name"}
            render={({ field }) => (
              <FormItem>
                <FormLabel>配置名</FormLabel>
                <FormControl>
                  <Input placeholder="输入配置名" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="kind"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="text-gray-800 dark:text-gray-200">
                  Provider 类型
                </FormLabel>
                <Select onValueChange={field.onChange} value={field.value}>
                  <FormControl className="w-full">
                    <SelectTrigger>
                      <SelectValue placeholder="选择 DNS Provider" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md shadow-lg">
                    {Kinds.map((k) => (
                      <SelectItem
                        key={k}
                        value={k}
                        className="text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-700 capitalize"
                      >
                        {k}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />
          {getInputConfig(kind).map((inputConfig) => (
            <FormField
              key={inputConfig.name}
              control={form.control}
              name={inputConfig.name}
              render={({ field }) => (
                <FormItem>
                  <FormLabel>{inputConfig.label}</FormLabel>
                  <FormControl>
                    <Input
                      type={inputConfig.type}
                      placeholder={`输入 ${inputConfig.label}`}
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          ))}
          <DialogFooter className="flex flex-col sm:flex-row sm:justify-end sm:space-x-2 pt-4">
            <DialogClose asChild>
              <Button variant="outline">取消</Button>
            </DialogClose>
            <Button onClick={form.handleSubmit(onSubmit)}>保存</Button>
          </DialogFooter>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
