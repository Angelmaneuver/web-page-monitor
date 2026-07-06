import { RotateCcw, X } from 'lucide-react';
import type { JSX } from 'react';

import { Button } from '@/components/ui/button';
import { close } from '@/lib/tauri/window';

function Window({
  label,
  reload,
  ...props
}: Exclude<JSX.IntrinsicElements['main'], 'children'> & {
  label: string;
  reload: () => Promise<void>;
}) {
  return (
    <main {...props}>
      <section className="system">
        <section className="bar" data-tauri-drag-region draggable="true">
          {label}
        </section>

        <Button className="button" onClick={reload}>
          <RotateCcw />
        </Button>

        <Button className="button" onClick={close}>
          <X />
        </Button>
      </section>

      <section className="monitor" />
    </main>
  );
}

export default Window;
