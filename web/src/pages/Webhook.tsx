import { Button } from "@/components/ui/button.tsx";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { Edit, PlusIcon, Trash2, WebhookIcon } from "lucide-react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Textarea } from "@/components/ui/textarea.tsx";
import { useState, useEffect } from "react";
import {
  deleteWebhook,
  getWebhooks,
  addWebhook,
  testWebhook,
  updateWebhook,
} from "@/lib/api.ts";
import { toast } from "@/lib/toast";
import { z } from "zod/v4";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form.tsx";
import { ConfirmationDialog } from "@/components/ConfirmationDialog.tsx";

const schema = z.object({
  name: z.string().nonempty("webhook name 不允许为空"),
  value: z.string().nonempty("webhook content 不允许为空"),
});

export type Webhook = z.infer<typeof schema>;

export default function App() {
  const [open, setOpen] = useState(false);
  const [editingWebhook, setEditingWebhook] = useState<Webhook | undefined>();
  const [deletingWebhook, setDeletingWebhook] = useState<string | undefined>();
  const [webhooks, setWebhooks] = useState<Webhook[]>([]);
  const reload = async () => {
    try {
      setWebhooks(await getWebhooks());
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    reload().then();
  }, []);

  const handleAddWebhook = () => {
    setEditingWebhook(undefined);
    setOpen(true);
  };

  const handleEditWebhook = async (webhook: Webhook) => {
    setEditingWebhook(webhook);
    setOpen(true);
  };

  const onSaveCompleted = async () => {
    toast.success("Save Webhook successfully");
    await reload();
    setOpen(false);
  };

  const showDeletingDialog = async (name: string) => {
    setDeletingWebhook(name);
  };

  return (
    <div className="size-full flex flex-col overflow-hidden">
      <EditingWebhookDialog
        open={open}
        onOpenChange={setOpen}
        webhook={editingWebhook}
        onSaveCompleted={onSaveCompleted}
      />

      <ConfirmationDialog
        title="确认删除"
        description={`您确定要删除此 Webhook [${deletingWebhook}] 配置吗？此操作不可撤销。`}
        open={deletingWebhook != null}
        onCancel={() => setDeletingWebhook(undefined)}
        onConfirm={async () => {
          if (deletingWebhook) {
            try {
              await deleteWebhook(deletingWebhook);
            } catch (err) {
              console.error(err);
              setDeletingWebhook(undefined);
              return;
            }
            await reload();
            setDeletingWebhook(undefined);
          }
        }}
      />

      <div className="mx-auto w-3/5 my-6 px-4 flex justify-end">
        <Button variant="outline" onClick={handleAddWebhook}>
          <PlusIcon />
          添加 Webhook
        </Button>
      </div>
      <ScrollArea className="w-3/5 mx-auto grow min-h-0 pb-8">
        <div className="w-full mx-auto px-4 space-y-4">
          {webhooks.length > 0 ? (
            webhooks.map((webhook, index) => (
              <ItemContent
                key={webhook.name + index}
                webhook={webhook}
                onClickEditing={handleEditWebhook}
                onClickDeleting={showDeletingDialog}
              />
            ))
          ) : (
            <p className="text-center text-gray-500 mt-10">
              暂无 Webhook，点击“添加 Webhook”创建。
            </p>
          )}
        </div>
      </ScrollArea>
    </div>
  );
}

function ItemContent({
  webhook,
  onClickEditing,
  onClickDeleting,
}: {
  webhook: Webhook;
  onClickEditing: (webhook: Webhook) => void;
  onClickDeleting: (webhookName: string) => void;
}) {
  return (
    <div className="flex flex-row w-full p-4 border border-gray-200 rounded-lg shadow-sm bg-white hover:shadow-md transition-shadow duration-200 items-center">
      <WebhookIcon className="size-8 rounded-full object-cover mr-4" />
      <div className="flex-grow overflow-hidden pr-4">
        <div className="font-semibold text-lg text-gray-800">
          {webhook.name}
        </div>
        <div className="text-muted-foreground text-sm flex flex-row space-x-2 mt-1">
          <div className="truncate">{webhook.value.split("\n")[0]}</div>
        </div>
      </div>
      <div className="flex flex-row gap-3 ml-auto">
        <Edit
          className="size-5 text-blue-500 cursor-pointer hover:text-blue-700 transition-colors duration-200"
          onClick={() => onClickEditing(webhook)}
        />
        <Trash2
          className="size-5 text-red-500 cursor-pointer hover:text-red-700 transition-colors duration-200"
          onClick={() => onClickDeleting(webhook.name)}
        />
      </div>
    </div>
  );
}

const placeholder = `输入原始Http请求 sample:

GET https://baidu.com/api/v1/webhooks
Content-Type: application/json

{
    "name":"111"
}`;

interface EditProps {
  webhook?: Webhook;

  open: boolean;

  onOpenChange(open: boolean): void;

  onSaveCompleted(): Promise<void>;
}

export function EditingWebhookDialog({
  open,
  onOpenChange,
  webhook,
  onSaveCompleted,
}: EditProps) {
  const form = useForm<Webhook>({
    mode: "all",
    resolver: zodResolver(schema),
    defaultValues: webhook,
  });
  useEffect(() => form.reset(webhook), [webhook, form]);

  const handleSubmit = async (data: Webhook) => {
    try {
      await (webhook ? updateWebhook(data) : addWebhook(data));
      await onSaveCompleted();
    } catch (err) {
      console.error(err);
    }
  };

  const handleTestClick = async (data: Webhook) => {
    try {
      const { data: resp } = await testWebhook(data);
      toast.message("Test Result", {
        description: resp,
        duration: 5000,
      });
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px] p-6 rounded-lg shadow-xl bg-white">
        <DialogHeader>
          <DialogTitle className="text-2xl font-bold text-gray-800">
            {webhook ? "编辑 Webhook" : "添加 Webhook"}
          </DialogTitle>
          <DialogDescription className="text-gray-600 mt-1">
            {webhook
              ? "修改你的 Webhook 配置。"
              : "创建一个新的 Webhook 配置。"}
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <FormField
            control={form.control}
            name="name"
            render={({ field }) => {
              return (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              );
            }}
          />
          <FormField
            control={form.control}
            name="value"
            render={({ field }) => {
              return (
                <FormItem className="mt-6">
                  <FormLabel>Webhook 内容</FormLabel>
                  <FormControl>
                    <Textarea
                      className="min-h-64"
                      placeholder={placeholder}
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              );
            }}
          />
          <DialogFooter className="flex flex-col sm:flex-row justify-end gap-2 mt-4">
            <DialogClose asChild>
              <Button variant="outline">取消</Button>
            </DialogClose>
            <Button
              variant="default"
              className="bg-green-600"
              onClick={form.handleSubmit(handleTestClick)}
            >
              测试
            </Button>
            <Button onClick={form.handleSubmit(handleSubmit)}>保存</Button>
          </DialogFooter>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
