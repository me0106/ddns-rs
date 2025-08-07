import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { Badge } from "@/components/ui/badge.tsx";
import * as React from "react";
import * as SelectPrimitive from "@radix-ui/react-select";

export function AcquisitionMethodSelect({
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Root>) {
  return (
    <Select {...props}>
      <SelectTrigger className="w-full">
        <SelectValue placeholder="Select acquisition method" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="api">Get from API</SelectItem>
        <SelectItem value="nic">Specify Network Interface</SelectItem>
        <SelectItem value="cmd">Command Line Tool Configuration</SelectItem>
      </SelectContent>
    </Select>
  );
}

export function NetCardSelect({
  netCards,
  selectedNetCard,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Root> & {
  netCards: { name: string; addrs: string[] }[];
  selectedNetCard?: { name: string; addrs: string[] };
}) {
  return (
    <Select {...props}>
      <SelectTrigger className="w-full overflow-hidden">
        <SelectValue
          placeholder="Select a network interface"
          asChild={selectedNetCard != undefined}
        >
          {selectedNetCard != undefined && (
            <div className="flex">
              <span className="text-sm font-medium  text-gray-900">
                {selectedNetCard.name}
              </span>
              <div className="flex gap-1">
                {selectedNetCard.addrs.slice(0, 2).map((ip, index) => (
                  <Badge
                    className="text-gray-500"
                    variant="secondary"
                    key={index}
                  >
                    {ip}
                  </Badge>
                ))}
              </div>
            </div>
          )}
        </SelectValue>
      </SelectTrigger>
      <SelectContent>
        {netCards.map(({ name, addrs }) => (
          <SelectItem key={`nic-${name}`} value={name}>
            <div className="flex flex-col">
              <span className="text-sm font-medium text-gray-900">{name}</span>
              <div className="flex flex-wrap gap-1 mt-1">
                {addrs.map((ip, index) => (
                  <Badge
                    className="text-gray-500"
                    variant="secondary"
                    key={index}
                  >
                    {ip}
                  </Badge>
                ))}
              </div>
            </div>
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
