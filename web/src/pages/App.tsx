import { LogOutIcon, TriangleAlertIcon } from "lucide-react";
import { Link, Route, Switch, useLocation } from "wouter";
import { Home } from "@/pages/Home.tsx";
import Config from "@/pages/Config.tsx";
import { Toaster } from "@/components/ui/sonner.tsx";
import { Login } from "@/pages/Login.tsx";
import Provider from "@/pages/Provider.tsx";
import Webhook from "@/pages/Webhook.tsx";
import { Button } from "@/components/ui/button.tsx";
import { navigate } from "wouter/use-browser-location";
import { useEffect, useState } from "react";
import { token } from "@/lib/storage";
import { Initialize } from "@/pages/Initialize.tsx";
import { type SysInfo, system } from "@/lib/api.ts";
import { toast } from "@/lib/toast.ts";
import Log from "@/pages/Log.tsx";

export default function App() {
  const [location, setLocation] = useLocation();
  const [sysInfo, setSysInfo] = useState<SysInfo>({
    version: "0.0.0",
    configPath: "-",
    initialized: true,
    commitId: "-",
  });
  useEffect(() => {
    const checking = async () => {
      const info = await system();
      setSysInfo(info);
      //未初始化
      if (!info.initialized && location !== "/initialize") {
        navigate("/initialize");
        return;
      }
      if (!info.initialized && location === "/initialize") {
        return;
      }
      if (info.initialized && location === "/initialize") {
        toast.success("System initialized!");
        navigate("/");
        return;
      }
      if (location !== "/login" && !token.exists()) {
        navigate("/login");
        return;
      }
    };
    checking().catch(console.error);
  }, [location, setLocation]);
  return (
    <div className="flex flex-col w-screen h-screen">
      <Toaster position="top-center" />
      <Header info={sysInfo} />
      <div className="flex-1 bg-gray-50 overflow-hidden">
        <Switch>
          <Route path="/" component={Home} />
          <Route path="/login" component={Login} />
          <Route path="/config/:id?" component={Config} />
          <Route path="/provider" component={Provider} />
          <Route path="/webhook" component={Webhook} />
          <Route path="/initialize" component={Initialize} />
          <Route path="/log" component={Log} />
          <Route path="*">{(params) => <NotFound path={params["*"]} />}</Route>
        </Switch>
      </div>
    </div>
  );
}

function NotFound({ path }: { path: string }) {
  return (
    <div className="size-full flex flex-col items-center justify-center bg-gray-50 font-inter text-gray-800 p-4">
      <div className="text-center bg-white p-8 rounded-lg shadow-lg max-w-md w-full">
        <TriangleAlertIcon className="size-20 text-yellow-500 mx-auto mb-6" />
        <h1 className="text-4xl font-bold mb-4">404</h1>
        <p className="text-xl mb-6">抱歉，页面未找到！</p>
        <p className="text-md text-gray-600 mb-8 break-words">
          您尝试访问的页面
          <span className="font-semibold text-red-500">/{path}</span> 不存在。
        </p>
        <Button onClick={() => navigate("/")} className="px-6 py-3 text-lg">
          返回主页
        </Button>
      </div>
    </div>
  );
}

function Header({ info }: { info: SysInfo }) {
  return (
    <div className="shrink-0 items-center w-full bg-gray-700 h-15">
      <header className="flex items-center h-full">
        <div className="w-5/6 flex flex-row pl-64 justify-between">
          <Link
            href="/"
            className="text-xl text-blue-300 hover:text-blue-200 transition-colors duration-200 font-bold rounded-md p-1"
          >
            DDNS-RS
          </Link>

          <div className="flex items-center gap-6">
            {token.exists() && (
              <nav className="flex flex-row gap-6">
                <Link
                  to="/provider"
                  className="text-sm text-gray-200 hover:text-blue-300 transition-colors duration-200 font-medium rounded-md p-1"
                >
                  Provider
                </Link>
                <Link
                  to="/webhook"
                  className="text-sm text-gray-200 hover:text-blue-300 transition-colors duration-200 font-medium rounded-md p-1"
                >
                  Webhook
                </Link>
                <Link
                  to="/log"
                  className="text-sm text-gray-200 hover:text-blue-300 transition-colors duration-200 font-medium rounded-md p-1"
                >
                  Log
                </Link>
                <div className="ml-6 p-1 rounded-md">
                  <Link to="/login" onClick={() => token.remove()}>
                    <LogOutIcon className="text-white hover:text-red-400 size-5 transition-colors duration-200" />
                  </Link>
                </div>
              </nav>
            )}
          </div>
        </div>

        <div className="flex flex-row items-center">
          <div className="h-6 w-px bg-gray-600 mx-2" />
          <div className="flex flex-col items-end text-xs text-gray-400 ml-4 max-w-96 overflow-hidden truncate">
            <span className="font-mono">
              v{info.version} | {info.commitId}
            </span>
          </div>
        </div>
      </header>
    </div>
  );
}
