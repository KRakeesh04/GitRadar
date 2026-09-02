import { Loader2, Minimize2 } from "lucide-react";
import { Button } from "./ui/button";
import { Card } from "./ui/card";
import { Progress } from "./ui/progress";

interface SyncToastProps {
  progress: number;
  current: number;
  total: number;
  minimized: boolean;
  onToggle(): void;
}

export function SyncToast(props: SyncToastProps) {
  if (props.minimized) {
    return (
      <Button onClick={props.onToggle}>
        <Loader2 className="animate-spin" />
      </Button>
    );
  }

  return (
    <Card>
      <div className="flex justify-between">
        <span>
          Syncing repositories
        </span>
        <Button
          size="icon"
          onClick={props.onToggle}
        >
          <Minimize2 />
        </Button>
      </div>
      <Progress value={props.progress} />
      <p>
        {props.current}/{props.total}
      </p>
    </Card>
  );
}
