import { useForm, useFormContext } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";

import {
  Card,
  CardHeader,
  CardDescription,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormControl,
  FormDescription,
  FormMessage,
} from "@/components/ui/form";
import { ScrollArea } from "@/components/ui/scroll-area.tsx";
import { useRoute } from "wouter";
import {
  addDdnsConfig,
  getDnsConfig,
  getNicList,
  getProviders,
  getWebhooks,
  updateDdnsConfig,
} from "@/lib/api.ts";
import { toast } from "@/lib/toast";
import { PlusIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { DnsProviderDialog } from "@/pages/Provider.tsx";
import { EditingWebhookDialog, type Webhook } from "@/pages/Webhook.tsx";
import {
  AcquisitionMethodSelect,
  NetCardSelect,
} from "@/components/IpAcquisition.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import {
  type DdnsConfig,
  type NetCard,
  networkConfigSchema,
} from "@/types/config.ts";
import type { ProviderConfig } from "@/types/provider.ts";
import { navigate } from "wouter/use-browser-location";

const IPv4ConfigCard = () => {
  const { watch, control, unregister } = useFormContext<DdnsConfig>();

  const [netCards, setNetCards] = useState<NetCard[]>([]);
  useEffect(() => {
    getNicList("ipv4").then(setNetCards);
  }, []);

  const ipv4Enabled = watch("ipv4.enabled");
  const selectedAcquisitionMethod = watch("ipv4.method");
  const selectedNetCardName = watch("ipv4.interface");
  const selectedNetCard = netCards.find(
    ({ name }) => name === selectedNetCardName,
  );
  useEffect(() => {
    if (!ipv4Enabled) {
      unregister(["ipv4.endpoint", "ipv4.interface", "ipv4.command"]);
    }
  }, [ipv4Enabled, unregister]);

  return (
    <Card
      className={`w-full border-none shadow-none pt-0 ${ipv4Enabled ? "" : "opacity-50"}`}
    >
      <CardHeader className="p-0 pb-3 flex flex-row items-center justify-between">
        <div>
          <h3 className="text-xl font-semibold text-gray-900 ">
            IPv4 Configuration
          </h3>
          <CardDescription className="text-sm text-gray-600 mt-1">
            Enable or disable IPv4 and select its address acquisition method.
          </CardDescription>
        </div>
        <div className="flex items-center space-x-2">
          <FormField
            control={control}
            name="ipv4.enabled"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="sr-only">Enable IPv4</FormLabel>
                <FormControl>
                  <Switch
                    checked={field.value}
                    onCheckedChange={field.onChange}
                  />
                </FormControl>
              </FormItem>
            )}
          />
        </div>
      </CardHeader>

      <CardContent className="p-0 pt-0">
        <div className="flex flex-col space-y-4">
          <FormField
            control={control}
            name="ipv4.method"
            render={({ field }) => (
              <FormItem>
                <FormLabel>IPv4 Acquisition Method</FormLabel>
                <AcquisitionMethodSelect
                  defaultValue={field.value}
                  value={field.value}
                  onValueChange={field.onChange}
                  disabled={!ipv4Enabled}
                />
                <FormMessage />
              </FormItem>
            )}
          />

          {selectedAcquisitionMethod === "api" && ipv4Enabled && (
            <FormField
              control={control}
              name="ipv4.endpoint"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>API Endpoint URL</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="e.g., https://api.example.com/ipv4"
                      {...field}
                      disabled={!ipv4Enabled}
                    />
                  </FormControl>
                  <FormDescription>
                    IPv4 address will be acquired via the configured API
                    endpoint.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
          {selectedAcquisitionMethod === "nic" && ipv4Enabled && (
            <FormField
              control={control}
              name="ipv4.interface"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Select Network Interface</FormLabel>
                  <NetCardSelect
                    netCards={netCards}
                    selectedNetCard={selectedNetCard}
                    defaultValue={field.value}
                    value={field.value}
                    onValueChange={field.onChange}
                    disabled={!ipv4Enabled}
                  />
                  <FormDescription>
                    IPv4 address will be acquired from the specified network
                    interface.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
          {selectedAcquisitionMethod === "cmd" && ipv4Enabled && (
            <FormField
              control={control}
              name="ipv4.command"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Command Line Commands</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="e.g., ip addr add 192.168.1.100/24 dev eth0"
                      {...field}
                      disabled={!ipv4Enabled}
                      rows={5}
                    />
                  </FormControl>
                  <FormDescription>
                    This setting indicates that IPv4 configuration will be
                    managed via external command-line tools.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
        </div>
      </CardContent>
    </Card>
  );
};

const IPv6ConfigCard = () => {
  const { watch, control, unregister } = useFormContext<DdnsConfig>();

  const [netCards, setNetCards] = useState<NetCard[]>([]);
  useEffect(() => {
    getNicList("ipv6").then(setNetCards);
  }, []);

  const ipv6Enabled = watch("ipv6.enabled");
  const selectedAcquisitionMethod = watch("ipv6.method");
  const selectedNetCardName = watch("ipv6.interface");
  const selectedNetCard = netCards.find(
    ({ name }) => name === selectedNetCardName,
  );
  useEffect(() => {
    if (!ipv6Enabled) {
      unregister(["ipv6.endpoint", "ipv6.interface", "ipv6.command"]);
    }
  }, [ipv6Enabled, unregister]);
  return (
    // <Card className="w-full rounded-lg shadow-sm border border-gray-200 p-4">
    <Card
      className={`w-full border-none shadow-none pt-0 ${ipv6Enabled ? "" : "opacity-50"}`}
    >
      <CardHeader className="p-0 pb-3 flex flex-row items-center justify-between">
        <div>
          <h3 className="text-xl font-semibold text-gray-900">
            IPv6 Configuration
          </h3>
          <CardDescription className="text-sm text-gray-600 mt-1">
            Enable or disable IPv6 and select its address acquisition method.
          </CardDescription>
        </div>
        <div className="flex items-center space-x-2">
          <FormField
            control={control}
            name="ipv6.enabled"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Switch
                    checked={field.value}
                    onCheckedChange={field.onChange}
                  />
                </FormControl>
              </FormItem>
            )}
          />
        </div>
      </CardHeader>

      <CardContent className="p-0 pt-0">
        <div className="flex flex-col space-y-4">
          <FormField
            control={control}
            name="ipv6.method"
            render={({ field }) => (
              <FormItem>
                <FormLabel>IPv6 Acquisition Method</FormLabel>
                <AcquisitionMethodSelect
                  onValueChange={field.onChange}
                  value={field.value}
                  defaultValue={field.value}
                  disabled={!ipv6Enabled}
                />
                <FormMessage />
              </FormItem>
            )}
          />

          {selectedAcquisitionMethod === "api" && ipv6Enabled && (
            <FormField
              control={control}
              name="ipv6.endpoint"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>API Endpoint URL</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="e.g., https://api.example.com/ipv6"
                      {...field}
                      disabled={!ipv6Enabled}
                    />
                  </FormControl>
                  <FormDescription>
                    IPv6 address will be acquired via the configured API
                    endpoint.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
          {selectedAcquisitionMethod === "nic" && ipv6Enabled && (
            <FormField
              control={control}
              name="ipv6.interface"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Select Network Interface</FormLabel>
                  <NetCardSelect
                    netCards={netCards}
                    selectedNetCard={selectedNetCard}
                    defaultValue={field.value}
                    value={field.value}
                    onValueChange={field.onChange}
                    disabled={!ipv6Enabled}
                  />
                  <FormDescription>
                    IPv6 address will be acquired from the specified network
                    interface.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
          {selectedAcquisitionMethod === "cmd" && ipv6Enabled && (
            <FormField
              control={control}
              name="ipv6.command"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Command Line Commands</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="e.g., ip -6 addr add 2001:db8::1/64 dev eth0"
                      {...field}
                      disabled={!ipv6Enabled}
                      rows={5}
                    />
                  </FormControl>
                  <FormDescription>
                    This setting indicates that IPv6 configuration will be
                    managed via external command-line tools.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          )}
        </div>
      </CardContent>
    </Card>
  );
};

const Editor = () => {
  const [, params] = useRoute<{ id?: string }>("/config/:id?");

  const [providers, setProviders] = useState<ProviderConfig[]>([]);

  const [webhooks, setWebhooks] = useState<Webhook[]>([]);

  const reload = async () => {
    return Promise.all([
      getProviders().then(setProviders),
      getWebhooks().then(setWebhooks),
    ]).then(() => console.log("Reload provider and webhooks complete"));
  };

  useEffect(() => {
    reload().then(console.log);
  }, []);

  // Initialize react-hook-form
  const form = useForm<DdnsConfig>({
    mode: "all",
    resolver: zodResolver(networkConfigSchema),
    defaultValues: async () => {
      if (params?.id) {
        try {
          const config = await getDnsConfig(params.id);
          if (config) {
            return config;
          }
          navigate(`/404?config=${params.id}`);
        } catch (err) {
          console.error(err);
        }
      }
      return {
        name: "",
        interval: 30,
        domain: "",
        subdomain: "",
        ipv4: {
          enabled: false,
          method: "api",
          endpoint: "",
        },
        ipv6: {
          enabled: false,
          method: "api",
          endpoint: "",
        },
        provider: "",
      };
    },
  });

  const { handleSubmit, control } = form;

  const onSubmit = async (data: DdnsConfig) => {
    try {
      if (params?.id) {
        await updateDdnsConfig(data);
      } else {
        await addDdnsConfig(data);
      }
      toast.success("Save Config succeed");
      navigate("/");
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex justify-center items-center bg-gray-100 pb-40">
      <Card className="w-full mt-4 max-w-2xl rounded-lg shadow-lg">
        <CardHeader className="p-6 pb-4 text-center">
          <h1 className="text-3xl font-bold text-gray-900">
            Network Configuration
          </h1>
          <CardDescription className="text-base text-gray-700 mt-2">
            Configure both IPv4 and IPv6 network settings for your device.
          </CardDescription>
        </CardHeader>
        <Form {...form}>
          <CardContent className="space-y-8">
            <FormField
              control={control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Configuration Name</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="e.g., Home Network Profile"
                      readOnly={params?.id != null}
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    Give this set of network configurations a unique name.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex flex-row items-start">
              <FormField
                control={control}
                name="subdomain"
                render={({ field }) => (
                  <FormItem className="w-1/4">
                    <FormLabel>Sub Domain</FormLabel>
                    <FormControl>
                      <Input
                        className="rounded-r-none"
                        placeholder="e.g., @、www、api"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>Enter the subdomain</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={control}
                name="domain"
                render={({ field }) => (
                  <FormItem className="grow">
                    <FormLabel>Primary Domain</FormLabel>
                    <FormControl>
                      <Input
                        className="rounded-l-none"
                        placeholder="e.g., example.com"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>Enter the primary domain</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>
            <FormField
              control={control}
              name="interval"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Interval</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      {...field}
                      onChange={(e) => field.onChange(Number(e.target.value))}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </CardContent>
          <Separator className="mt-2" />
          <CardContent>
            <IPv4ConfigCard />
          </CardContent>
          <Separator className="mt-2" />
          <CardContent>
            <IPv6ConfigCard />
          </CardContent>
          <Separator className="mt-2" />

          <CardContent className="space-y-8">
            <FormField
              control={control}
              name="provider"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>DNS Provider</FormLabel>
                  <div className="flex items-center space-x-2">
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                      value={field.value}
                    >
                      <SelectTrigger className="w-full">
                        <SelectValue placeholder="Select a DNS provider" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Select Provider</SelectLabel>
                          {providers.map((option) => (
                            <SelectItem
                              key={`provider-${option.name}`}
                              value={option.name}
                            >
                              {option.name}
                            </SelectItem>
                          ))}
                        </SelectGroup>
                      </SelectContent>
                    </Select>
                    <AddDnsProviderButton
                      onSaveCompleted={async () => reload()}
                    />
                  </div>
                  <FormDescription>
                    Choose your Dynamic DNS service provider.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            {/* Webhook Select with Button */}
            <FormField
              control={control}
              name="webhook"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Webhook</FormLabel>
                  <div className="flex items-center space-x-2">
                    <Select
                      onValueChange={field.onChange}
                      defaultValue={field.value}
                      value={field.value}
                    >
                      <FormControl>
                        <SelectTrigger className="w-full">
                          <SelectValue placeholder="Select a webhook option" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        <SelectGroup>
                          <SelectLabel>Select Webhook</SelectLabel>
                          {webhooks.map((option) => (
                            <SelectItem
                              key={`webhook-${option.name}`}
                              value={option.name}
                            >
                              {option.name}
                            </SelectItem>
                          ))}
                        </SelectGroup>
                        <SelectSeparator />
                        <Button
                          className="w-full px-2"
                          variant="secondary"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation();
                            field.onChange("");
                          }}
                        >
                          Clear
                        </Button>
                      </SelectContent>
                    </Select>
                    <AddWebhookButton onSaveCompleted={async () => reload()} />
                  </div>
                  <FormDescription>
                    Configure a webhook for notifications or custom actions.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </CardContent>
        </Form>

        {/* Global Save Button in CardFooter */}
        <CardFooter className="p-6 pt-0 flex justify-end">
          <Button onClick={handleSubmit(onSubmit)}>
            {"Save All Settings"}
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
};

const AddDnsProviderButton = ({
  onSaveCompleted,
}: {
  onSaveCompleted(): Promise<void>;
}) => {
  const [open, setOpen] = useState(false);

  return (
    <>
      <DnsProviderDialog
        open={open}
        onOpenChange={setOpen}
        onSaveCompleted={async () => {
          setOpen(false);
          await onSaveCompleted();
        }}
      />
      <Button
        variant="outline"
        size="icon"
        onClick={() => setOpen(true)}
        className="flex-shrink-0"
      >
        <PlusIcon />
      </Button>
    </>
  );
};

const AddWebhookButton = ({
  onSaveCompleted,
}: {
  onSaveCompleted(): Promise<void>;
}) => {
  const [open, setOpen] = useState(false);

  return (
    <>
      <EditingWebhookDialog
        open={open}
        onOpenChange={setOpen}
        onSaveCompleted={async () => {
          setOpen(false);
          await onSaveCompleted();
        }}
      />
      <Button
        variant="outline"
        size="icon"
        onClick={() => setOpen(true)}
        className="flex-shrink-0"
      >
        <PlusIcon />
      </Button>
    </>
  );
};

export default function Config() {
  return (
    <div className="size-full overflow-hidden">
      <ScrollArea className="size-full overflow-hidden">
        <Editor />
      </ScrollArea>
    </div>
  );
}
