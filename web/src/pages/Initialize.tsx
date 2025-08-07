import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { z } from "zod/v4";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form.tsx";
import { initialize } from "@/lib/api.ts";
import { navigate } from "wouter/use-browser-location";
import { toast } from "@/lib/toast";

const user = z
  .object({
    username: z.string().nonempty({ error: "Username is required" }),
    password: z.string().nonempty({ error: "Password is required" }),
    confirmPassword: z
      .string()
      .nonempty({ error: "Confirm password is required" }),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ["confirmPassword"],
  });
type User = z.infer<typeof user>;

export function Initialize() {
  const form = useForm({
    mode: "all",
    resolver: zodResolver(user),
    defaultValues: {
      username: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onClick = async (user: User) => {
    try {
      await initialize({ user: user });
      toast.success("init completed.");
      navigate("/");
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <Card className="mx-auto w-96 pt-10 mt-20 transition-all duration-1000 ease-in-out">
      <CardHeader>
        <CardTitle>Initialize System </CardTitle>
        <CardDescription>
          Enter your username below to login to your account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <div className="flex flex-col space-y-6">
            <FormField
              control={form.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username</FormLabel>
                  <Input {...field} autoFocus />
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <div className="flex items-center">
                    <FormLabel>Password</FormLabel>
                  </div>
                  <Input {...field} />
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="confirmPassword"
              render={({ field }) => (
                <FormItem>
                  <div className="flex items-center">
                    <FormLabel>Repeat Password</FormLabel>
                  </div>
                  <Input {...field} />
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </Form>
      </CardContent>
      <CardFooter className="mt-4 mb-4">
        <Button className="w-full" onClick={form.handleSubmit(onClick)}>
          Init
        </Button>
      </CardFooter>
    </Card>
  );
}
