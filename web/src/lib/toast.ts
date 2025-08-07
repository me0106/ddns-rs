import { type ExternalToast, toast } from "sonner";
import React from "react";

type Title = (() => React.ReactNode) | React.ReactNode;

const DEFAULT: ExternalToast = {
  // id: "default",
  duration: 1500,
  closeButton: false,
  action: {
    label: "Dismiss",
    onClick: () => {},
  },
};
const myToast = {
  message(message: Title, data?: ExternalToast): string | number {
    return toast.message(message, data ?? DEFAULT);
  },
  error(message: Title, data?: ExternalToast): string | number {
    return toast.error(message, data ?? DEFAULT);
  },
  success(message: Title, data?: ExternalToast): string | number {
    return toast.success(message, data ?? DEFAULT);
  },
};

export { myToast as toast };
