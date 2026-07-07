import { RotateCcw, X } from 'lucide-react';
import type { JSX } from 'react';
import { useState } from 'react';

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
  const [isHovered, setIsHovered] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [isBeforeLeave, setIsBeforeLeave] = useState(false);

  return (
    <main {...props}>
      <section
        className="system"
        onMouseEnter={() => {
          setIsHovered(true);

          if (isBeforeLeave) {
            setIsBeforeLeave(false);
          }

          if (isDragging) {
            setIsDragging(false);
          }
        }}
        onMouseLeave={() => {
          if (!isDragging || isBeforeLeave) {
            setIsHovered(false);
          }

          setIsBeforeLeave(true);
        }}
      >
        <section
          className={`bar${isHovered || isDragging ? ' visible' : ''}`}
          data-tauri-drag-region
          onPointerDown={() => {
            setIsDragging(true);
          }}
        >
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
