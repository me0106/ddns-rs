import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
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
import { login } from "@/lib/api.ts";
import { toast } from "@/lib/toast";
import { token } from "@/lib/storage.ts";
import { navigate } from "wouter/use-browser-location";

const user = z.object({
  username: z.string().nonempty({ error: "Username is required" }),
  password: z.string().nonempty({ error: "Password is required" }),
});
type User = z.infer<typeof user>;

export function Login() {
  const form = useForm({
    mode: "all",
    resolver: zodResolver(user),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const onClick = async (value: User) => {
    try {
      const res = await login(value);
      token.set(res.token);
      toast.success("Login successfully!");
      navigate("/");
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="w-full h-full">
      <form>
        <Card className="mx-auto w-96 pt-10 mt-40">
          <CardHeader>
            <CardTitle>Login to your account</CardTitle>
          </CardHeader>
          <CardContent>
            <Form {...form}>
              <div className="flex flex-col">
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
                    <FormItem className="mt-8">
                      <div className="flex items-center">
                        <FormLabel>Password</FormLabel>
                        <a
                          href="#"
                          className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                        >
                          Forgot your password?
                        </a>
                      </div>
                      <Input {...field} />
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
            </Form>
          </CardContent>
          <CardFooter className="flex-col my-6">
            <Button
              className="w-full"
              type="submit"
              onClick={form.handleSubmit(onClick)}
            >
              Login
            </Button>
          </CardFooter>
        </Card>
      </form>
    </div>
  );
}
